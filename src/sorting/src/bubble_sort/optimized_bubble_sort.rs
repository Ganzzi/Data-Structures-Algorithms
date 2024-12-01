/// # Optimized bubble sort
///
/// Sorts a slice of integers using an optimized version of the bubble sort algorithm.
///
/// This function iterates through the slice, comparing adjacent elements and swapping them
/// if they are in the wrong order. It repeats this process until the slice is fully sorted.
/// Unlike the standard bubble sort, it includes an optimization that stops iterating if no
/// swaps are made in a pass, indicating that the slice is already sorted.
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
/// crate::sorting::bubble_sort::optimized_bubble_sort::optimized_bubble_sort(&mut nums);
///
/// assert_eq!(nums, [1, 2, 3, 4]);
/// ```
pub fn optimized_bubble_sort(nums: &mut [i32]) {
    let mut unordered = true;
    let mut len = nums.len() - 1;

    while len > 0 && unordered {
        unordered = false;

        for i in 0..len {
            if nums[i] > nums[i + 1] {
                nums.swap(i, i + 1);
                unordered = true;
            }
        }

        len -= 1;
    }
}
