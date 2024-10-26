pub mod binary_insertion_sort;

/// # Insertion Sort
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
/// let mut nums = [4, 2, 3, 1];
///
/// crate::sorting::insertion_sort::insertion_sort(&mut nums);
///
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
