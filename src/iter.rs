/*
 * // Copyright (c) Radzivon Bartoshyk 3/2026. All rights reserved.
 * //
 * // Redistribution and use in source and binary forms, with or without modification,
 * // are permitted provided that the following conditions are met:
 * //
 * // 1.  Redistributions of source code must retain the above copyright notice, this
 * // list of conditions and the following disclaimer.
 * //
 * // 2.  Redistributions in binary form must reproduce the above copyright notice,
 * // this list of conditions and the following disclaimer in the documentation
 * // and/or other materials provided with the distribution.
 * //
 * // 3.  Neither the name of the copyright holder nor the names of its
 * // contributors may be used to endorse or promote products derived from
 * // this software without specific prior written permission.
 * //
 * // THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * // AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * // IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * // DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
 * // FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * // DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * // SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * // CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * // OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * // OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
use crate::HampelFilter;
use num_traits::AsPrimitive;
use num_traits::float::FloatCore;

pub struct HampelIter<T, I, const W: usize>
where
    T: FloatCore + Default + 'static,
    I: Iterator<Item = T>,
    f64: AsPrimitive<T>,
{
    inner: I,
    filter: HampelFilter<T, W>,
}

impl<T, I, const W: usize> Iterator for HampelIter<T, I, W>
where
    T: FloatCore + Default + 'static,
    I: Iterator<Item = T>,
    f64: AsPrimitive<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.inner.next().map(|x| self.filter.update(x))
    }
}

/// Extension trait — lets you call `.hampel::<7>(3.0)` on any float iterator
pub trait HampelExt<T>: Iterator<Item = T> + Sized
where
    T: FloatCore + Default + 'static,
    f64: AsPrimitive<T>,
{
    fn hampel<const W: usize>(self, n_sigma: T) -> HampelIter<T, Self, W> {
        HampelIter {
            inner: self,
            filter: HampelFilter::new(n_sigma),
        }
    }
}

// Blanket impl for all compatible iterators
impl<T, I> HampelExt<T> for I
where
    T: FloatCore + Default + 'static,
    I: Iterator<Item = T>,
    f64: AsPrimitive<T>,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    fn signal() -> Vec<f32> {
        vec![1.0, 1.1, 1.0, 1.2, 1.1, 1.0, 1.1, 1.2, 1.0, 1.1]
    }

    // --- basic passthrough ---

    #[test]
    fn test_clean_signal_unchanged() {
        // clean signal should pass through mostly untouched
        let input = signal();
        let output: Vec<f32> = input.iter().copied().hampel::<3>(3.0).collect();
        assert_eq!(input.len(), output.len());
        for (i, o) in input.iter().zip(output.iter()) {
            assert!((i - o).abs() < 0.5, "clean sample was altered: {i} -> {o}");
        }
    }

    #[test]
    fn test_output_length_matches_input() {
        let input = signal();
        let output: Vec<f32> = input.iter().copied().hampel::<5>(3.0).collect();
        assert_eq!(input.len(), output.len());
    }

    // --- outlier replacement ---

    #[test]
    fn test_single_spike_replaced() {
        let mut input = signal();
        input[5] = 999.0; // obvious spike
        let output: Vec<f32> = input.iter().copied().hampel::<7>(3.0).collect();
        assert!(
            output[5] < 10.0,
            "spike at index 5 was not replaced: {}",
            output[5]
        );
    }

    #[test]
    fn test_spike_replaced_with_approx_median() {
        let input = vec![1.0f32, 1.0, 1.0, 999.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        let output: Vec<f32> = input.iter().copied().hampel::<7>(3.0).collect();
        // after window fills, the spike should be pulled toward 1.0
        let spike_out = output[7]; // spike influence felt once window is full
        assert!(spike_out < 10.0, "spike not suppressed: {spike_out}");
    }

    #[test]
    fn test_negative_spike_replaced() {
        let mut input = signal();
        input[4] = -999.0;
        let output: Vec<f32> = input.iter().copied().hampel::<7>(3.0).collect();
        assert!(
            output[4] > -10.0,
            "negative spike not replaced: {}",
            output[4]
        );
    }

    // --- warmup behaviour ---

    #[test]
    fn test_warmup_passthrough() {
        // first WINDOW_SIZE-1 samples must pass through unmodified
        let input = vec![1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let output: Vec<f32> = input.iter().copied().hampel::<7>(3.0).collect();
        for i in 0..6 {
            assert_eq!(input[i], output[i], "warmup sample {i} was modified");
        }
    }

    // --- sigma sensitivity ---

    #[test]
    fn test_tight_sigma_catches_more() {
        let mut input = signal();
        input[5] = 5.0; // mild outlier
        let strict: Vec<f32> = input.iter().copied().hampel::<7>(1.0).collect();
        let lenient: Vec<f32> = input.iter().copied().hampel::<7>(5.0).collect();
        // strict filter should deviate more from input than lenient
        let strict_diff: f32 = strict
            .iter()
            .zip(input.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();
        let lenient_diff: f32 = lenient
            .iter()
            .zip(input.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();
        assert!(
            strict_diff >= lenient_diff,
            "strict sigma should catch more outliers"
        );
    }

    // --- window sizes ---

    #[test]
    fn test_window_3() {
        let mut input = signal();
        input[4] = 999.0;
        let output: Vec<f32> = input.iter().copied().hampel::<3>(3.0).collect();
        assert_eq!(input.len(), output.len());
    }

    #[test]
    fn test_window_5() {
        let mut input = signal();
        input[4] = 999.0;
        let output: Vec<f32> = input.iter().copied().hampel::<5>(3.0).collect();
        assert_eq!(input.len(), output.len());
    }

    #[test]
    fn test_window_7() {
        let mut input = signal();
        input[4] = 999.0;
        let output: Vec<f32> = input.iter().copied().hampel::<7>(3.0).collect();
        assert_eq!(input.len(), output.len());
    }

    // --- edge cases ---

    #[test]
    fn test_empty_input() {
        let input: Vec<f32> = vec![];
        let output: Vec<f32> = input.iter().copied().hampel::<7>(3.0).collect();
        assert!(output.is_empty());
    }

    #[test]
    fn test_shorter_than_window() {
        // fewer samples than window — all should pass through as warmup
        let input = vec![1.0f32, 2.0, 3.0];
        let output: Vec<f32> = input.iter().copied().hampel::<7>(3.0).collect();
        assert_eq!(input, output);
    }

    #[test]
    fn test_constant_signal() {
        let input = vec![5.0f32; 20];
        let output: Vec<f32> = input.iter().copied().hampel::<7>(3.0).collect();
        // constant signal — MAD is zero, nothing should be replaced
        assert_eq!(input, output);
    }

    #[test]
    fn test_chained_with_map() {
        // verify it composes cleanly in iterator pipelines
        let input = signal();
        let output: Vec<f32> = input
            .iter()
            .copied()
            .hampel::<5>(3.0)
            .map(|x| x * 2.0)
            .collect();
        assert_eq!(output.len(), input.len());
    }

    #[test]
    fn test_f64() {
        let input = vec![1.0f64, 1.1, 1.0, 999.0, 1.0, 1.1, 1.0, 1.0, 1.0, 1.0];
        let output: Vec<f64> = input.iter().copied().hampel::<7>(3.0).collect();
        assert_eq!(input.len(), output.len());
    }
}
