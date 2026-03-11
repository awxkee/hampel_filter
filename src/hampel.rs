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
use crate::median::{median3, median4};
use crate::quick_select::quick_select;
use num_traits::AsPrimitive;
use num_traits::float::FloatCore;

/// Hampel filter for outlier detection in a sliding window.
/// Replaces outliers with the window median based on MAD (Median Absolute Deviation).
///
/// `WINDOW_SIZE` should be odd — even sizes have no true median (lower median is used).
/// Typical values: 3, 5, 7.
pub struct HampelFilter<T: FloatCore, const WINDOW_SIZE: usize> {
    /// Circular buffer holding the last WINDOW_SIZE samples
    window: [T; WINDOW_SIZE],
    /// Scratch space for median computation — avoids stack allocation on every update
    working_array: [T; WINDOW_SIZE],
    /// Index of the oldest sample (next write position)
    oldest: usize,
    /// Precomputed: 1.4826 * n_sigma (1.4826 links MAD to std deviation for Gaussian data)
    coef: T,
    /// Tracks whether the window has been fully populated
    filled: usize,
}

impl<T: FloatCore + Default + 'static, const WINDOW_SIZE: usize> HampelFilter<T, WINDOW_SIZE>
where
    f64: AsPrimitive<T>,
{
    pub fn new(n_sigma: T) -> Self {
        assert!(WINDOW_SIZE >= 3, "WINDOW_SIZE must be at least 3");

        Self {
            window: [T::default(); WINDOW_SIZE],
            working_array: [T::default(); WINDOW_SIZE],
            oldest: 0,
            coef: 1.4826f64.as_() * n_sigma,
            filled: 0,
        }
    }

    #[inline]
    pub fn update(&mut self, x: T) -> T {
        // Write new sample into circular buffer at oldest position
        // SAFETY: oldest is always in [0, WINDOW_SIZE) by construction
        unsafe { *self.window.get_unchecked_mut(self.oldest) = x };
        self.oldest = (self.oldest + 1) % WINDOW_SIZE;

        // Pass through unchanged until window is fully populated —
        // partial windows produce unreliable medians
        if self.filled < WINDOW_SIZE {
            self.filled += 1;
            // fill remaining slots with this sample
            for i in self.filled..WINDOW_SIZE {
                unsafe {
                    *self
                        .window
                        .get_unchecked_mut((self.oldest + i) % WINDOW_SIZE) = x
                };
            }
        }

        self.working_array = self.window;
        let w0 = self.get_median();
        for w in self.working_array.iter_mut() {
            *w = (*w - w0).abs();
        }
        let s0 = self.get_median();

        //
        if (x - w0).abs() <= self.coef * s0 {
            x
        } else {
            w0
        }
    }

    fn get_median(&mut self) -> T {
        if WINDOW_SIZE == 3 {
            median3(
                self.working_array[0],
                self.working_array[1],
                self.working_array[2],
            )
        } else if WINDOW_SIZE == 4 {
            median4(
                self.working_array[0],
                self.working_array[1],
                self.working_array[2],
                self.working_array[3],
            )
        } else {
            quick_select(&mut self.working_array)
        }
    }
}
