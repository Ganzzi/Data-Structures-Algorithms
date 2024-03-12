/// Sequential Search
/// 
/// Performs a sequential search for a target number in a slice of integers.
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
/// use searching::sequential_search::sequential_search;
/// 
/// let nums = [10, 20, 30, 40, 50];
/// let target = 30;
/// assert_eq!(sequential_search(&nums, target), Some(2));
/// ```
pub fn sequential_search(nums: &[i32], num: i32) -> Option<usize> {
    let mut found = None;
    let mut pos = 0;

    while found.is_none() && pos < nums.len() {
        if num == nums[pos] {
            found = Some(pos);
        } else {
            pos += 1;
        }
    }
    found
}

/// Ordered Sequential Search
/// 
/// Performs a sequential search for a target number in a sorted slice of integers.
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
/// use searching::sequential_search::ordered_sequential_search;
/// 
/// let nums = [10, 20, 30, 40, 50];
/// let target = 30;
/// assert_eq!(ordered_sequential_search(&nums, target), Some(2));
/// ```
pub fn ordered_sequential_search(nums: &[i32], num: i32) -> Option<usize> {
    let mut found = None;
    let mut pos = 0;
    let mut stop = false;

    while found.is_none() && pos < nums.len() && !stop {
        if num == nums[pos] {
            found = Some(pos);
        } else if num < nums[pos] {
            stop = true;
        } else {
            pos += 1;
        }
    }
    found
}
