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

//! Single-threaded transport: a hyper executor that spawns onto the current
//! `tokio::task::LocalSet` (or `LocalRuntime`), plus the server and channel
//! built on top of it.
//!
//! Calling into this module requires a tokio local context: a
//! `tokio::task::LocalSet` (any tokio 1.x) or a `tokio::runtime::LocalRuntime`
//! (tokio >= 1.51). `tokio::task::spawn_local` panics outside of one.

use std::future::Future;

pub mod channel;
pub mod server;

/// A [`hyper::rt::Executor`] that spawns futures onto the current
/// `tokio::task::LocalSet`/`LocalRuntime` via `tokio::task::spawn_local`.
///
/// Panics if used outside a tokio local context; not guarded, matches
/// `spawn_local`'s own contract.
#[derive(Clone, Copy, Debug)]
pub(crate) struct LocalExec;

impl<F: Future + 'static> hyper::rt::Executor<F> for LocalExec {
    fn execute(&self, fut: F) {
        tokio::task::spawn_local(fut);
    }
}
