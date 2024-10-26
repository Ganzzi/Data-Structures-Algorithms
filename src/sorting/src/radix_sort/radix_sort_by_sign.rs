///
/// # Radix Sort by sign
///
/// Radix sort by sign is a non-comparison sorting algorithm that sorts signed integers in ascending order.
/// It sorts the negative integers in descending order and the positive integers in ascending order.
/// It uses counting sort as a subroutine to sort the integers by their digits.
///
/// # Arguments
///
/// * `nums` - A mutable slice of i32 elements.
///
/// # Examples
///
/// ```
/// use crate::sorting::radix_sort::radix_sort_by_sign;
///
/// let mut nums = vec![-170, 45, -75, 90, -802, 24, -2, 66];
///
/// radix_sort_by_sign(&mut nums);
///
/// assert_eq!(nums, [-802, -170, -75, -2, 24, 45, 66, 90]);
///
/// ```
pub fn radix_sort_by_sign(nums: &mut [i32]) {
    if nums.is_empty() {
        return;
    }

    let mut negatives: Vec<i32> = nums.iter().filter(|&&x| x < 0).copied().collect();
    let mut positives: Vec<i32> = nums.iter().filter(|&&x| x >= 0).copied().collect();

    radix_sort_internal(&mut positives, false);
    radix_sort_internal(&mut negatives, true);

    nums.clone_from_slice(&[negatives, positives].concat());

    fn radix_sort_internal(nums: &mut [i32], sort_descending: bool) {
        let limit = if sort_descending {
            -nums.iter().min().unwrap_or(&0)
        } else {
            *nums.iter().max().unwrap_or(&0)
        };

        let mut digit_place = 1;
        while limit / digit_place > 0 {
            counting_sort_by_digit(nums, digit_place);
            digit_place *= 10;
        }

        if sort_descending {
            nums.reverse();
        }
    }

    fn counting_sort_by_digit(nums: &mut [i32], digit_place: i32) {
        let mut counts = vec![0; 10];
        let digit_value = |x: i32| (x / digit_place % 10).abs() as usize;

        for &num in nums.iter() {
            counts[digit_value(num)] += 1;
        }

        for i in 1..10 {
            counts[i] += counts[i - 1];
        }

        let mut sorted = vec![0; nums.len()];
        for &num in nums.iter().rev() {
            sorted[counts[digit_value(num)] - 1] = num;
            counts[digit_value(num)] -= 1;
        }

        nums.clone_from_slice(&sorted);
    }
}
