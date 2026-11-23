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

//! Client-side adaptive throttling for the RLS control-plane channel.
//!
//! Uses a fixed-size ring buffer of time buckets to estimate the probability
//! that the RLS server will reject or fail the next request:
//!
//! ```text
//! throttle_probability = (requests - ratio_for_accepts * accepts) / (requests + requests_padding)
//! ```
//!
//! Callers are responsible for external synchronization (all mutating methods
//! take `&mut self`).

use std::num::NonZeroU64;
use std::time::Duration;
use std::time::Instant;

const DEFAULT_DURATION: Duration = Duration::from_secs(30);
const DEFAULT_BINS: usize = 100;
const DEFAULT_RATIO_FOR_ACCEPTS: f64 = 2.0;
const DEFAULT_REQUESTS_PADDING: f64 = 8.0;

/// Outcome of an RLS request that was sent to the backend, as reported to
/// [`AdaptiveThrottler::register_backend_response`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BackendResponse {
    /// The backend request succeeded.
    Accepted,
    /// The backend request failed or was rejected.
    Throttled,
}

/// Client-side adaptive throttler for RLS requests.
///
/// Tracks accepted and throttled requests over a sliding window using
/// fixed-size bucketed ring buffers (`O(1)` memory). Callers handle
/// synchronization externally.
#[derive(Debug)]
pub(crate) struct AdaptiveThrottler {
    ratio_for_accepts: f64,
    requests_padding: f64,
    accepts: RollingCounter,
    throttles: RollingCounter,
}

impl AdaptiveThrottler {
    /// Creates a new [`AdaptiveThrottler`] with the default RLS parameters
    /// (30s window, 100 bins, 2.0 accept ratio, 8 request padding), anchored
    /// at `now`.
    pub fn new(now: Instant) -> Self {
        Self::with_params(
            DEFAULT_DURATION,
            DEFAULT_BINS,
            DEFAULT_RATIO_FOR_ACCEPTS,
            DEFAULT_REQUESTS_PADDING,
            now,
        )
    }

    fn with_params(
        duration: Duration,
        bins: usize,
        ratio_for_accepts: f64,
        requests_padding: f64,
        origin: Instant,
    ) -> Self {
        Self {
            ratio_for_accepts,
            requests_padding,
            accepts: RollingCounter::new(duration, bins, origin),
            throttles: RollingCounter::new(duration, bins, origin),
        }
    }

    /// Returns whether the next request at `now` should be throttled by the
    /// client before hitting the network.
    ///
    /// When this returns `true`, the throttled request is immediately recorded
    /// in the history window. When it returns `false`, the caller must invoke
    /// [`register_backend_response`](Self::register_backend_response) once the
    /// backend responds.
    pub fn should_throttle(&mut self, now: Instant) -> bool {
        self.should_throttle_with(now, rand::random::<f64>)
    }

    /// Registers a backend `response` received at `now` for a request
    /// previously allowed by [`should_throttle`](Self::should_throttle).
    pub fn register_backend_response(&mut self, now: Instant, response: BackendResponse) {
        match response {
            BackendResponse::Accepted => self.accepts.add(now, 1),
            BackendResponse::Throttled => self.throttles.add(now, 1),
        }
    }

    fn should_throttle_with(
        &mut self,
        now: Instant,
        random_probability: impl FnOnce() -> f64,
    ) -> bool {
        let accepts = self.accepts.sum(now) as f64;
        let throttles = self.throttles.sum(now) as f64;
        let requests = accepts + throttles;
        let throttle_probability =
            (requests - self.ratio_for_accepts * accepts) / (requests + self.requests_padding);
        if throttle_probability <= 0.0 || throttle_probability <= random_probability() {
            return false;
        }
        self.throttles.add(now, 1);
        true
    }
}

/// A rolling counter over a fixed-duration sliding window discretized into a
/// ring buffer of equal-width bins.
#[derive(Debug)]
struct RollingCounter {
    /// Width of each bin in nanoseconds.
    width_nanos: NonZeroU64,
    /// Reference instant corresponding to bin index `0`.
    origin: Instant,
    /// Absolute bin index of the most recent bin observed so far.
    latest_bin: u64,
    /// Running sum of all active bins in `buf`.
    total: u64,
    /// Ring buffer storing per-bin counts.
    buf: Vec<u64>,
}

