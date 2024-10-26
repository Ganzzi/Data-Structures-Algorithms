pub mod bidirectional_selection_sort;

/// # Selection Sort
///
/// Selection sort is a simple sorting algorithm that works by selecting the smallest element from the unsorted portion of the array and placing it at the beginning of the array.
///
/// # Arguments
///
/// * `nums` - A mutable reference to a slice of integers
///
/// # Examples
///
/// ```
/// use crate::sorting::selection_sort::selection_sort;
///
/// let mut nums = vec![4, 2, 3, 1];
///
/// selection_sort(&mut nums);
///
/// assert_eq!(nums, [1, 2, 3, 4]);
///
/// ```
pub fn selection_sort(nums: &mut [i32]) {
    for i in 0..nums.len() {
        let mut min_index = i;

        for j in i + 1..nums.len() {
            if nums[min_index] > nums[j] {
                min_index = j;
            }
        }

        nums.swap(i, min_index);
    }
}
