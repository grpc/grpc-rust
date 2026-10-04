use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use http::Uri;
use hyper_util::rt::TokioIo;

use tower::Service;

use crate::status::ConnectError;

#[cfg(unix)]
use tokio::net::UnixStream;

#[cfg(unix)]
async fn connect_uds(uds_path: String) -> Result<UnixStream, ConnectError> {
    UnixStream::connect(uds_path)
        .await
        .map_err(|err| ConnectError(From::from(err)))
}

// Dummy type that will allow us to compile and match trait bounds
// but is never used.
#[cfg(not(unix))]
#[allow(dead_code)]
type UnixStream = tokio::io::DuplexStream;

#[cfg(not(unix))]
async fn connect_uds(_uds_path: String) -> Result<UnixStream, ConnectError> {
    Err(ConnectError(
        "uds connections are not supported on this platform".into(),
    ))
}

pub(crate) struct UdsConnector {
    uds_filepath: String,
}

impl UdsConnector {
    pub(crate) fn new(uds_filepath: &str) -> Self {
        UdsConnector {
            uds_filepath: uds_filepath.to_string(),
        }
    }
}

impl Service<Uri> for UdsConnector {
    type Response = TokioIo<UnixStream>;
    type Error = ConnectError;
    type Future = UdsConnecting;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: Uri) -> Self::Future {
        let uds_path = self.uds_filepath.clone();
        let fut = async move {
            let stream = connect_uds(uds_path).await?;
            Ok(TokioIo::new(stream))
        };
        UdsConnecting {
            inner: Box::pin(fut),
        }
    }
}

type ConnectResult = Result<TokioIo<UnixStream>, ConnectError>;

pub(crate) struct UdsConnecting {
    inner: Pin<Box<dyn Future<Output = ConnectResult> + Send>>,
}

impl Future for UdsConnecting {
    type Output = ConnectResult;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.get_mut().inner.as_mut().poll(cx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    #[tokio::test]
    async fn connects_to_unix_socket() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("socket");
        let listener = tokio::net::UnixListener::bind(&path).unwrap();
        let mut connector = UdsConnector::new(path.to_str().unwrap());

        let stream = connector
            .call(Uri::from_static("http://localhost"))
            .await
            .unwrap()
            .into_inner();
        let (_accepted, _) = listener.accept().await.unwrap();

        assert_eq!(
            stream.peer_addr().unwrap().as_pathname(),
            Some(path.as_path())
        );
    }

    #[cfg(not(unix))]
    #[tokio::test]
    async fn rejects_unix_socket_on_unsupported_platform() {
        let mut connector = UdsConnector::new("socket");
        let error = connector
            .call(Uri::from_static("http://localhost"))
            .await
            .unwrap_err();

        assert_eq!(
            error.to_string(),
            "uds connections are not supported on this platform"
        );
    }
}
