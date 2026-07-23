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

use super::Connected;
use std::sync::Arc;

/// Connection info for Unix domain socket streams.
///
/// This type will be accessible through [request extensions][ext] if you're using
/// a unix stream.
///
/// See [Connected] for more details.
///
/// [ext]: crate::Request::extensions
#[derive(Clone, Debug)]
pub struct UdsConnectInfo {
    /// Peer address. This will be "unnamed" for client unix sockets.
    pub peer_addr: Option<Arc<tokio::net::unix::SocketAddr>>,
    /// Process credentials for the unix socket.
    pub peer_cred: Option<tokio::net::unix::UCred>,
}

impl Connected for tokio::net::UnixStream {
    type ConnectInfo = UdsConnectInfo;

    fn connect_info(&self) -> Self::ConnectInfo {
        UdsConnectInfo {
            peer_addr: self.peer_addr().ok().map(Arc::new),
            peer_cred: self.peer_cred().ok(),
        }
    }
}
