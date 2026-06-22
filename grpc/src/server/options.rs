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

/// A read-only snapshot of the server-wide options that apply across all
/// connections.
///
/// Options are configured through [`ServerBuilder`](crate::server::builder::ServerBuilder),
/// which is the single mutation surface. This type is the read side: obtain it
/// via [`Server::options()`](crate::server::Server::options) to introspect the
/// effective configuration a server is running with.
///
/// # Planned options
///
/// This type is currently empty because no server-wide options are enforced
/// yet. Options that have been considered but are not implemented:
///
/// - **Maximum concurrent RPCs** (`max_concurrent_rpcs`): a cap on the number
///   of in-flight RPCs across all connections, beyond which new RPCs would be
///   rejected with `RESOURCE_EXHAUSTED`. This was previously exposed on the
///   builder but removed because nothing enforced it (it was a no-op). It
///   should be reintroduced alongside the machinery that actually applies the
///   limit.
#[derive(Debug, Clone, Default)]
pub struct ServerOptions {}
