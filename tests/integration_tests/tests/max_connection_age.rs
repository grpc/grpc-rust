use std::time::Duration;

use integration_tests::pb::{test_stream_client, test_stream_server, InputStream, OutputStream};
use tokio::net::TcpListener;
use tokio_stream::StreamExt;
use tonic::transport::{server::TcpIncoming, Server};
use tonic::{Request, Response, Status};

type Stream<T> = std::pin::Pin<
    Box<dyn tokio_stream::Stream<Item = std::result::Result<T, Status>> + Send + 'static>,
>;

const TICK: Duration = Duration::from_millis(50);

/// Streams `items` messages, one every [`TICK`].
struct Svc {
    items: usize,
}

#[tonic::async_trait]
impl test_stream_server::TestStream for Svc {
    type StreamCallStream = Stream<OutputStream>;

    async fn stream_call(
        &self,
        _: Request<InputStream>,
    ) -> Result<Response<Self::StreamCallStream>, Status> {
        let s = tokio_stream::iter(0..self.items).then(|_| async {
            tokio::time::sleep(TICK).await;
            Ok(OutputStream {})
        });
        Ok(Response::new(Box::pin(s) as Self::StreamCallStream))
    }
}

async fn serve(items: usize, age: Duration, grace: Option<Duration>) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let incoming = TcpIncoming::from(listener).with_nodelay(Some(true));

    let mut server = Server::builder().max_connection_age(age);
    if let Some(grace) = grace {
        server = server.max_connection_age_grace(grace);
    }
    let router = server.add_service(test_stream_server::TestStreamServer::new(Svc { items }));
    tokio::spawn(async move { router.serve_with_incoming(incoming).await.unwrap() });

    format!("http://{addr}")
}

/// Count the messages a `StreamCall` delivers before it ends or fails.
async fn drain(uri: String) -> (usize, Result<(), Status>) {
    let mut client = test_stream_client::TestStreamClient::connect(uri)
        .await
        .unwrap();
    let mut stream = client
        .stream_call(Request::new(InputStream {}))
        .await
        .unwrap()
        .into_inner();
    let mut received = 0;
    loop {
        match stream.next().await {
            Some(Ok(_)) => received += 1,
            Some(Err(status)) => return (received, Err(status)),
            None => return (received, Ok(())),
        }
    }
}

/// A bare gRPC request for `StreamCall` carrying one empty message.
fn stream_call_request(uri: &str) -> (http::Request<()>, bytes::Bytes) {
    let request = http::Request::builder()
        .method("POST")
        .uri(format!("{uri}/stream.TestStream/StreamCall"))
        .header("content-type", "application/grpc")
        .header("te", "trailers")
        .body(())
        .unwrap();
    (request, bytes::Bytes::from_static(&[0, 0, 0, 0, 0]))
}

#[tokio::test]
async fn graceful_shutdown_starts_at_max_connection_age_when_grace_is_set() {
    // The grace is far longer than the test: everything observed here happens
    // in the graceful phase, which must begin at `max_connection_age`.
    let uri = serve(
        100,
        Duration::from_millis(200),
        Some(Duration::from_secs(60)),
    )
    .await;

    let tcp = tokio::net::TcpStream::connect(uri.trim_start_matches("http://"))
        .await
        .unwrap();
    let (mut h2, connection) = h2::client::handshake(tcp).await.unwrap();
    tokio::spawn(async move {
        let _ = connection.await;
    });

    // An in-flight stream keeps the connection open through the graceful phase.
    let (request, message) = stream_call_request(&uri);
    let (response, mut body) = h2.send_request(request, false).unwrap();
    body.send_data(message, true).unwrap();
    let response = response.await.unwrap();
    assert_eq!(response.status(), http::StatusCode::OK);

    tokio::time::sleep(Duration::from_millis(500)).await;

    // Past `max_connection_age`, the server has sent GOAWAY, so it must refuse
    // new streams on this connection.
    let (request, message) = stream_call_request(&uri);
    let refused = async {
        let mut h2 = h2.ready().await?;
        let (response, mut body) = h2.send_request(request, false)?;
        body.send_data(message, true)?;
        response.await
    }
    .await
    .expect_err("a stream opened after max_connection_age must be refused");
    assert!(
        refused.is_go_away() || refused.reason() == Some(h2::Reason::REFUSED_STREAM),
        "expected GOAWAY or REFUSED_STREAM, got {refused:?}"
    );
}

#[tokio::test]
async fn in_flight_stream_completes_within_max_connection_age_grace() {
    // 20 messages over ~1s: crosses `max_connection_age`, ends well inside the grace.
    let uri = serve(20, Duration::from_millis(200), Some(Duration::from_secs(5))).await;

    let (received, result) = drain(uri).await;

    assert!(
        result.is_ok(),
        "stream failed after {received} messages: {result:?}"
    );
    assert_eq!(received, 20);
}

#[tokio::test]
async fn connection_is_closed_at_max_connection_age_plus_grace() {
    // Effectively unbounded: only the forced close can end it.
    let uri = serve(
        1_000,
        Duration::from_millis(200),
        Some(Duration::from_millis(300)),
    )
    .await;

    let started = tokio::time::Instant::now();
    let (received, result) = drain(uri).await;
    let elapsed = started.elapsed();

    assert!(result.is_err(), "stream should be cut by the forced close");
    assert!(received < 1_000);
    assert!(
        elapsed >= Duration::from_millis(500) && elapsed < Duration::from_secs(3),
        "forced close should land at age + grace (500ms), took {elapsed:?}"
    );
}
