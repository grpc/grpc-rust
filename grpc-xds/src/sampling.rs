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

//! Weighted random sampling.

use std::fmt::Debug;
use std::sync::Arc;

/// Produces the random keys [`WeightedSampler`] uses to select an item. Tests
/// supply fixed keys to make selection deterministic.
trait Rng: Send + Sync + Debug {
    /// Generates a uniform random value in the half-open range `[0, total_weight)`.
    fn sample(&self, total_weight: u64) -> u64;
}

#[derive(Debug)]
struct DefaultRng;

impl Rng for DefaultRng {
    fn sample(&self, total_weight: u64) -> u64 {
        rand::random_range(0..total_weight)
    }
}

/// Constructs [`WeightedSampler`] instances from `(item, weight)` pairs.
#[derive(Clone, Debug)]
pub(crate) struct WeightedSamplerBuilder {
    rng: Arc<dyn Rng>,
}

impl Default for WeightedSamplerBuilder {
    fn default() -> Self {
        Self {
            rng: Arc::new(DefaultRng),
        }
    }
}

impl WeightedSamplerBuilder {
    pub(crate) fn build<T>(&self, items: impl IntoIterator<Item = (T, u32)>) -> WeightedSampler<T> {
        let mut cumulative_weight = 0u64;
        let items = items
            .into_iter()
            .filter(|(_, weight)| *weight > 0)
            .map(|(item, weight)| {
                cumulative_weight += u64::from(weight);
                (cumulative_weight, item)
            })
            .collect();
        WeightedSampler {
            items,
            rng: self.rng.clone(),
        }
    }
}

/// Weighted random item sampler.
///
/// Stores `(cumulative_weight, item)` pairs in ascending order of cumulative
/// weight and selects items in proportion to their weight by binary-searching
/// for a uniform sample in `[0, total_weight)`.
#[derive(Debug)]
pub(crate) struct WeightedSampler<T> {
    items: Vec<(u64, T)>,
    rng: Arc<dyn Rng>,
}

impl<T> WeightedSampler<T> {
    /// Selects an item from the set, or returns `None` if no items have a
    /// positive weight.
    pub(crate) fn next(&self) -> Option<&T> {
        let &(total_weight, _) = self.items.last()?;
        let key = self.rng.sample(total_weight);
        let index = self.items.partition_point(|(w, _)| *w <= key);
        Some(&self.items[index].1)
    }
}

#[cfg(test)]
pub(crate) mod test_utils {
    use std::collections::VecDeque;
    use std::sync::Mutex;

    use super::*;

    /// Returns a [`WeightedSamplerBuilder`] whose RNG yields `keys` in
    /// order across all [`WeightedSampler`] instances created by the builder.
    pub(crate) fn sampler_builder(keys: &[u64]) -> WeightedSamplerBuilder {
        WeightedSamplerBuilder {
            rng: Arc::new(MockRng(Mutex::new(keys.to_vec().into()))),
        }
    }

    /// Returns the given keys in order. Panics when the keys run out or a key
    /// is not below the total weight, so no pick depends on an unplanned key.
    #[derive(Debug)]
    struct MockRng(Mutex<VecDeque<u64>>);

    impl Rng for MockRng {
        fn sample(&self, total_weight: u64) -> u64 {
            let key = self.0.lock().unwrap().pop_front();
            let key = key.expect("test should provide a key for every weighted pick");
            assert!(
                key < total_weight,
                "key {key} should be below total weight {total_weight}"
            );
            key
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::sampler_builder;
    use super::*;

    #[test]
    fn weighted_sampler_partitions_weights_correctly() {
        let sampler =
            sampler_builder(&[0, 19, 20, 49, 50, 99]).build([("a", 20), ("b", 30), ("c", 50)]);
        for expected in ["a", "a", "b", "b", "c", "c"] {
            assert_eq!(sampler.next(), Some(&expected));
        }

        let default_sampler = WeightedSamplerBuilder::default().build([("a", 50)]);
        assert_eq!(default_sampler.next(), Some(&"a"));
    }

    #[test]
    fn weighted_sampler_skips_zero_weight_and_handles_empty() {
        let empty: WeightedSampler<&str> = WeightedSamplerBuilder::default().build([]);
        assert_eq!(empty.next(), None);

        let all_zero = WeightedSamplerBuilder::default().build([("a", 0), ("b", 0)]);
        assert_eq!(all_zero.next(), None);

        let with_zero = sampler_builder(&[0]).build([("a", 0), ("b", 10), ("c", 0)]);
        assert_eq!(with_zero.next(), Some(&"b"));
    }
}
