use http::{header::HeaderName, HeaderValue};
use integration_tests::pb::{test_client::TestClient, test_server, Input, Output};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::{net::TcpListener, sync::oneshot};
use tonic::{
    transport::{server::TcpIncoming, Endpoint, Server},
    Code, Request, Response, Status,
};
use tower_http::set_header::SetRequestHeaderLayer;

struct Svc(Arc<Mutex<Option<oneshot::Sender<()>>>>);

#[tonic::async_trait]
impl test_server::Test for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        let mut l = self.0.lock().unwrap();
        l.take().unwrap().send(()).unwrap();

        Ok(Response::new(Output {}))
    }
}

#[tokio::test]
async fn connect_returns_err() {
    let res = TestClient::connect("http://thisdoesntexist.test").await;

    assert!(res.is_err());
}

#[tokio::test]
async fn connect_handles_tls() {
    rustls::crypto::ring::default_provider()
        .install_default()
        .unwrap();
    TestClient::connect("https://github.com").await.unwrap();
}

#[tokio::test]
async fn connect_returns_err_via_call_after_connected() {
    let (tx, rx) = oneshot::channel();
    let sender = Arc::new(Mutex::new(Some(tx)));
    let svc = test_server::TestServer::new(Svc(sender));

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let incoming = TcpIncoming::from(listener).with_nodelay(Some(true));

    let jh = tokio::spawn(async move {
        Server::builder()
            .add_service(svc)
            .serve_with_incoming_shutdown(incoming, async { drop(rx.await) })
            .await
            .unwrap();
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    let mut client = TestClient::connect(format!("http://{addr}")).await.unwrap();

    // First call should pass, then shutdown the server
    client.unary_call(Request::new(Input {})).await.unwrap();

    tokio::time::sleep(Duration::from_millis(100)).await;

    let res = client.unary_call(Request::new(Input {})).await;

    let err = res.unwrap_err();
    assert_eq!(err.code(), Code::Unavailable);

    jh.await.unwrap();
}

#[tokio::test]
async fn endpoint_layer_stacks_and_applies_to_requests() {
    struct HeaderSvc;

    #[tonic::async_trait]
    impl test_server::Test for HeaderSvc {
        async fn unary_call(&self, req: Request<Input>) -> Result<Response<Output>, Status> {
            // Both layers must have been applied for this to succeed, proving the
            // layers stack rather than the second `layer` call replacing the first.
            match (
                req.metadata().get("x-first"),
                req.metadata().get("x-second"),
            ) {
                (Some(_), Some(_)) => Ok(Response::new(Output {})),
                _ => Err(Status::internal("a header set by a layer is missing")),
            }
        }
    }

    let (tx, rx) = oneshot::channel();
    let svc = test_server::TestServer::new(HeaderSvc);

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let incoming = TcpIncoming::from(listener).with_nodelay(Some(true));

    let jh = tokio::spawn(async move {
        Server::builder()
            .add_service(svc)
            .serve_with_incoming_shutdown(incoming, async { drop(rx.await) })
            .await
            .unwrap();
    });

    // The layers are configured directly on the `Endpoint`, so no type
    // parameters leak onto the resulting `Channel`.
    let channel = Endpoint::from_shared(format!("http://{addr}"))
        .unwrap()
        .layer(SetRequestHeaderLayer::overriding(
            HeaderName::from_static("x-first"),
            HeaderValue::from_static("first"),
        ))
        .layer(SetRequestHeaderLayer::overriding(
            HeaderName::from_static("x-second"),
            HeaderValue::from_static("second"),
        ))
        .connect_lazy();

    let mut client = TestClient::new(channel);

    tokio::time::sleep(Duration::from_millis(100)).await;
    client.unary_call(Request::new(Input {})).await.unwrap();

    tx.send(()).unwrap();
    jh.await.unwrap();
}

#[tokio::test]
async fn connect_lazy_reconnects_after_first_failure() {
    let (tx, rx) = oneshot::channel();
    let sender = Arc::new(Mutex::new(Some(tx)));
    let svc = test_server::TestServer::new(Svc(sender));

    {
        let channel = Endpoint::from_static("http://127.0.0.1:0").connect_lazy();
        let mut client = TestClient::new(channel);

        // First call should fail, the server is not running
        client.unary_call(Request::new(Input {})).await.unwrap_err();
    }

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let incoming = TcpIncoming::from(listener).with_nodelay(Some(true));

    // Start the server now, second call should succeed
    let jh = tokio::spawn(async move {
        Server::builder()
            .add_service(svc)
            .serve_with_incoming_shutdown(incoming, async { drop(rx.await) })
            .await
            .unwrap();
    });

    let channel = Endpoint::from_shared(format!("http://{addr}"))
        .unwrap()
        .connect_lazy();

    let mut client = TestClient::new(channel);

    tokio::time::sleep(Duration::from_millis(100)).await;
    client.unary_call(Request::new(Input {})).await.unwrap();

    // The server shut down, third call should fail
    tokio::time::sleep(Duration::from_millis(100)).await;
    let err = client.unary_call(Request::new(Input {})).await.unwrap_err();

    assert_eq!(err.code(), Code::Unavailable);

    jh.await.unwrap();
}
