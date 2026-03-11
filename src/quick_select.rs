#[inline(always)]
fn swap<T: Copy>(arr: &mut [T], a: usize, b: usize) {
    unsafe {
        // Can't take two mutable loans from one vector, so instead just cast
        // them to their raw pointers to do the swap
        let pa: *mut T = arr.get_unchecked_mut(a);
        let pb: *mut T = arr.get_unchecked_mut(b);
        std::ptr::swap(pa, pb);
    }
}

pub(crate) fn quick_select<T: Copy + PartialEq + PartialOrd>(arr: &mut [T]) -> T {
    let (mut low, mut high);
    let (mut middle, mut ll, mut hh);
    low = 0;
    let n = arr.len();
    high = n - 1;
    let median = (low + high) / 2;
    loop {
        if high <= low {
            return unsafe { *arr.get_unchecked(median) };
        }
        if high == low + 1 {
            /* Two elements only */
            unsafe {
                if *arr.get_unchecked(low) > *arr.get_unchecked(high) {
                    swap(arr, low, high)
                }
            }
            return unsafe { *arr.get_unchecked(median) };
        }
        /* Find median of low, middle and high items; swap into position low */
        middle = (low + high) / 2;
        unsafe {
            if *arr.get_unchecked(middle) > *arr.get_unchecked(high) {
                swap(arr, middle, high);
            }
            if *arr.get_unchecked(low) > *arr.get_unchecked(high) {
                swap(arr, low, high);
            }
            if *arr.get_unchecked(middle) > *arr.get_unchecked(low) {
                swap(arr, middle, low);
            }
        }
        /* Swap low item (now in position middle) into position (low+1) */
        swap(arr, middle, low + 1);
        /* Nibble from each end towards middle, swapping items when stuck */
        ll = low + 1;
        hh = high;
        loop {
            loop {
                ll += 1;
                unsafe {
                    #[allow(clippy::neg_cmp_op_on_partial_ord)]
                    if !(*arr.get_unchecked(low) > *arr.get_unchecked(ll)) {
                        break;
                    }
                }
            }
            loop {
                hh -= 1;
                unsafe {
                    #[allow(clippy::neg_cmp_op_on_partial_ord)]
                    if !(*arr.get_unchecked(hh) > *arr.get_unchecked(low)) {
                        break;
                    }
                }
            }
            if hh < ll {
                break;
            }
            swap(arr, ll, hh);
        }
        /* Swap middle item (in position low) back into correct position */
        swap(arr, low, hh);
        /* Re-set active partition */
        if hh <= median {
            low = ll;
        }
        if hh >= median {
            high = hh - 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_VALUES: [i32; 10] = [0, 1, 2, 3, 4, 9, 6, 7, 8, 10];

    #[test]
    fn test_q_sel() {
        let mut v = Vec::from(TEST_VALUES);
        let item = quick_select(&mut v);
        println!("x {:?}", v);
        println!("{:?}", item);
    }
}
