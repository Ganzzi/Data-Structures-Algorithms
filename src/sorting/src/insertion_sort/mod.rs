/// Insertion Sort
///
/// Insertion sort is a simple sorting algorithm that builds the final sorted array one item at a time.
///
/// # Arguments
///
/// * `nums` - A mutable slice of integers.
///
/// # Examples
///
/// ```
/// use crate::sorting::insertion_sort::insertion_sort;
///
/// let mut nums = [4, 2, 3, 1];
/// insertion_sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4]);
/// ```
pub fn insertion_sort(nums: &mut [i32]) {
    for i in 1..nums.len() {
        let mut j = i;

        while j > 0 && nums[j - 1] > nums[j] {
            nums.swap(j, j - 1);
            j -= 1;
        }
    }
}

/// Binary Insertion Sort
///
/// Binary insertion sort is a variant of insertion sort that uses a binary search to find the
/// correct position to insert the current element at each iteration.
///     
/// # Arguments
///
/// * `nums` - A mutable slice of integers.
///
/// # Examples
///
///
/// ```
/// use crate::sorting::insertion_sort::binary_insertion_sort;
///
/// let mut nums = [4, 2, 3, 1];
/// binary_insertion_sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4]);
/// ```
pub fn binary_insertion_sort(nums: &mut [i32]) {
    for i in 1..nums.len() {
        let mut j = i;

        let mut left = 0;
        let mut right = j;

        while left < right {
            let mid = left + (right - left) / 2;

            if nums[j] < nums[mid] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        while j > left {
            nums.swap(j, j - 1);
            j -= 1;
        }
    }
}
