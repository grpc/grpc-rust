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

//! The cached state of each watched xDS resource or DNS target.

use std::sync::Arc;

use xds_client::ProcessingDone;
use xds_client::ResourceEvent;

/// The latest known state of one watched resource or DNS target.
///
/// Follows gRFC A88: a *resource error* (the resource itself is bad) drops
/// the cached value, while an *ambient error* (such as a broken stream) keeps
/// it and only adds a note.
///
/// `T` may be unsized so that DNS watches can hold `[EndpointAddress]`
/// (`Arc<[EndpointAddress]>`).
#[derive(Debug, PartialEq, Eq)]
pub(super) enum WatchState<T: ?Sized> {
    /// The watch has started but nothing has been received yet.
    Pending,
    /// A usable value. `note` holds the latest ambient error, if any; the next
    /// valid update clears it.
    Ready {
        value: Arc<T>,
        note: Option<Arc<str>>,
    },
    /// Nothing usable: the resource does not exist, is invalid, or an error
    /// arrived before the first valid value.
    Failed(Arc<str>),
}

// Manual `Clone` avoids `#[derive(Clone)]`'s `T: Clone` bound, which unsized
// values like `[EndpointAddress]` cannot satisfy even though `Arc<T>: Clone`.
impl<T: ?Sized> Clone for WatchState<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Pending => Self::Pending,
            Self::Ready { value, note } => Self::Ready {
                value: Arc::clone(value),
                note: note.clone(),
            },
            Self::Failed(reason) => Self::Failed(Arc::clone(reason)),
        }
    }
}

impl<T: ?Sized> WatchState<T> {
    /// Applies an XdsClient event, and returns its flow-control token.
    ///
    /// `describe` turns the event's error into the reason or note to store.
    pub(super) fn apply(
        &mut self,
        event: ResourceEvent<T>,
        describe: impl FnOnce(xds_client::Error) -> Arc<str>,
    ) -> ProcessingDone
    where
        T: Sized,
    {
        match event {
            ResourceEvent::ResourceChanged { result, done } => {
                self.on_changed(result.map_err(describe));
                done
            }
            ResourceEvent::AmbientError { error, done } => {
                self.on_ambient_error(describe(error));
                done
            }
        }
    }

    /// Applies a `ResourceChanged` event. An `Err` discards any cached value.
    fn on_changed(&mut self, result: Result<Arc<T>, Arc<str>>) {
        *self = match result {
            Ok(value) => Self::Ready { value, note: None },
            Err(reason) => Self::Failed(reason),
        };
    }

    /// Applies an `AmbientError` event: attaches `reason` to the cached value
    /// as a note.
    ///
    /// The XdsClient sends ambient errors only once a value is cached, and
    /// otherwise sends a `ResourceChanged` error (gRFC A88). Any other ambient
    /// error is ignored, so that it can't hide the reason a watch failed.
    fn on_ambient_error(&mut self, reason: Arc<str>) {
        if let Self::Ready { note, .. } = self {
            *note = Some(reason);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    enum Event {
        Changed(Result<u32, &'static str>),
        Ambient(&'static str),
    }

    fn ready(value: u32, note: Option<&str>) -> WatchState<u32> {
        WatchState::Ready {
            value: Arc::new(value),
            note: note.map(Arc::from),
        }
    }

    fn failed(reason: &str) -> WatchState<u32> {
        WatchState::Failed(reason.into())
    }

    #[test]
    fn events_follow_a88_transitions() {
        use Event::Ambient;
        use Event::Changed;
        use WatchState::Pending;

        for (start, event, expected) in [
            // A valid resource always replaces the state and clears the note.
            (Pending, Changed(Ok(2)), ready(2, None)),
            (ready(1, Some("old")), Changed(Ok(2)), ready(2, None)),
            (failed("old"), Changed(Ok(2)), ready(2, None)),
            // A resource error always discards the cached value.
            (Pending, Changed(Err("gone")), failed("gone")),
            (ready(1, None), Changed(Err("gone")), failed("gone")),
            (failed("old"), Changed(Err("gone")), failed("gone")),
            // An ambient error keeps a cached value and only replaces its note.
            (ready(1, None), Ambient("down"), ready(1, Some("down"))),
            (
                ready(1, Some("old")),
                Ambient("down"),
                ready(1, Some("down")),
            ),
            // Without a cached value, it's ignored, keeping the failure reason.
            (Pending, Ambient("down"), Pending),
            (failed("gone"), Ambient("down"), failed("gone")),
        ] {
            let mut state = start.clone();
            match &event {
                Changed(result) => state.on_changed((*result).map(Arc::new).map_err(Arc::from)),
                Ambient(reason) => state.on_ambient_error(Arc::from(*reason)),
            }
            assert_eq!(state, expected, "{start:?} after {event:?}");
        }
    }

    #[test]
    fn supports_unsized_slice_values() {
        let mut state: WatchState<[u32]> = WatchState::Pending;
        state.on_changed(Ok(Arc::from([1, 2])));
        state.on_ambient_error(Arc::from("stale"));

        let cloned = state.clone();
        assert_eq!(
            cloned,
            WatchState::Ready {
                value: Arc::from([1, 2]),
                note: Some(Arc::from("stale")),
            }
        );
    }
}
