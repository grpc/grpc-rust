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

use tonic::async_trait;

use crate::credentials::ProtocolInfo;
use crate::credentials::ServerCredentials;
use crate::credentials::server::HandshakeOutput as ServerHandshakeOutput;
use crate::private;
use crate::rt::BoxEndpoint;
use crate::rt::GrpcEndpoint;
use crate::rt::GrpcRuntime;
use crate::send_future::SendFuture;

// Bridge trait for type erasure.
#[async_trait]
pub(crate) trait DynServerCredentials: Send + Sync {
    async fn dyn_accept(
        &self,
        source: BoxEndpoint,
        runtime: GrpcRuntime,
    ) -> Result<ServerHandshakeOutput<BoxEndpoint>, String>;

    fn info(&self) -> &ProtocolInfo;
}

#[async_trait]
impl<T> DynServerCredentials for T
where
    T: ServerCredentials,
    T::Output<BoxEndpoint>: GrpcEndpoint,
{
    async fn dyn_accept(
        &self,
        source: BoxEndpoint,
        runtime: GrpcRuntime,
    ) -> Result<ServerHandshakeOutput<BoxEndpoint>, String> {
        let output = SendFuture::make_send(self.accept(source, runtime, private::Internal)).await?;
        Ok(ServerHandshakeOutput {
            endpoint: Box::new(output.endpoint),
            security: output.security,
        })
    }

    fn info(&self) -> &ProtocolInfo {
        self.info()
    }
}

#[cfg(test)]
mod tests {
    use tokio::io::AsyncReadExt;
    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpListener;
    use tokio::net::TcpStream;

    use super::*;
    use crate::credentials::LocalServerCredentials;
    use crate::credentials::SecurityLevel;
    use crate::rt;
    use crate::rt::EndpointIoStream;
    use crate::rt::StreamEndpoint;

    #[tokio::test]
    async fn test_dyn_server_credential_dispatch() {
        let creds = LocalServerCredentials::new();
        let dyn_creds: Box<dyn DynServerCredentials> = Box::new(creds);

        let info = dyn_creds.info();
        assert_eq!(info.security_protocol, "local");

        let addr = "127.0.0.1:0";
        let runtime = rt::default_runtime();
        let listener = TcpListener::bind(addr).await.unwrap();
        let server_addr = listener.local_addr().unwrap();

        let client_handle = tokio::spawn(async move {
            let mut stream = TcpStream::connect(server_addr).await.unwrap();
            let data = b"hello dynamic grpc server";
            stream.write_all(data).await.unwrap();

            // Keep the connection alive for a bit so server can read
            let mut buf = vec![0u8; 1];
            let _ = stream.read(&mut buf).await;
        });

        let (stream, _) = listener.accept().await.unwrap();
        let server_stream = StreamEndpoint::new_from_tcp(stream).unwrap();

        let result = dyn_creds
            .dyn_accept(Box::new(server_stream) as Box<dyn GrpcEndpoint>, runtime)
            .await;

        assert!(result.is_ok());
        let output = result.unwrap();
        let endpoint = output.endpoint;
        let security_info = output.security;

        assert_eq!(security_info.security_protocol(), "local");
        assert_eq!(security_info.security_level(), SecurityLevel::NoSecurity);

        let mut buf = vec![0u8; 25];
        EndpointIoStream::new(endpoint)
            .read_exact(&mut buf)
            .await
            .unwrap();
        assert_eq!(&buf[..], b"hello dynamic grpc server");

        client_handle.abort();
    }
}
