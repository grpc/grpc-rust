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

//! Client-side cancellation support.

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::task::Waker;

use atomic_waker::AtomicWaker;
use http::Extensions;

/// Shared state used to coordinate cancellation of an outbound request stream.
///
/// It uses `AtomicBool` for the cancellation flag and `AtomicWaker` to store
/// the waker of the task polling the stream. This allows lock-free coordination
/// between the canceller (holder of `CancellationHandle`) and the poller
/// (holder of `CancellationListener`).
///
/// Supports only one waker at a time.
#[derive(Default, Debug)]
pub(crate) struct CancellationState {
    cancellation_requested: AtomicBool,
    poll_waker: AtomicWaker,
}

impl CancellationListener {
    /// Attempts to create a new `CancellationListener` by extracting the shared
    /// `CancellationState` from the provided `Extensions`.
    ///
    /// If the state is present, it is removed from `Extensions` to ensure that
    /// only one listener can be created for a given cancellation state. This
    /// enforces the single-waker constraint of `CancellationState`.
    pub(crate) fn new(extensions: &mut Extensions) -> Option<CancellationListener> {
        // We can't send a listener in extensions as it doesn't implement Clone
        // (to ensure only one waker is registered).
        // Since we want to propagate the cancellation capability, we use
        // `Arc<CancellationState>` for propagation in extensions and remove it
        // here to avoid further creation of listeners with the same state.
        extensions
            .remove::<Arc<CancellationState>>()
            .map(|state| CancellationListener { state })
    }

    /// Registers a new waker to be notified when cancellation is requested,
    /// and returns the current cancellation state.
    ///
    /// Returns `true` if cancellation has already been requested, `false`
    /// otherwise.
    ///
    /// The registration and check are ordered to ensure that if a cancellation
    /// happens concurrently, either the canceller sees the waker and wakes it,
    /// or this method returns `true` (or both).
    pub(crate) fn update_waker(&mut self, waker: &Waker) -> bool {
        // Order of operations is important to ensure correct synchronization:
        // 1. Register the waker.
        self.state.poll_waker.register(waker);
        // 2. Load the cancellation flag.
        // If cancellation happened before register, we might see it here and
        // return true.
        // If cancellation happens after register, the canceller will see the
        // registered waker and wake us.
        self.state.cancellation_requested.load(Ordering::Acquire)
    }
}

/// A handle that allows triggering the cancellation of an outbound request
/// stream.
///
/// This handle can be stored by the user and invoked to abort the request.
#[derive(Debug)]
pub struct CancellationHandle {
    state: Arc<CancellationState>,
}

/// A listener that can check if cancellation has been requested and register
/// a single waker to be notified when cancellation occurs.
///
/// This is used by the task responsible for sending the outbound request stream
/// to react to cancellation requests.
#[derive(Debug)]
pub(crate) struct CancellationListener {
    state: Arc<CancellationState>,
}

impl CancellationHandle {
    /// Creates a new `CancellationHandle` and inserts the shared
    /// `CancellationState` into the provided `Extensions`.
    //
    // The corresponding `CancellationListener` can later be extracted from the
    // same extensions using `CancellationListener::new`.
    pub fn new(extensions: &mut Extensions) -> CancellationHandle {
        let state = Arc::new(CancellationState::default());
        extensions.insert(Arc::clone(&state));
        CancellationHandle { state }
    }

    /// Cancel the stream.
    ///
    /// This will result in the outbound stream getting cancelled with an
    /// RST_STREAM frame, if it hasn't gracefully closed.
    ///
    /// Consumes `self` to ensure cancellation is only requested once.
    pub fn cancel(self) {
        // Order of operations is important to ensure correct synchronization:
        // 1. Set the cancellation flag.
        // 2. Wake the registered waker.
        //
        // If `update_waker` registered the waker before we set the flag, we will
        // wake it here.
        // If `update_waker` registers the waker after we set the flag, it will
        // see the flag as `true` and return immediately.
        if self
            .state
            .cancellation_requested
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            self.state.poll_waker.wake();
        }
    }
}