impl RollingCounter {
    /// Creates a new [`RollingCounter`] covering `duration` split across
    /// `bins` equal-width buckets, anchored at `origin`.
    ///
    /// # Panics
    ///
    /// Panics if `bins == 0`, if the per-bin width (`duration / bins`) is less
    /// than 1 nanosecond, or if the per-bin width exceeds `u64::MAX`
    /// nanoseconds (~584 years).
    fn new(duration: Duration, bins: usize, origin: Instant) -> Self {
        assert!(bins > 0, "rolling counter bins must be non-zero");
        let width_nanos = u64::try_from(duration.as_nanos() / (bins as u128))
            .ok()
            .and_then(NonZeroU64::new)
            .unwrap_or_else(|| {
                panic!(
                    "rolling counter bin width for {duration:?} / {bins} bins must be in 1..=u64::MAX ns"
                )
            });
        Self {
            width_nanos,
            origin,
            latest_bin: 0,
            total: 0,
            buf: vec![0; bins],
        }
    }

    /// Increments the bin corresponding to `now` by `value`.
    ///
    /// Events earlier than `origin` or older than the rolling window behind
    /// `latest_bin` are ignored.
    fn add(&mut self, now: Instant, value: u64) {
        let Some(bin_index) = self.advance(now) else {
            return;
        };
        let bins = self.buf.len() as u64;
        if self.latest_bin - bin_index >= bins {
            return;
        }
        let slot = (bin_index % bins) as usize;
        self.buf[slot] += value;
        self.total += value;
    }

    /// Returns the total across the rolling window ending at
    /// `max(now, latest_bin)`.
    fn sum(&mut self, now: Instant) -> u64 {
        let _ = self.advance(now);
        self.total
    }

    /// Advances `latest_bin` up to the bin index for `now`, expiring any bins
    /// that have fallen out of the window. Returns the bin index for `now`
    /// (which is always `<= self.latest_bin`), or `None` if `now < self.origin`.
    fn advance(&mut self, now: Instant) -> Option<u64> {
        let target_bin = self.bin_index(now)?;
        let previous_latest = self.latest_bin;
        if target_bin <= previous_latest {
            // Either `latest_bin` is unchanged or `now` is behind it.
            return Some(target_bin);
        }

        let bins = self.buf.len() as u64;
        let bins_to_clear = (target_bin - previous_latest).min(bins);
        for offset in 1..=bins_to_clear {
            let slot = ((previous_latest + offset) % bins) as usize;
            self.total -= self.buf[slot];
            self.buf[slot] = 0;
        }
        self.latest_bin = target_bin;
        Some(target_bin)
    }

