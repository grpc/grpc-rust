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

use std::{
    marker::PhantomData,
    task::{Context, Poll},
};

use tower_layer::Layer;
use tower_service::Service;

use crate::server::NamedService;

/// A layered service to propagate [`NamedService`] implementation.
#[derive(Debug, Clone)]
pub struct Layered<S, T> {
    inner: S,
    _ty: PhantomData<T>,
}

impl<S, T: NamedService> NamedService for Layered<S, T> {
    const NAME: &'static str = T::NAME;
}

impl<Req, S, T> Service<Req> for Layered<S, T>
where
    S: Service<Req>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Req) -> Self::Future {
        self.inner.call(req)
    }
}

/// Extension trait which adds utility methods to types which implement [`tower_layer::Layer`].
pub trait LayerExt<L>: sealed::Sealed {
    /// Applies the layer to a service and wraps it in [`Layered`].
    fn named_layer<S>(&self, service: S) -> Layered<L::Service, S>
    where
        L: Layer<S>;
}

impl<L> LayerExt<L> for L {
    fn named_layer<S>(&self, service: S) -> Layered<<L>::Service, S>
    where
        L: Layer<S>,
    {
        Layered {
            inner: self.layer(service),
            _ty: PhantomData,
        }
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T> Sealed for T {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default)]
    struct TestService {}

    const TEST_SERVICE_NAME: &str = "test-service-name";

    impl NamedService for TestService {
        const NAME: &'static str = TEST_SERVICE_NAME;
    }

    // Checks if the argument implements `NamedService` and returns the implemented `NAME`.
    fn get_name_of_named_service<S: NamedService>(_s: &S) -> &'static str {
        S::NAME
    }

    #[test]
    fn named_service_is_propagated_to_layered() {
        use std::time::Duration;
        use tower::{limit::ConcurrencyLimitLayer, timeout::TimeoutLayer};

        let layered = TimeoutLayer::new(Duration::from_secs(5)).named_layer(TestService::default());
        assert_eq!(get_name_of_named_service(&layered), TEST_SERVICE_NAME);

        let layered = ConcurrencyLimitLayer::new(3).named_layer(layered);
        assert_eq!(get_name_of_named_service(&layered), TEST_SERVICE_NAME);
    }
}
