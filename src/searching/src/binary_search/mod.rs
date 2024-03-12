/// Binary Search
/// 
/// Performs a binary search for a target number in a sorted slice of integers.
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
/// use searching::binary_search::binary_search;
/// 
/// let nums = [10, 20, 30, 40, 50];
/// let target = 30;
/// assert_eq!(binary_search(&nums, target), Some(2));
/// ```
pub fn binary_search(nums: &[i32], num: i32) -> Option<usize> {
    let mut found = None;
    if 0 == nums.len() {
        return found;
    }
    let mut low_index = 0;
    let mut high_index = nums.len() - 1;

    while found.is_none() && low_index <= high_index {
        let mid_index = (low_index + high_index) / 2 + (low_index + high_index) % 2;
        if num == nums[mid_index] {
            found = Some(mid_index);
        } else if num > nums[mid_index] {
            low_index = mid_index + 1;
        } else {
            high_index = mid_index - 1;
        }
    }

    found
}

/// Recursive Binary Search
/// 
/// Performs a recursive binary search for a target number in a sorted slice of integers.
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
/// use searching::binary_search::recursive_binary_search;
/// 
/// let nums = [10, 20, 30, 40, 50];
/// let target = 30;
/// assert_eq!(recursive_binary_search(&nums, target), Some(2));
/// ```
pub fn recursive_binary_search(nums: &[i32], num: i32) -> Option<usize> {
    if 0 == nums.len() { 
        return None;
    }

    let high_index = nums.len() - 1;
    let mid_index = high_index / 2 + high_index % 2;

    if num == nums[mid_index] {
        return Some(mid_index);
    } else if num > nums[mid_index] {
        return recursive_binary_search(&nums[mid_index+1..], num).map(|i| i + mid_index + 1);
    } else {
        return recursive_binary_search(&nums[..mid_index], num);
    }
}
