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

//! Retry policy configuration based on gRFC A6.

use std::time::Duration;

use rand::RngExt;

use crate::error::{Error, Result};

/// Retry policy for xDS client connection attempts.
///
/// This configuration follows the gRFC A6 proposal for client retries,
/// using exponential backoff with jitter for reconnection attempts.
///
/// # Example
///
/// ```
/// use xds_client::RetryPolicy;
/// use std::time::Duration;
///
/// let policy = RetryPolicy::default()
///     .with_initial_backoff(Duration::from_secs(1)).unwrap()
///     .with_max_backoff(Duration::from_secs(30)).unwrap()
///     .with_backoff_multiplier(2.0).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Initial backoff duration for the first retry attempt.
    ///
    /// Default: 1 second.
    initial_backoff: Duration,

    /// Maximum backoff duration.
    ///
    /// The backoff will not grow beyond this value, regardless of how many
    /// retry attempts have been made.
    ///
    /// Default: 30 seconds.
    max_backoff: Duration,

    /// Multiplier for exponential backoff.
    ///
    /// After each failed attempt, the current backoff duration is multiplied
    /// by this value (up to `max_backoff`).
    ///
    /// Default: 2.0 (exponential backoff).
    backoff_multiplier: f64,

    /// Maximum number of retry attempts.
    ///
    /// If `None`, retries indefinitely. If `Some(n)`, stops after `n` attempts.
    ///
    /// Default: None (infinite retries).
    max_attempts: Option<usize>,

    /// The factor with which backoffs are randomized.
    jitter: f64,
}

impl RetryPolicy {
    /// Create a new retry policy with custom parameters.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - `backoff_multiplier` is less than 1.0
    /// - `max_backoff` is less than `initial_backoff`
    /// - `initial_backoff` is zero
    ///
    /// # Example
    ///
    /// ```
    /// use xds_client::RetryPolicy;
    /// use std::time::Duration;
    ///
    /// let policy = RetryPolicy::new(
    ///     Duration::from_millis(500),  // initial_backoff
    ///     Duration::from_secs(60),     // max_backoff
    ///     1.5,                         // backoff_multiplier
    /// )?;
    /// # Ok::<(), xds_client::Error>(())
    /// ```
    pub fn new(
        initial_backoff: Duration,
        max_backoff: Duration,
        backoff_multiplier: f64,
    ) -> Result<Self> {
        if initial_backoff.is_zero() {
            return Err(Error::Validation(
                "initial_backoff must be greater than zero".into(),
            ));
        }

        if backoff_multiplier < 1.0 {
            return Err(Error::Validation(format!(
                "backoff_multiplier must be >= 1.0, got {backoff_multiplier}"
            )));
        }

        if max_backoff < initial_backoff {
            return Err(Error::Validation(format!(
                "max_backoff ({max_backoff:?}) must be >= initial_backoff ({initial_backoff:?})"
            )));
        }

        Ok(Self {
            initial_backoff,
            max_backoff,
            backoff_multiplier,
            ..Default::default()
        })
    }

    /// Set the initial backoff duration.
    ///
    /// # Errors
    ///
    /// Returns an error if `duration` is zero or greater than `max_backoff`.
    pub fn with_initial_backoff(mut self, duration: Duration) -> Result<Self> {
        if duration.is_zero() {
            return Err(Error::Validation(
                "initial_backoff must be greater than zero".into(),
            ));
        }
        if duration > self.max_backoff {
            let max_backoff = self.max_backoff;
            return Err(Error::Validation(format!(
                "initial_backoff ({duration:?}) must be <= max_backoff ({max_backoff:?})"
            )));
        }
        self.initial_backoff = duration;
        Ok(self)
    }

    /// Set the maximum backoff duration.
    ///
    /// # Errors
    ///
    /// Returns an error if `duration` is less than `initial_backoff`.
    pub fn with_max_backoff(mut self, duration: Duration) -> Result<Self> {
        if duration < self.initial_backoff {
            let initial_backoff = self.initial_backoff;
            return Err(Error::Validation(format!(
                "max_backoff ({duration:?}) must be >= initial_backoff ({initial_backoff:?})"
            )));
        }
        self.max_backoff = duration;
        Ok(self)
    }

