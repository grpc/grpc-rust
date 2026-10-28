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

//! A client that drops response streams before their end resets them, but the
//! server can still have DATA in flight for them. h2 ignores DATA for a reset
//! stream that it remembers. DATA for a reset stream that it has forgotten makes
//! it reset the stream again and count that reset, and too many of those close
//! the connection with `GOAWAY(ENHANCE_YOUR_CALM, "too_many_internal_resets")`.

use std::io::ErrorKind;
use std::sync::Arc;
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tonic::body::Body;
use tonic::transport::Endpoint;
use tower::ServiceExt;

const DATA: u8 = 0;
const HEADERS: u8 = 1;
const RST_STREAM: u8 = 3;
const SETTINGS: u8 = 4;
const PING: u8 = 6;
const GOAWAY: u8 = 7;
const END_HEADERS: u8 = 0x4;
const ACK: u8 = 0x1;
/// More than the 50 reset streams that h2 remembers by default.
const STREAMS: u32 = 100;
/// Late DATA frames for each reset stream. With the h2 defaults, the forgotten
/// streams get more than 1024 of them in total.
const LATE_FRAMES: usize = 40;

type Writer = Arc<Mutex<OwnedWriteHalf>>;

/// How the connection ended, as the server saw it.
#[derive(Debug, PartialEq)]
enum End {
    /// The connection stayed open.
    Open,
    /// The client sent `GOAWAY` with this debug data.
    GoAway(String),
    /// The connection was reset before the server read a `GOAWAY`. A client
    /// that closes the socket with unread late DATA resets the connection, and
    /// Windows then discards the `GOAWAY` that the server did not read yet.
    Reset,
}

/// Writes one frame. It returns `false` if the client closed the connection.
async fn write_frame(io: &Writer, kind: u8, flags: u8, stream: u32, payload: &[u8]) -> bool {
    let length = u32::try_from(payload.len()).unwrap().to_be_bytes();
    let mut frame = vec![length[1], length[2], length[3], kind, flags];
    frame.extend_from_slice(&stream.to_be_bytes());
    frame.extend_from_slice(payload);
    io.lock().await.write_all(&frame).await.is_ok()
}

/// An HTTP/2 server that answers each request with headers only. After the
/// client resets `STREAMS` streams, it sends `LATE_FRAMES` DATA frames on each
/// of them. A separate task writes the late DATA, so that the server keeps
/// reading while it writes.
async fn late_data_server(listener: TcpListener) -> End {
    let (io, _) = listener.accept().await.unwrap();
    let (mut reader, writer) = io.into_split();
    let writer: Writer = Arc::new(Mutex::new(writer));
    let mut preface = [0_u8; 24];
    reader.read_exact(&mut preface).await.unwrap();
    write_frame(&writer, SETTINGS, 0, 0, &[]).await;
    let mut reset = Vec::new();
    loop {
        let mut header = [0_u8; 9];
        let read =
            tokio::time::timeout(Duration::from_secs(2), reader.read_exact(&mut header)).await;
        // Only a quiet connection means that it stayed open.
        let Ok(read) = read else { return End::Open };
        if let Err(error) = read {
            match error.kind() {
                ErrorKind::ConnectionReset | ErrorKind::ConnectionAborted => return End::Reset,
                _ => panic!("the client closed the connection without GOAWAY: {error}"),
            }
        }
        let length =
            usize::from(header[0]) << 16 | usize::from(header[1]) << 8 | usize::from(header[2]);
        let stream = u32::from_be_bytes([header[5], header[6], header[7], header[8]]) & 0x7fff_ffff;
        let mut payload = vec![0_u8; length];
        reader.read_exact(&mut payload).await.unwrap();
        match header[3] {
            SETTINGS if header[4] & ACK == 0 => {
                write_frame(&writer, SETTINGS, ACK, 0, &[]).await;
            }
            PING if header[4] & ACK == 0 => {
                write_frame(&writer, PING, ACK, 0, &payload).await;
            }
            // `0x88` is `:status: 200` from the HPACK static table.
            HEADERS => {
                write_frame(&writer, HEADERS, END_HEADERS, stream, &[0x88]).await;
            }
            RST_STREAM => {
                reset.push(stream);
                if reset.len() == STREAMS as usize {
                    let writer = writer.clone();
                    let reset = reset.clone();
                    tokio::spawn(async move {
                        // Frames above 256 bytes, so that h2 does not count them as small frames.
                        let late = [0_u8; 1024];
                        for _ in 0..LATE_FRAMES {
                            for &stream in &reset {
                                if !write_frame(&writer, DATA, 0, stream, &late).await {
                                    return;
                                }
                            }
                        }
                    });
                }
            }
            GOAWAY => return End::GoAway(String::from_utf8_lossy(&payload[8..]).into_owned()),
            _ => {}
        }
    }
}

/// Resets `STREAMS` streams, then lets the server send late DATA on all of them.
async fn late_data_end(configure: impl FnOnce(Endpoint) -> Endpoint) -> End {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(late_data_server(listener));
    let channel = configure(Endpoint::from_shared(format!("http://{addr}")).unwrap())
        .connect()
        .await
        .unwrap();
    for _ in 0..STREAMS {
        let request = http::Request::builder()
            .uri("/test.Test/UnaryCall")
            .body(Body::empty())
            .unwrap();
        // The response body is dropped before its end, which resets the stream.
        let response = channel.clone().oneshot(request).await.unwrap();
        drop(response);
    }
    server.await.unwrap()
}

#[tokio::test]
async fn late_data_on_remembered_reset_streams_keeps_the_connection() {
    let end = late_data_end(|endpoint| {
        endpoint
            .http2_max_concurrent_reset_streams(10_000)
            .http2_reset_stream_duration(Duration::from_secs(10))
    })
    .await;
    assert_eq!(end, End::Open);
}

/// With the defaults, the same late DATA closes the connection. This shows that
/// the test above makes the failure that the settings prevent.
#[tokio::test]
async fn late_data_on_forgotten_reset_streams_closes_the_connection() {
    let end = late_data_end(|endpoint| endpoint).await;
    assert!(
        matches!(&end, End::GoAway(debug) if debug == "too_many_internal_resets")
            || (cfg!(windows) && end == End::Reset),
        "{end:?}"
    );
}

/// With the limit disabled, the same late DATA does not close the connection.
#[tokio::test]
async fn late_data_without_the_local_error_reset_limit_keeps_the_connection() {
    let end = late_data_end(|endpoint| endpoint.http2_max_local_error_reset_streams(None)).await;
    assert_eq!(end, End::Open);
}

/// A limit above the late DATA that the server sends keeps the connection.
#[tokio::test]
async fn late_data_below_a_raised_local_error_reset_limit_keeps_the_connection() {
    let end = late_data_end(|endpoint| {
        endpoint.http2_max_local_error_reset_streams(STREAMS as usize * LATE_FRAMES)
    })
    .await;
    assert_eq!(end, End::Open);
}
