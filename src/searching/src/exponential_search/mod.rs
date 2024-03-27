use crate::binary_search::binary_search;

/// Exponential Search
/// 
/// Performs an exponential search for a target number in a sorted slice of integers.
/// 
/// # Arguments
/// 
/// * `nums` - A slice of integers to be searched.
/// * `num` - The target number to search for.
/// 
/// # Returns
/// 
/// Returns the index of the target number if found, otherwise returns None.
/// 
/// # Examples
/// 
/// ```
/// use searching::exponential_search::exponential_search;
/// 
/// let nums = [10, 20, 30, 40, 50];
/// let target = 30;
/// assert_eq!(exponential_search(&nums, target), Some(2));
/// ```
pub fn exponential_search(nums: &[i32], num: i32) -> Option<usize> {
    let size = nums.len();
    let mut high_index = 1;

    while high_index < size && nums[high_index] < num {
        high_index *= 2;
    }

    let low_index = high_index / 2 + high_index % 2;
    let slice_end = high_index.min(size-1);
    binary_search(&nums[low_index..=slice_end], num)
        .map(|i| i + (low_index))
}