/// Interpolation Search
/// 
/// Performs an interpolation search for a target number in a sorted slice of integers.
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
/// use searching::interpolation_search::interpolation_search;
/// 
/// let nums = [10, 20, 30, 40, 50];
/// let target = 30;
/// assert_eq!(interpolation_search(&nums, target), Some(2));
/// ```
pub fn interpolation_search(nums: &[i32], num: i32) -> Option<usize> {
    let mut found = None;

    let mut high_index = nums.len() - 1;
    let mut low_index = 0;

    loop {
        let high_value = nums[high_index];
        let low_value = nums[low_index];

        if num > high_value || num < low_value  {
            break;
        }

        if high_value == low_value {
            if num == nums[high_index] || num == nums[low_index] {
                found = Some(high_index);
            }
            break;
        }

        let interpolant_index = (((num - low_value) * (high_index - low_index) as i32)
            / (high_value - low_value)) as usize
            + low_index;
    
        if nums[interpolant_index] < num {
            low_index = interpolant_index + 1;
        } else if nums[interpolant_index] > num {
            high_index = interpolant_index - 1;
        } else {
            found = Some(interpolant_index);
            break;
        }
    }

    found
}
