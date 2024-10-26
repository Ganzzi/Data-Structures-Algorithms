pub mod cocktail_sort;
pub mod comb_sort;
pub mod naive_sort;
pub mod optimized_bubble_sort;

/// # Bubble Sort
///
/// Sorts a slice of integers using the bubble sort algorithm.
///
/// This function iterates through the slice, comparing adjacent elements and swapping them
/// if they are in the wrong order. It repeats this process until the slice is fully sorted.
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
/// crate::sorting::bubble_sort::bubble_sort(&mut nums);
///
/// assert_eq!(nums, [1, 2, 3, 4]);
/// ```
pub fn bubble_sort(nums: &mut [i32]) {
    let len = nums.len();

    if 2 > len {
        return;
    }

    for i in 1..len {
        for j in 0..len - i {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
            }
        }
    }
}
