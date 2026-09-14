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

use integration_tests::pb::{test_client::TestClient, test_server, Input, Output};
use std::io;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::{net::TcpListener, net::TcpStream, sync::oneshot};
use tokio_stream::Stream;
use tonic::{
    transport::{server::TcpIncoming, Endpoint, Server},
    Code, Request, Response, Status,
};

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

/// A unary handler. The call waits until `hold` is received.
struct HoldSvc {
    started: Mutex<Option<oneshot::Sender<()>>>,
    hold: Mutex<Option<oneshot::Receiver<()>>>,
}

#[tonic::async_trait]
impl test_server::Test for HoldSvc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        let started = self.started.lock().unwrap().take();
        if let Some(tx) = started {
            let _ = tx.send(());
        }
        let hold = self.hold.lock().unwrap().take();
        if let Some(rx) = hold {
            let _ = rx.await;
        }
        Ok(Response::new(Output {}))
    }
}

/// Forwards polls to `inner`. Sends on `on_drop` when this value is dropped.
struct NotifyOnDrop<S> {
    inner: S,
    on_drop: Option<oneshot::Sender<()>>,
}

impl<S> Drop for NotifyOnDrop<S> {
    fn drop(&mut self) {
        if let Some(tx) = self.on_drop.take() {
            let _ = tx.send(());
        }
    }
}

impl<S: Stream + Unpin> Stream for NotifyOnDrop<S> {
    type Item = S::Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.inner).poll_next(cx)
    }
}

/// Shutdown must drop `incoming` before in-flight RPCs complete.
/// If `incoming` is a `TcpIncoming`, drop closes the listen socket.
#[tokio::test]
async fn shutdown_closes_listener_before_drain() {
    let (started_tx, started_rx) = oneshot::channel();
    let (hold_tx, hold_rx) = oneshot::channel();
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let (dropped_tx, dropped_rx) = oneshot::channel();

    let svc = test_server::TestServer::new(HoldSvc {
        started: Mutex::new(Some(started_tx)),
        hold: Mutex::new(Some(hold_rx)),
    });

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let incoming = NotifyOnDrop {
        inner: TcpIncoming::from(listener).with_nodelay(Some(true)),
        on_drop: Some(dropped_tx),
    };

    let jh = tokio::spawn(async move {
        Server::builder()
            .add_service(svc)
            .serve_with_incoming_shutdown(incoming, async { drop(shutdown_rx.await) })
            .await
            .unwrap();
    });

    let mut client = TestClient::connect(format!("http://{addr}")).await.unwrap();
    let call = tokio::spawn(async move { client.unary_call(Request::new(Input {})).await });
    started_rx.await.unwrap();

    shutdown_tx.send(()).unwrap();

    // Wait until `incoming` is dropped.
    // Do not call connect in a loop before that.
    // A connect loop can make `incoming.next()` ready.
    // Then `select!` can accept the connection and ignore shutdown.
    // The timeout is only a hang guard. On an unfixed server, drop
    // waits for drain, and drain waits for `hold`.
    tokio::time::timeout(Duration::from_secs(1), dropped_rx)
        .await
        .expect("incoming was not dropped before drain")
        .unwrap();

    let err = TcpStream::connect(addr)
        .await
        .expect_err("connect succeeded after incoming drop");
    assert!(
        matches!(
            err.kind(),
            io::ErrorKind::ConnectionRefused | io::ErrorKind::ConnectionReset
        ),
        "connect error was {:?}, not refused or reset",
        err.kind()
    );

    hold_tx.send(()).unwrap();
    call.await.unwrap().unwrap();
    jh.await.unwrap();
}
