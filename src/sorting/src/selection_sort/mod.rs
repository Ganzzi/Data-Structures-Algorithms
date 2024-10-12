/// Selection Sort
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

///
/// Bidirectional Selection Sort
///
/// Bidirectional selection sort is a variation of selection sort that works by selecting the smallest and largest elements from the unsorted portion of the array and placing them at the beginning and end of the array, respectively.
///
/// # Arguments
///
/// * `nums` - A mutable reference to a slice of integers
///
/// # Examples
///
/// ```
/// use crate::sorting::selection_sort::bidirectional_selection_sort;
///
/// let mut nums = vec![4, 2, 3, 1];
///
/// bidirectional_selection_sort(&mut nums);
///
/// assert_eq!(nums, [1, 2, 3, 4]);
///
/// ```
pub fn bidirectional_selection_sort(nums: &mut [i32]) {
    let mut left_index = 0;
    let mut right_index = nums.len() - 1;

    while left_index < right_index {
        let mut min_index = left_index;
        let mut max_index = right_index;

        for i in left_index..=right_index {
            if nums[i] < nums[min_index] {
                min_index = i;
            }

            if nums[i] > nums[max_index] {
                max_index = i;
            }
        }

        if max_index == left_index {
            max_index = min_index;
        }

        nums.swap(left_index, min_index);

        nums.swap(right_index, max_index);

        left_index += 1;
        right_index -= 1;
    }
}
