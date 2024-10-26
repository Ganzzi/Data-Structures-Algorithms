/// # Sorts a slice of integers using the comb sort algorithm.
///
/// This function iteratively shrinks the gap between elements to reduce the number of inversions
/// in the array, thus improving performance. It starts with a large gap and shrinks it by a constant
/// factor until it becomes 1. At each iteration, it performs bubble sort-like comparisons and swaps
/// elements if they are in the wrong order.
///
/// # Arguments
///
/// * `nums` - A mutable reference to a slice of integers to be sorted.
///
/// # Examples
///
/// ```
/// let mut nums = [4, 2, 3, 1];
///
/// crate::sorting::bubble_sort::comb_sort(&mut nums);
///
/// assert_eq!(nums, [1, 2, 3, 4]);
/// ```
pub fn comb_sort(nums: &mut [i32]) {
    const SHRINK_FACTOR: f32 = 1.3;

    let len = nums.len();
    let mut gap = len;
    let mut sorted = false;

    while gap > 1 || !sorted {
        gap = (gap as f32 / SHRINK_FACTOR).max(1.0) as usize;

        sorted = true;

        for i in gap..len {
            if nums[i - gap] > nums[i] {
                nums.swap(i - gap, i);
                sorted = false;
            }
        }
    }
}