    /// Maps `now` to a bin index relative to `self.origin`, returning `None`
    /// if `now` is earlier than `self.origin`.
    fn bin_index(&self, now: Instant) -> Option<u64> {
        let elapsed = now.checked_duration_since(self.origin)?;
        let elapsed_nanos = u64::try_from(elapsed.as_nanos()).unwrap_or(u64::MAX);
        Some(elapsed_nanos / self.width_nanos.get())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use BackendResponse::Accepted;
    use BackendResponse::Throttled;

    impl AdaptiveThrottler {
        fn stats(&mut self, now: Instant) -> (u64, u64) {
            (self.accepts.sum(now), self.throttles.sum(now))
        }
    }

    #[test]
    fn rolling_counter_cases() {
        struct TestCase {
            name: &'static str,
            bins: usize,
            ticks_nanos: &'static [u64],
            values: &'static [u64],
            want_sums: &'static [u64],
        }

        let cases = [
            TestCase {
                name: "accumulate",
                bins: 3,
                ticks_nanos: &[0, 1, 2],
                values: &[1, 2, 3],
                want_sums: &[1, 3, 6],
            },
            // Out-of-order tick (0) is still within the 3-bin window behind
            // latest_bin (1), so it is counted.
            TestCase {
                name: "light_time_travel",
                bins: 3,
                ticks_nanos: &[1, 0, 2],
                values: &[1, 2, 3],
                want_sums: &[1, 3, 6],
            },
            // Out-of-order tick (0) is more than 3 bins behind latest_bin (8),
            // so it has fallen outside the rolling window and is ignored.
            TestCase {
                name: "heavy_time_travel",
                bins: 3,
                ticks_nanos: &[8, 0, 9],
                values: &[1, 2, 3],
                want_sums: &[1, 1, 4],
            },
            TestCase {
                name: "rollover",
                bins: 1,
                ticks_nanos: &[0, 1, 2],
                values: &[1, 2, 3],
                want_sums: &[1, 2, 3],
            },
        ];

        for case in cases {
            let base = Instant::now();
            let mut counter =
                RollingCounter::new(Duration::from_nanos(case.bins as u64), case.bins, base);
            for ((&tick, &value), &expected) in
                case.ticks_nanos.iter().zip(case.values).zip(case.want_sums)
            {
                let timestamp = base + Duration::from_nanos(tick);
                counter.add(timestamp, value);
                assert_eq!(
                    counter.sum(timestamp),
                    expected,
                    "case {} at tick {tick}",
                    case.name
                );
            }
        }
    }

    #[test]
    fn rolling_counter_ignores_time_before_origin() {
        let now = Instant::now();
        let origin = now + Duration::from_secs(10);
        let mut counter = RollingCounter::new(Duration::from_secs(3), 3, origin);
        counter.add(origin, 5);
        assert_eq!(counter.sum(origin), 5);

        // Events earlier than `origin` are ignored without affecting the total.
        let one_sec_before = origin - Duration::from_secs(1);
        counter.add(one_sec_before, 9);
        assert_eq!(counter.sum(one_sec_before), 5);
        assert_eq!(counter.sum(origin), 5);
    }

    #[test]
    fn register_backend_response_cases() {
        struct TestCase {
            name: &'static str,
            bins: usize,
            ticks_nanos: &'static [u64],
            responses: &'static [Option<BackendResponse>],
            want_accepts: &'static [u64],
            want_throttled: &'static [u64],
        }

        let cases = [
            TestCase {
                name: "accumulate",
                bins: 3,
                ticks_nanos: &[0, 1, 2],
                responses: &[Some(Accepted), Some(Throttled), None],
                want_accepts: &[1, 1, 1],
                want_throttled: &[0, 1, 1],
            },
            // Out-of-order tick (0) is still within the 3-bin window behind
            // latest_bin (1), so the throttled response is recorded.
            TestCase {
                name: "light_time_travel",
                bins: 3,
                ticks_nanos: &[1, 0, 2],
                responses: &[Some(Accepted), Some(Throttled), None],
                want_accepts: &[1, 1, 1],
                want_throttled: &[0, 1, 1],
            },
            // Out-of-order tick (0) is more than 3 bins behind latest_bin (8),
            // so the second Accept falls outside the window and is ignored.
            TestCase {
                name: "heavy_time_travel",
                bins: 3,
                ticks_nanos: &[8, 0, 9],
                responses: &[Some(Accepted), Some(Accepted), Some(Accepted)],
                want_accepts: &[1, 1, 2],
                want_throttled: &[0, 0, 0],
            },
            TestCase {
                name: "rollover",
                bins: 1,
                ticks_nanos: &[0, 1, 2],
                responses: &[Some(Accepted), Some(Throttled), None],
                want_accepts: &[1, 0, 0],
                want_throttled: &[0, 1, 0],
            },
        ];

        for case in cases {
            let base = Instant::now();
            let mut throttler = AdaptiveThrottler::with_params(
                Duration::from_nanos(case.bins as u64),
                case.bins,
                2.0,
                8.0,
                base,
            );
            for index in 0..case.ticks_nanos.len() {
                let timestamp = base + Duration::from_nanos(case.ticks_nanos[index]);
                if let Some(response) = case.responses[index] {
                    throttler.register_backend_response(timestamp, response);
                }
                assert_eq!(
                    throttler.stats(timestamp),
                    (case.want_accepts[index], case.want_throttled[index]),
                    "case {} at index {index}",
                    case.name
                );
            }
        }
    }

    #[test]
    fn should_throttle_respects_requests_padding() {
        // With 0 accepts, throttle_probability = throttles / (throttles + padding).
        // At random = 0.5, throttling starts once throttles > padding.
        for padding in [7usize, 8] {
            let base = Instant::now();
            let mut throttler = AdaptiveThrottler::with_params(
                Duration::from_secs(1),
                1,
                2.0,
                padding as f64,
                base,
            );

            for count in 1..=padding {
                throttler.register_backend_response(base, Throttled);
                // count / (count + padding) <= 0.5
                assert!(
                    !throttler.should_throttle_with(base, || 0.5),
                    "padding={padding}: should not throttle at {count} failures"
                );
            }
            assert_eq!(throttler.stats(base), (0, padding as u64));

            // One more failure pushes probability to (padding + 1) / (2 * padding + 1) > 0.5.
            throttler.register_backend_response(base, Throttled);
            assert!(
                throttler.should_throttle_with(base, || 0.5),
                "padding={padding}: should throttle at {} failures",
                padding + 1
            );
            // The client-side throttle itself is also recorded.
            assert_eq!(throttler.stats(base), (0, padding as u64 + 2));
        }
    }

    #[test]
    fn should_throttle_respects_ratio_for_accepts() {
        // 2 accepts + 10 throttles = 12 requests; with padding = 8,
        // denominator is 20. Expected probability = (12 - ratio * 2) / 20.
        let cases = [
            // (12 - 2.0 * 2) / 20 = 0.4
            (2.0, 0.4),
            // (12 - 1.0 * 2) / 20 = 0.5
            (1.0, 0.5),
        ];

        for (ratio_for_accepts, expected_prob) in cases {
            let base = Instant::now();
            let mut throttler = AdaptiveThrottler::with_params(
                Duration::from_secs(1),
                1,
                ratio_for_accepts,
                8.0,
                base,
            );
            for _ in 0..2 {
                throttler.register_backend_response(base, Accepted);
            }
            for _ in 0..10 {
                throttler.register_backend_response(base, Throttled);
            }

            // At random == expected_prob, probability <= random -> not throttled.
            assert!(
                !throttler.should_throttle_with(base, || expected_prob),
                "ratio={ratio_for_accepts}: should not throttle at random == {expected_prob}"
            );
            assert_eq!(throttler.stats(base), (2, 10));

            // Slightly below expected_prob -> throttled.
            assert!(
                throttler.should_throttle_with(base, || expected_prob - 0.01),
                "ratio={ratio_for_accepts}: should throttle below {expected_prob}"
            );
            assert_eq!(throttler.stats(base), (2, 11));
        }
    }

    #[test]
    fn should_throttle_records_client_throttles_and_recovers_on_accepts() {
        let base = Instant::now();
        let mut throttler = AdaptiveThrottler::new(base);

        // 10 failures -> probability = 10 / (10 + 8) = 0.555...
        for _ in 0..10 {
            throttler.register_backend_response(base, Throttled);
        }

        // Allowed pick (0.555 <= 0.56) does not mutate counters.
        assert!(!throttler.should_throttle_with(base, || 0.56));
        assert_eq!(throttler.stats(base), (0, 10));

        // Throttled pick (0.555 > 0.5) records a client-side throttle, raising
        // probability to 11 / 19 = 0.578...
        assert!(throttler.should_throttle_with(base, || 0.5));
        assert_eq!(throttler.stats(base), (0, 11));

        // Now 0.56 is below the updated probability (0.578 > 0.56), so it throttles.
        assert!(throttler.should_throttle_with(base, || 0.56));
        assert_eq!(throttler.stats(base), (0, 12));

        // 12 accepts bring (requests - 2 * accepts) = (24 - 24) = 0 -> stops throttling.
        for _ in 0..12 {
            throttler.register_backend_response(base, Accepted);
        }
        assert!(!throttler.should_throttle_with(base, || panic!("rng should not be called")));
        assert_eq!(throttler.stats(base), (12, 12));
    }

    #[test]
    fn default_throttler_never_throttles_when_empty_and_expires_after_window() {
        let base = Instant::now();
        let mut throttler = AdaptiveThrottler::new(base);

        // With zero history, probability is 0.0 so the RNG is not even invoked.
        assert!(!throttler.should_throttle_with(base, || panic!("rng should not be called")));
        assert_eq!(throttler.stats(base), (0, 0));

        // Healthy traffic (probability < 0.0) also skips the RNG.
        throttler.register_backend_response(base, Accepted);
        assert!(!throttler.should_throttle_with(base, || panic!("rng should not be called")));

        // Flood with failures so throttling kicks in:
        // (101 requests - 2 * 1 accept) / (101 + 8) = 99 / 109 > 0.9.
        for _ in 0..100 {
            throttler.register_backend_response(base, Throttled);
        }
        assert!(throttler.should_throttle_with(base, || 0.5));
        // Client-side throttle increments throttled count.
        assert_eq!(throttler.stats(base), (1, 101));

        // After the 30s window elapses, history expires and throttling stops.
        let after_window = base + DEFAULT_DURATION;
        assert!(
            !throttler.should_throttle_with(after_window, || panic!("rng should not be called"))
        );
        assert_eq!(throttler.stats(after_window), (0, 0));
    }

    #[test]
    fn should_throttle_smoke_with_real_rng() {
        let base = Instant::now();
        let mut throttler = AdaptiveThrottler::new(base);
        // 10 failures -> probability = 10 / 18 = 0.555..., so the real RNG is
        // consulted on every call. Each throttle raises the probability to
        // n / (n + 8), staying strictly below 1, so over 200 draws both
        // outcomes are expected (the chance of either never happening is
        // below 1e-9).
        for _ in 0..10 {
            throttler.register_backend_response(base, Throttled);
        }
        let mut throttled = 0;
        let mut allowed = 0;
        for _ in 0..200 {
            if throttler.should_throttle(base) {
                throttled += 1;
            } else {
                allowed += 1;
            }
        }
        assert!(throttled > 0, "expected some requests to be throttled");
        assert!(allowed > 0, "expected some requests to be allowed");
        // Only client-side throttles are recorded; allowed picks leave counters alone.
        assert_eq!(throttler.stats(base), (0, 10 + throttled));
    }
}
