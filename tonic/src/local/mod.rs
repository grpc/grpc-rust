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

//! Single-threaded (`!Send`) gRPC client and server support.
//!
//! This module mirrors the public surface of [`crate::client`],
//! [`crate::server`], [`crate::codec`] and [`crate::body`], minus every
//! `Send`/`Sync` bound, so handlers, services, streams and messages can hold
//! `!Send` state (`Rc`, `RefCell`, ...). Generated code opts in with a
//! codegen builder flag (`tonic_prost_build::Builder::local(true)` /
//! `tonic_build::CodeGenBuilder::local(true)`) and always addresses this
//! module through full paths, never through the re-exports below.
//!
//! # Runtime contract
//!
//! Every async entry point here — the local `Channel`, the local `Server`,
//! and generated local clients/servers — drives its work with
//! `tokio::task::spawn_local`, so it must run inside a tokio *local*
//! context:
//! - a [`tokio::task::LocalSet`] (works on any tokio 1.x), or
//! - a `tokio::runtime::LocalRuntime` (stable since tokio 1.51).
//!
//! Calling in from outside one of these panics, matching `spawn_local`'s own
//! contract.
//!
//! # Errors stay `Send + Sync`
//!
//! [`crate::Status`] and this crate's boxed error type are unchanged: still
//! `Send + Sync`. That is a transport requirement (hyper's HTTP/2 error
//! signatures need `Into<Box<dyn std::error::Error + Send + Sync>>`, which is
//! not pluggable) and `Status` stores its source in a `Send + Sync` box for
//! the same reason. It constrains only the *error value*, never the
//! handlers, futures, streams or messages that produce it — exactly the
//! bound this module removes.
//!
//! See [grpc/grpc-rust#2790](https://github.com/grpc/grpc-rust/issues/2790)
//! for the tracking issue.

pub mod body;
pub mod client;
pub mod codec;
pub mod request;
pub mod router;
pub mod server;

#[cfg(feature = "local-transport")]
pub mod transport;

pub use body::Body;
pub use codec::Streaming;
pub use router::{Routes, RoutesBuilder};

/// A minimal `!Send` channel and server built on the same HTTP/2 transport
/// used by [`crate::transport`], re-exported here for convenience (generated
/// code still spells these out as `tonic::local::transport::{channel::Channel,
/// server::Server}`).
///
/// ```no_run
/// use std::convert::Infallible;
/// use std::task::{Context, Poll};
///
/// use http::{Request, Response};
/// use tower_service::Service;
///
/// use tonic::local::{Body, Channel, Server};
/// use tonic::server::NamedService;
///
/// #[derive(Clone)]
/// struct Echo;
///
/// impl NamedService for Echo {
///     const NAME: &'static str = "example.Echo";
/// }
///
/// impl Service<Request<Body>> for Echo {
///     type Response = Response<Body>;
///     type Error = Infallible;
///     type Future = std::future::Ready<Result<Self::Response, Self::Error>>;
///
///     fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Infallible>> {
///         Poll::Ready(Ok(()))
///     }
///
///     fn call(&mut self, _req: Request<Body>) -> Self::Future {
///         std::future::ready(Ok(Response::new(Body::empty())))
///     }
/// }
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     tokio::task::LocalSet::new()
///         .run_until(async {
///             let addr = "127.0.0.1:50051".parse()?;
///             let router = Server::builder().add_service(Echo);
///             tokio::task::spawn_local(async move {
///                 let _ = router.serve(addr).await;
///             });
///
///             let mut channel = Channel::connect("http://127.0.0.1:50051".parse()?).await?;
///             std::future::poll_fn(|cx| channel.poll_ready(cx)).await?;
///             let req = Request::builder()
///                 .uri("/example.Echo/Method")
///                 .body(Body::empty())?;
///             let _resp = channel.call(req).await?;
///             Ok(())
///         })
///         .await
/// }
/// ```
#[cfg(feature = "local-transport")]
pub use transport::{channel::Channel, server::Server};
