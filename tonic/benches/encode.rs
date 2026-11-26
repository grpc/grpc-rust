/*
 *
 * Copyright 2026 gRPC authors.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to
 * deal in the Software without restriction, including without limitation the
 * rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
 * sell copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 *
 */

#![allow(missing_docs)]

use bencher::{Bencher, benchmark_group, benchmark_main};
use bytes::{BufMut, Bytes};
use http_body_util::BodyExt;
use std::pin::pin;
use tonic::{
    Status,
    codec::{BufferSettings, EncodeBody, EncodeBuf, Encoder, SingleMessageCompressionOverride},
};

macro_rules! bench {
    ($name:ident, $payload_size:expr, $message_count:expr, $zero_copy_threshold:expr) => {
        fn $name(b: &mut Bencher) {
            let rt = tokio::runtime::Builder::new_current_thread()
                .build()
                .expect("runtime");

            let payload = Bytes::from(vec![97u8; $payload_size]);
            let mut buffer_settings = BufferSettings::default();
            if let Some(threshold) = $zero_copy_threshold {
                buffer_settings = buffer_settings.with_zero_copy_threshold(threshold);
            }
            b.bytes = ($payload_size * $message_count) as u64;

            b.iter(|| {
                rt.block_on(async {
                    let messages = std::iter::repeat_n(Ok(payload.clone()), $message_count);
                    let mut body = pin!(EncodeBody::new_server(
                        MockEncoder { buffer_settings },
                        tokio_stream::iter(messages),
                        None,
                        SingleMessageCompressionOverride::default(),
                        None,
                    ));

                    let mut len = 0;
                    while let Some(frame) = body.frame().await {
                        if let Ok(data) = frame.unwrap().into_data() {
                            len += data.len();
                        }
                    }
                    assert!(len > $payload_size * $message_count);
                })
            })
        }
    };
}

#[derive(Debug, Clone)]
struct MockEncoder {
    buffer_settings: BufferSettings,
}

impl Encoder for MockEncoder {
    type Item = Bytes;
    type Error = Status;

    fn encode(&mut self, item: Self::Item, dst: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        dst.put_slice(b"prefix");
        dst.put(item);
        dst.put_slice(b"suffix");
        Ok(())
    }

    fn buffer_settings(&self) -> BufferSettings {
        self.buffer_settings
    }
}

bench!(copy_2mib, 2 * 1024 * 1024, 8, None);
bench!(zero_copy_2mib, 2 * 1024 * 1024, 8, Some(64 * 1024));
bench!(copy_1kib, 1024, 1000, None);
bench!(zero_copy_1kib, 1024, 1000, Some(64 * 1024));

benchmark_group!(large_payload, copy_2mib, zero_copy_2mib);

benchmark_group!(small_payload, copy_1kib, zero_copy_1kib);

benchmark_main!(large_payload, small_payload);
