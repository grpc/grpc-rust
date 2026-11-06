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

//! Adapts the `grpc` crate's [`GrpcRuntime`] to [`xds_client::Runtime`].

use std::future::Future;
use std::time::Duration;

use grpc::__unstable::rt::GrpcRuntime;

/// An [`xds_client::Runtime`] backed by a [`GrpcRuntime`].
#[derive(Clone, Debug)]
pub struct XdsRuntime(GrpcRuntime);

impl XdsRuntime {
    /// Wraps `runtime` for use by `xds-client`.
    pub fn new(runtime: GrpcRuntime) -> Self {
        Self(runtime)
    }
}

impl xds_client::Runtime for XdsRuntime {
    fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        // xds-client has no use for the handle; dropping it detaches the task.
        self.0.spawn(Box::pin(future));
    }

    fn sleep(&self, duration: Duration) -> impl Future<Output = ()> + Send {
        self.0.sleep(duration)
    }
}

#[cfg(test)]
mod tests {
    use tokio::sync::oneshot;
    use xds_client::Runtime as _;

    use super::*;

    #[tokio::test]
    async fn spawn_runs_task() {
        let runtime = XdsRuntime::new(GrpcRuntime::default());
        let (tx, rx) = oneshot::channel();
        runtime.spawn(async move {
            tx.send(()).unwrap();
        });
        rx.await.expect("spawned task ran");
    }

    #[tokio::test]
    async fn sleep_waits_for_duration() {
        let runtime = XdsRuntime::new(GrpcRuntime::default());
        let start = std::time::Instant::now();
        runtime.sleep(Duration::from_millis(20)).await;
        assert!(start.elapsed() >= Duration::from_millis(20));
    }
}
