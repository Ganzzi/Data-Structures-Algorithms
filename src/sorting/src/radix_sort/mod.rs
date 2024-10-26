pub mod radix_sort_by_sign;

///
/// # Radix Sort
///
/// Radix sort is a non-comparative integer sorting algorithm that sorts data with integer keys by grouping keys by the individual digits which share the same significant position and value. A positional notation is required, but because integers can represent strings of characters (e.g., names or dates) and specially formatted floating-point numbers, radix sort is not limited to integers.
///
/// # Arguments
///
/// * `nums` - A mutable slice of i32 elements.
///
/// # Examples
///
/// ```
///
/// use crate::sorting::radix_sort::radix_sort;
///
/// let mut nums = vec![-170, 45, -75, 90, -802, 24, -2, 66];
///
/// radix_sort(&mut nums);
///
/// assert_eq!(nums, [-802, -170, -75, -2, 24, 45, 66, 90]);
///
/// ```
pub fn radix_sort(nums: &mut [i32]) {
    if nums.is_empty() {
        return;
    }

    let min = *nums.iter().min().unwrap();
    let max = *nums.iter().max().unwrap();

    let limit = max.abs().max(min.abs());
    let mut digit_place = 1;

    while limit / digit_place > 0 {
        counting_sort_by_digit(nums, digit_place);
        digit_place *= 10;
    }

    fn counting_sort_by_digit(nums: &mut [i32], digit_place: i32) {
        let mut counts = vec![0; 19];
        let digit_index = |x: i32| {
            if x < 0 {
                9 - (-x / digit_place % 10) as usize
            } else {
                9 + (x / digit_place % 10) as usize
            }
        };

        for &num in nums.iter() {
            counts[digit_index(num)] += 1;
        }

        for i in 1..19 {
            counts[i] += counts[i - 1];
        }

        let mut sorted = vec![0; nums.len()];
        for &num in nums.iter().rev() {
            let idx = digit_index(num);
            sorted[counts[idx] - 1] = num;
            counts[idx] -= 1;
        }

        nums.clone_from_slice(&sorted);
    }
}
