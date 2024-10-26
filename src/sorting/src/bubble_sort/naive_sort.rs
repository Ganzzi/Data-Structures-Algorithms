/// # Naive sort
///
/// Sorts a mutable slice of integers using the simplest sort algorithm.
///
/// Naive sort is a simple sorting algorithm that compares each element with every other element
/// in the slice and swaps them if they are in the wrong order. It repeats this process until the
/// slice is fully sorted. This algorithm is inefficient and should not be used for large slices.
///
/// # Arguments
///
/// * `nums` - A mutable reference to a slice of integers to be sorted.
///
/// # Examples
///
/// ```
/// let mut nums = [4, 2, 1, 3];
///
/// crate::sorting::bubble_sort::naive_sort(&mut nums);
///
/// assert_eq!(nums, [1, 2, 3, 4]);
/// ```
pub fn naive_sort(nums: &mut [i32]) {
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if nums[i] < nums[j] {
                nums.swap(i, j);
            }
        }
    }
}
