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

//! A `!Send` service router for local mode.
//!
//! NB: keep in sync with src/service/router.rs

use std::{
    collections::HashMap,
    convert::Infallible,
    fmt, future,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use http::{Request, Response};
use tower_service::Service;

use crate::{Status, local::body::Body, server::NamedService};

type LocalBoxFuture = Pin<Box<dyn Future<Output = Result<Response<Body>, Infallible>> + 'static>>;

/// Object-safe, cloneable `!Send` service — a minimal mirror of
/// `tower::util::BoxCloneService` without the `Send` bound.
trait CloneLocalService {
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Infallible>>;
    fn call(&mut self, req: Request<Body>) -> LocalBoxFuture;
    fn clone_box(&self) -> Box<dyn CloneLocalService + 'static>;
}

impl<S, ResBody> CloneLocalService for S
where
    S: Service<Request<Body>, Response = Response<ResBody>, Error = Infallible> + Clone + 'static,
    S::Future: 'static,
    ResBody: http_body::Body<Data = bytes::Bytes> + 'static,
    ResBody::Error: Into<crate::BoxError>,
{
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Infallible>> {
        Service::poll_ready(self, cx)
    }

    fn call(&mut self, req: Request<Body>) -> LocalBoxFuture {
        let fut = Service::call(self, req);
        Box::pin(async move {
            let resp = fut.await?;
            Ok(resp.map(Body::new))
        })
    }

    fn clone_box(&self) -> Box<dyn CloneLocalService + 'static> {
        Box::new(self.clone())
    }
}

struct LocalBoxCloneService(Box<dyn CloneLocalService + 'static>);

impl Clone for LocalBoxCloneService {
    fn clone(&self) -> Self {
        Self(self.0.clone_box())
    }
}

/// A [`Service`] router for `!Send` services, routing on the gRPC service name
/// (the first path segment of `/{package.Service}/{Method}`).
#[derive(Clone, Default)]
pub struct Routes {
    svcs: HashMap<&'static str, LocalBoxCloneService>,
}

/// Allows adding new services to routes by passing a mutable reference to this builder.
#[derive(Default, Clone)]
pub struct RoutesBuilder {
    routes: Option<Routes>,
}

impl RoutesBuilder {
    /// Add a new service.
    pub fn add_service<S, ResBody>(&mut self, svc: S) -> &mut Self
    where
        S: Service<Request<Body>, Response = Response<ResBody>, Error = Infallible>
            + NamedService
            + Clone
            + 'static,
        S::Future: 'static,
        ResBody: http_body::Body<Data = bytes::Bytes> + 'static,
        ResBody::Error: Into<crate::BoxError>,
    {
        let routes = self.routes.take().unwrap_or_default();
        self.routes.replace(routes.add_service(svc));
        self
    }

    /// Returns the routes with added services or empty [`Routes`] if no service was added.
    pub fn routes(self) -> Routes {
        self.routes.unwrap_or_default()
    }
}

impl Routes {
    /// Create a new routes with `svc` already added to it.
    pub fn new<S, ResBody>(svc: S) -> Self
    where
        S: Service<Request<Body>, Response = Response<ResBody>, Error = Infallible>
            + NamedService
            + Clone
            + 'static,
        S::Future: 'static,
        ResBody: http_body::Body<Data = bytes::Bytes> + 'static,
        ResBody::Error: Into<crate::BoxError>,
    {
        Self::default().add_service(svc)
    }

    /// Create a new empty builder.
    pub fn builder() -> RoutesBuilder {
        RoutesBuilder::default()
    }

    /// Add a new service.
    pub fn add_service<S, ResBody>(mut self, svc: S) -> Self
    where
        S: Service<Request<Body>, Response = Response<ResBody>, Error = Infallible>
            + NamedService
            + Clone
            + 'static,
        S::Future: 'static,
        ResBody: http_body::Body<Data = bytes::Bytes> + 'static,
        ResBody::Error: Into<crate::BoxError>,
    {
        self.svcs
            .insert(S::NAME, LocalBoxCloneService(Box::new(svc)));
        self
    }
}

impl fmt::Debug for Routes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Routes")
            .field("services", &self.svcs.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl fmt::Debug for RoutesBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RoutesBuilder")
            .field("routes", &self.routes)
            .finish()
    }
}

fn unimplemented() -> Response<Body> {
    let (parts, ()) = Status::unimplemented("").into_http::<()>().into_parts();
    Response::from_parts(parts, Body::empty())
}

