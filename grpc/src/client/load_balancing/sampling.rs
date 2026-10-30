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
    /// Generates a uniform random value in the half-open range `[0, upper_bound)`.
    fn sample(&self, upper_bound: u64) -> u64;
}

#[derive(Debug)]
struct DefaultRng;

impl Rng for DefaultRng {
    fn sample(&self, upper_bound: u64) -> u64 {
        rand::random_range(0..upper_bound)
    }
}

/// Constructs [`WeightedSampler`] instances from `(item, weight)` pairs.
#[derive(Clone, Debug)]
pub struct WeightedSamplerBuilder {
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
    /// Returns a sampler for `items`, which are `(item, weight)` pairs.
    /// Items with a weight of zero are never selected.
    pub fn build<T>(&self, items: impl IntoIterator<Item = (T, u32)>) -> WeightedSampler<T> {
        let mut cumulative_weight = 0u64;
        let mut first_weight = None;
        let mut equal_weights = true;
        let items = items
            .into_iter()
            .filter(|(_, weight)| *weight > 0)
            .map(|(item, weight)| {
                if *first_weight.get_or_insert(weight) != weight {
                    equal_weights = false;
                }
                cumulative_weight += u64::from(weight);
                (cumulative_weight, item)
            })
            .collect();
        WeightedSampler {
            items,
            equal_weights,
            rng: self.rng.clone(),
        }
    }
}

/// Samples items at random in proportion to their configured weights.
///
/// Construct an instance with [`WeightedSamplerBuilder::build`] (which filters
/// out zero-weight items) and call [`WeightedSampler::next`] to draw items.
#[derive(Debug)]
pub struct WeightedSampler<T> {
    // `(cumulative_weight, item)` pairs in ascending order of cumulative weight.
    items: Vec<(u64, T)>,
    equal_weights: bool,
    rng: Arc<dyn Rng>,
}

impl<T> WeightedSampler<T> {
    /// Selects an item from the set, or returns `None` if no items have a
    /// positive weight.
    pub fn next(&self) -> Option<&T> {
        // Single item — return directly without invoking the RNG.
        if let [(_, item)] = self.items.as_slice() {
            return Some(item);
        }
        let &(total_weight, _) = self.items.last()?;
        // Equal weights — sample a uniform index in `0..items.len()`.
        if self.equal_weights {
            let index = self.rng.sample(self.items.len() as u64) as usize;
            return Some(&self.items[index].1);
        }
        // Distinct weights — binary-search cumulative weights for a
        // uniform sample in `[0, total_weight)`.
        let key = self.rng.sample(total_weight);
        let index = self.items.partition_point(|(w, _)| *w <= key);
        Some(&self.items[index].1)
    }
}

#[cfg(test)]
pub(super) mod test_utils {
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
    /// is not below `upper_bound`, so no pick depends on an unplanned key.
    #[derive(Debug)]
    struct MockRng(Mutex<VecDeque<u64>>);

    impl Rng for MockRng {
        fn sample(&self, upper_bound: u64) -> u64 {
            let key = self.0.lock().unwrap().pop_front();
            let key = key.expect("test should provide a key for every weighted pick");
            assert!(
                key < upper_bound,
                "key {key} should be below upper bound {upper_bound}"
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

        let default_sampler = WeightedSamplerBuilder::default().build([("a", 50), ("b", 30)]);
        assert!(default_sampler.next().is_some());
    }

    #[test]
    fn weighted_sampler_fast_paths_single_item_and_equal_weights() {
        // Single item does not invoke the RNG (no keys provided).
        let single = sampler_builder(&[]).build([("a", 50)]);
        assert_eq!(single.next(), Some(&"a"));

        // Equal weights sample directly from `0..items.len()`.
        let equal = sampler_builder(&[0, 1, 2]).build([("a", 50), ("b", 50), ("c", 50)]);
        for expected in ["a", "b", "c"] {
            assert_eq!(equal.next(), Some(&expected));
        }
    }

    #[test]
    fn weighted_sampler_skips_zero_weight_and_handles_empty() {
        let empty: WeightedSampler<&str> = WeightedSamplerBuilder::default().build([]);
        assert_eq!(empty.next(), None);

        let all_zero = WeightedSamplerBuilder::default().build([("a", 0), ("b", 0)]);
        assert_eq!(all_zero.next(), None);

        let with_zero = sampler_builder(&[0, 10]).build([("a", 0), ("b", 10), ("c", 0), ("d", 20)]);
        assert_eq!(with_zero.next(), Some(&"b"));
        assert_eq!(with_zero.next(), Some(&"d"));
    }
}
