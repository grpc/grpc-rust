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

//! Client-side cancellation support.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use atomic_waker::AtomicWaker;

/// Shared state used to coordinate cancellation of an outbound request stream.
#[derive(Default, Debug)]
pub struct CancellationState {
    pub(crate) cancellation_requested: AtomicBool,
    pub(crate) poll_waker: AtomicWaker,
}

impl CancellationState {
    /// Request cancellation.
    pub fn cancel(&self) {
        if self
            .cancellation_requested
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            self.poll_waker.wake();
        }
    }
}

/// A handle to cancel an outbound request stream.
#[derive(Clone, Debug)]
pub struct CancelHandle {
    state: Arc<CancellationState>,
}

impl CancelHandle {
    pub(crate) fn new(state: Arc<CancellationState>) -> Self {
        Self { state }
    }

    /// Cancel the stream.
    pub fn cancel(&self) {
        self.state.cancel();
    }
}