impl<B> Service<Request<B>> for Routes
where
    B: http_body::Body<Data = bytes::Bytes> + 'static,
    B::Error: Into<crate::BoxError>,
{
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = LocalBoxFuture;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let svc = req
            .uri()
            .path()
            .split('/')
            .nth(1)
            .and_then(|name| self.svcs.get(name))
            .cloned();

        match svc {
            Some(mut svc) => Box::pin(async move {
                let req = req.map(Body::new);
                future::poll_fn(|cx| svc.0.poll_ready(cx)).await?;
                svc.0.call(req).await
            }),
            None => Box::pin(future::ready(Ok(unimplemented()))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::rc::Rc;

    use super::*;

    /// A non-`Send` fake service answering with a marker header.
    #[derive(Clone)]
    struct FakeSvc {
        marker: &'static str,
        hits: Rc<Cell<u32>>,
    }

    impl Service<Request<Body>> for FakeSvc {
        type Response = Response<Body>;
        type Error = Infallible;
        type Future = future::Ready<Result<Self::Response, Self::Error>>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, _req: Request<Body>) -> Self::Future {
            self.hits.set(self.hits.get() + 1);
            let resp = Response::builder()
                .header("svc", self.marker)
                .body(Body::empty())
                .unwrap();
            future::ready(Ok(resp))
        }
    }

    struct SvcA;
    struct SvcB;

    impl NamedService for SvcA {
        const NAME: &'static str = "pkg.A";
    }
    impl NamedService for SvcB {
        const NAME: &'static str = "pkg.B";
    }

    struct Named<N>(FakeSvc, std::marker::PhantomData<N>);

    impl<N> Clone for Named<N> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), std::marker::PhantomData)
        }
    }

    impl<N> Service<Request<Body>> for Named<N> {
        type Response = Response<Body>;
        type Error = Infallible;
        type Future = future::Ready<Result<Self::Response, Self::Error>>;

        fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Service::poll_ready(&mut self.0, cx)
        }

        fn call(&mut self, req: Request<Body>) -> Self::Future {
            Service::call(&mut self.0, req)
        }
    }

    impl<N: NamedService> NamedService for Named<N> {
        const NAME: &'static str = N::NAME;
    }

    fn fake<N>(marker: &'static str, hits: &Rc<Cell<u32>>) -> Named<N> {
        Named(
            FakeSvc {
                marker,
                hits: hits.clone(),
            },
            std::marker::PhantomData,
        )
    }

    fn req(path: &str) -> Request<Body> {
        Request::builder().uri(path).body(Body::empty()).unwrap()
    }

    async fn call(routes: &mut Routes, path: &str) -> Response<Body> {
        Service::<Request<Body>>::call(routes, req(path))
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn routes_by_service_name() {
        let hits_a = Rc::new(Cell::new(0));
        let hits_b = Rc::new(Cell::new(0));
        let mut routes = Routes::default()
            .add_service(fake::<SvcA>("A", &hits_a))
            .add_service(fake::<SvcB>("B", &hits_b));

        let resp = call(&mut routes, "/pkg.A/Method").await;
        assert_eq!(resp.headers().get("svc").unwrap(), "A");

        let resp = call(&mut routes, "/pkg.B/Method").await;
        assert_eq!(resp.headers().get("svc").unwrap(), "B");

        assert_eq!((hits_a.get(), hits_b.get()), (1, 1));
    }

    #[tokio::test]
    async fn unknown_service_unimplemented() {
        let hits = Rc::new(Cell::new(0));
        let mut routes = Routes::new(fake::<SvcA>("A", &hits));

        let resp = call(&mut routes, "/nope/Method").await;
        assert_eq!(
            resp.headers().get("grpc-status").unwrap(),
            &(crate::Code::Unimplemented as i32).to_string()
        );
        assert_eq!(hits.get(), 0);
    }

    #[tokio::test]
    async fn routes_is_clone() {
        let hits = Rc::new(Cell::new(0));
        let routes = Routes::new(fake::<SvcA>("A", &hits));

        let mut c1 = routes.clone();
        let mut c2 = routes;
        call(&mut c1, "/pkg.A/M").await;
        call(&mut c2, "/pkg.A/M").await;
        assert_eq!(hits.get(), 2);
    }

    /// A service whose response body is a foreign type (not `local::body::Body`),
    /// mirroring what `with_interceptor(..)`-wrapped generated services produce.
    #[derive(Clone)]
    struct ForeignBodySvc;

    impl NamedService for ForeignBodySvc {
        const NAME: &'static str = "pkg.Foreign";
    }

    impl Service<Request<Body>> for ForeignBodySvc {
        type Response = Response<http_body_util::Full<bytes::Bytes>>;
        type Error = Infallible;
        type Future = future::Ready<Result<Self::Response, Self::Error>>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, _req: Request<Body>) -> Self::Future {
            let resp = Response::builder()
                .header("svc", "foreign")
                .body(http_body_util::Full::new(bytes::Bytes::from_static(b"hi")))
                .unwrap();
            future::ready(Ok(resp))
        }
    }

    #[tokio::test]
    async fn routes_accept_foreign_body() {
        use http_body_util::BodyExt as _;

        let mut routes = Routes::new(ForeignBodySvc);
        let resp = call(&mut routes, "/pkg.Foreign/Method").await;
        assert_eq!(resp.headers().get("svc").unwrap(), "foreign");
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(body, bytes::Bytes::from_static(b"hi"));
    }
}
