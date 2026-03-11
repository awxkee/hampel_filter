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

#[inline(always)]
pub(crate) fn median3<T: PartialOrd>(a: T, b: T, c: T) -> T {
    if a < b {
        if b < c {
            b
        } else if a < c {
            c
        } else {
            a
        }
    } else {
        if a < c {
            a
        } else if b < c {
            c
        } else {
            b
        }
    }
}

#[inline(always)]
pub(crate) fn median4<T: PartialOrd>(a: T, b: T, c: T, d: T) -> T {
    // returns lower median (2nd of 4 sorted)
    let (lo1, hi1) = if a < b { (a, b) } else { (b, a) };
    let (lo2, hi2) = if c < d { (c, d) } else { (d, c) };
    let low = if lo1 < lo2 { lo2 } else { lo1 }; // max of lows
    let high = if hi1 < hi2 { hi1 } else { hi2 }; // min of highs
    if low < high { low } else { high }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- median3 ---
    #[test]
    fn test_median3_middle_first() {
        assert_eq!(median3(2.0f32, 1.0, 3.0), 2.0);
    }

    #[test]
    fn test_median3_all_permutations() {
        let vals = [1.0f32, 2.0, 3.0];
        let expected = 2.0f32;
        // all 6 permutations
        assert_eq!(median3(vals[0], vals[1], vals[2]), expected);
        assert_eq!(median3(vals[0], vals[2], vals[1]), expected);
        assert_eq!(median3(vals[1], vals[0], vals[2]), expected);
        assert_eq!(median3(vals[1], vals[2], vals[0]), expected);
        assert_eq!(median3(vals[2], vals[0], vals[1]), expected);
        assert_eq!(median3(vals[2], vals[1], vals[0]), expected);
    }

    #[test]
    fn test_median3_duplicates() {
        assert_eq!(median3(2.0f32, 2.0, 1.0), 2.0);
        assert_eq!(median3(1.0f32, 1.0, 1.0), 1.0);
    }

    // --- median4 ---
    #[test]
    fn test_median4_ordered() {
        assert_eq!(median4(1.0f32, 2.0, 3.0, 4.0), 2.0); // lower median
    }

    #[test]
    fn test_median4_reversed() {
        assert_eq!(median4(4.0f32, 3.0, 2.0, 1.0), 2.0);
    }

    #[test]
    fn test_median4_all_same() {
        assert_eq!(median4(7.0f32, 7.0, 7.0, 7.0), 7.0);
    }

    #[test]
    fn test_median4_duplicates() {
        assert_eq!(median4(1.0f32, 3.0, 3.0, 5.0), 3.0);
    }

    #[test]
    fn test_median4_mixed() {
        assert_eq!(median4(5.0f32, 1.0, 4.0, 2.0), 2.0);
    }
}