    /// Set the backoff multiplier.
    ///
    /// # Errors
    ///
    /// Returns an error if `multiplier` is less than 1.0.
    pub fn with_backoff_multiplier(mut self, multiplier: f64) -> Result<Self> {
        if multiplier < 1.0 {
            return Err(Error::Validation(format!(
                "backoff_multiplier must be >= 1.0, got {multiplier}"
            )));
        }
        self.backoff_multiplier = multiplier;
        Ok(self)
    }

    /// Set the jitter factor applied to each backoff delay (gRFC A6).
    ///
    /// `0.0` disables jitter; the default `0.2` randomizes each delay by ±20%.
    ///
    /// # Errors
    ///
    /// Returns an error if `jitter` is not in the range `[0.0, 1.0]`.
    pub fn with_jitter(mut self, jitter: f64) -> Result<Self> {
        if !(0.0..=1.0).contains(&jitter) {
            return Err(Error::Validation(format!(
                "jitter must be between 0.0 and 1.0, got {jitter}"
            )));
        }
        self.jitter = jitter;
        Ok(self)
    }

    /// Set the maximum number of retry attempts.
    ///
    /// If set to `None`, retries indefinitely.
    pub fn with_max_attempts(mut self, max_attempts: Option<usize>) -> Self {
        self.max_attempts = max_attempts;
        self
    }

    /// Calculate the backoff duration for a given attempt number.
    ///
    /// Returns `None` if `max_attempts` is set and the attempt exceeds it.
    ///
    /// # Arguments
    ///
    /// * `attempt` - The retry attempt number (0-indexed).
    ///
    /// # Example
    ///
    /// ```
    /// use xds_client::RetryPolicy;
    /// use std::time::Duration;
    ///
    /// let policy = RetryPolicy::default();
    /// assert_eq!(policy.backoff_duration(0), Some(Duration::from_secs(1)));
    /// assert_eq!(policy.backoff_duration(1), Some(Duration::from_secs(2)));
    /// assert_eq!(policy.backoff_duration(2), Some(Duration::from_secs(4)));
    /// ```
    pub fn backoff_duration(&self, attempt: usize) -> Option<Duration> {
        // Check if we've exceeded max attempts
        if let Some(max) = self.max_attempts
            && attempt >= max
        {
            return None;
        }

        // Calculate exponential backoff (saturate to i32::MAX to avoid overflow in powi)
        let exponent = i32::try_from(attempt).unwrap_or(i32::MAX);
        let multiplier = self.backoff_multiplier.powi(exponent);
        let backoff = self.initial_backoff.mul_f64(multiplier);

        // Cap at max_backoff
        Some(backoff.min(self.max_backoff))
    }
}

impl Default for RetryPolicy {
    /// Create a retry policy with default values based on gRFC A6.
    ///
    /// Defaults:
    /// - `initial_backoff`: 1 second
    /// - `max_backoff`: 30 seconds
    /// - `backoff_multiplier`: 2.0
    /// - `max_attempts`: None (infinite retries)
    /// - `jitter`: 0.2
    fn default() -> Self {
        Self {
            initial_backoff: Duration::from_secs(1),
            max_backoff: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            max_attempts: None,
            jitter: 0.2,
        }
    }
}

/// Stateful backoff calculator based on a [`RetryPolicy`].
///
/// This struct tracks the current attempt number and provides methods to
/// get the next backoff duration and reset after successful operations.
///
/// # Example
///
/// ```
/// use xds_client::{Backoff, RetryPolicy};
/// use std::time::Duration;
///
/// let mut backoff = Backoff::new(RetryPolicy::default());
///
/// // Each backoff is the exponential base (1s, 2s, ...) randomized by ±20%
/// // jitter, so it lands within [0.8x, 1.2x) of the base.
/// let first = backoff.next_backoff().unwrap();
/// assert!(first >= Duration::from_millis(800) && first < Duration::from_millis(1200));
///
/// // Second failure: the base doubles to 2s (again jittered ±20%).
/// let second = backoff.next_backoff().unwrap();
/// assert!(second >= Duration::from_millis(1600) && second < Duration::from_millis(2400));
///
/// // Success: reset for the next failure sequence (base returns to 1s).
/// backoff.reset();
/// let after_reset = backoff.next_backoff().unwrap();
/// assert!(after_reset >= Duration::from_millis(800) && after_reset < Duration::from_millis(1200));
/// ```
#[derive(Debug, Clone)]
pub struct Backoff {
    policy: RetryPolicy,
    attempt: usize,
}

