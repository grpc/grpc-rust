/*
 *
 * Copyright 2025 gRPC authors.
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

//! This module defines a common encoder with small buffers. This is useful
//! when you have many concurrent RPC's, and not a huge volume of data per
//! rpc normally.
//!
//! Note that you can customize your codecs per call to the code generator's
//! compile function. This lets you group services by their codec needs.
//!
//! While this codec demonstrates customizing the built-in Prost codec, you
//! can use this to implement other codecs as well, as long as they have a
//! Default implementation.

use std::marker::PhantomData;

use prost::Message;
use tonic::codec::{BufferSettings, Codec};
use tonic_prost::ProstCodec;

#[derive(Debug, Clone, Copy, Default)]
pub struct SmallBufferCodec<T, U>(PhantomData<(T, U)>);

impl<T, U> Codec for SmallBufferCodec<T, U>
where
    T: Message + Send + 'static,
    U: Message + Default + Send + 'static,
{
    type Encode = T;
    type Decode = U;

    type Encoder = <ProstCodec<T, U> as Codec>::Encoder;
    type Decoder = <ProstCodec<T, U> as Codec>::Decoder;

    fn encoder(&mut self) -> Self::Encoder {
        // Here, we will just customize the prost codec's internal buffer settings.
        // You can of course implement a complete Codec, Encoder, and Decoder if
        // you wish!
        ProstCodec::<T, U>::raw_encoder(BufferSettings::new(512, 4096))
    }

    fn decoder(&mut self) -> Self::Decoder {
        ProstCodec::<T, U>::raw_decoder(BufferSettings::new(512, 4096))
    }
}
