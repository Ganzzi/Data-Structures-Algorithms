/// Quick sort
///
/// Quick sort is a sorting algorithm that uses the divide and conquer strategy to sort an array.
///
/// # Arguments
///
/// * `nums` - A mutable reference to a vector of i32 integers
/// * `low` - The lower bound of the array
/// * `high` - The upper bound of the array
///
/// # Examples
///
/// ```
/// use crate::sorting::quick_sort::quick_sort;
///
/// let mut nums = vec![4, 2, 1, 3];
/// let len = nums.len();
/// quick_sort(&mut nums, 0, len - 1);
///
/// assert_eq!(nums, vec![1, 2, 3, 4]);
/// ```
pub fn quick_sort(nums: &mut [i32], low: usize, high: usize) {
    if low >= high {
        return;
    }

    let mut left_index = low;
    let mut right_index = high;

    while left_index < right_index {
        while left_index < right_index && nums[right_index] >= nums[low] {
            right_index -= 1;
        }

        while left_index < right_index && nums[left_index] <= nums[low] {
            left_index += 1;
        }

        nums.swap(left_index, right_index);
    }

    nums.swap(low, left_index);

    if left_index >= 2 {
        quick_sort(nums, low, left_index - 1);
    }

    quick_sort(nums, left_index + 1, high);
}