impl Backoff {
    /// Create a new backoff calculator from a retry policy.
    pub fn new(policy: RetryPolicy) -> Self {
        Self { policy, attempt: 0 }
    }

    /// Get the next backoff duration and advance the attempt counter.
    ///
    /// Returns `None` if `max_attempts` is set and has been exceeded.
    pub fn next_backoff(&mut self) -> Option<Duration> {
        let base = self.policy.backoff_duration(self.attempt)?;
        self.attempt += 1;
        let factor = 1.0 + self.policy.jitter * rand::rng().random_range(-1.0..1.0);
        Some(base.mul_f64(factor))
    }

    /// Reset the backoff after a successful operation.
    ///
    /// This resets the attempt counter to 0, so the next failure will
    /// use the initial backoff duration.
    pub fn reset(&mut self) {
        self.attempt = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Epsilon for the float-derived duration bounds.
    const EPSILON: f64 = 1e-9;

    /// Each backoff is the exponential base randomized by ±jitter (0.2), so it
    /// falls within `[base * 0.8, base * 1.2]` for the 1s/2s/4s schedule.
    #[test]
    fn next_backoff_applies_bounded_jitter() {
        let mut backoff = Backoff::new(RetryPolicy::default()); // jitter 0.2, base 1s, mult 2
        let msg = "next_backoff is Some while max_attempts is None";

        // base 1s -> [0.8s, 1.2s]
        let d = backoff.next_backoff().expect(msg);
        assert!(d > Duration::from_secs_f64(0.8 - EPSILON));
        assert!(d < Duration::from_secs_f64(1.2 + EPSILON));
        // base 2s -> [1.6s, 2.4s]
        let d = backoff.next_backoff().expect(msg);
        assert!(d > Duration::from_secs_f64(1.6 - EPSILON));
        assert!(d < Duration::from_secs_f64(2.4 + EPSILON));
        // base 4s -> [3.2s, 4.8s]
        let d = backoff.next_backoff().expect(msg);
        assert!(d > Duration::from_secs_f64(3.2 - EPSILON));
        assert!(d < Duration::from_secs_f64(4.8 + EPSILON));
    }

    /// With jitter disabled the backoff is fully deterministic: it doubles each
    /// attempt, caps at `max_backoff`, and returns to the base after `reset`.
    #[test]
    fn backoff_reset_no_jitter() {
        let policy = RetryPolicy::default()
            .with_jitter(0.0)
            .expect("0.0 should disabled jitter on retry policy");
        let msg = "next_backoff is Some while max_attempts is None";
        let mut backoff = Backoff::new(policy);

        assert_eq!(backoff.next_backoff().expect(msg), Duration::from_secs(1));
        assert_eq!(backoff.next_backoff().expect(msg), Duration::from_secs(2));
        assert_eq!(backoff.next_backoff().expect(msg), Duration::from_secs(4));
        assert_eq!(backoff.next_backoff().expect(msg), Duration::from_secs(8));
        assert_eq!(backoff.next_backoff().expect(msg), Duration::from_secs(16));
        // Capped at max_backoff (32s -> 30s).
        assert_eq!(backoff.next_backoff().expect(msg), Duration::from_secs(30));
        assert_eq!(backoff.next_backoff().expect(msg), Duration::from_secs(30));

        backoff.reset();
        assert_eq!(backoff.next_backoff().expect(msg), Duration::from_secs(1));
    }

    /// Jitter must be in `[0.0, 1.0]`; out-of-range values are rejected and a
    /// valid value is stored verbatim.
    #[test]
    fn jitter_validation() {
        for j in [-0.1, 1.5] {
            assert!(RetryPolicy::default().with_jitter(j).is_err());
        }
        let policy = RetryPolicy::default()
            .with_jitter(0.5)
            .expect("0.5 is a valid jitter");
        assert_eq!(policy.jitter, 0.5);
    }
}
