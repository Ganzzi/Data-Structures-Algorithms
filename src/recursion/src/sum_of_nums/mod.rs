/// # Sum of Numbers
///
/// This function calculates the sum of numbers in a slice using recursion.
///
/// # Arguments
///
/// * `nums` - A slice of integers representing the numbers to be summed.
///
/// # Returns
///
/// The sum of the numbers in the input slice.
///
/// # Examples
///
/// ```
/// use crate::recursion::sum_of_nums::sum_of_nums;
///
/// let nums = [1, 3, 9, 2, 4, 5];
/// let sum = sum_of_nums(&nums);
///
/// assert_eq!(sum, 24);
/// ```
pub fn sum_of_nums(nums: &[i32]) -> i32 {
    if 1 == nums.len() {
        nums[0]
    } else {
        nums[0] + sum_of_nums(&nums[1..])
    }
}

/// # Sum of Numbers
///
/// This function calculates the sum of numbers in a slice using tail recursion.
///
/// # Arguments
///
/// * `sum` - An accumulator variable for the sum.
/// * `nums` - A slice of integers representing the numbers to be summed.
///
/// # Returns
///
/// The sum of the numbers in the input slice.
///
/// # Examples
///
/// ```
/// use crate::recursion::sum_of_nums::tail_sum_of_nums;
///
/// let nums = [1, 3, 9, 2, 4, 5];
/// let sum = tail_sum_of_nums(0, &nums);
///
/// assert_eq!(sum, 24);
/// ```
pub fn tail_sum_of_nums(sum: i32, nums: &[i32]) -> i32 {
    if 1 == nums.len() {
        sum + nums[0]
    } else {
        tail_sum_of_nums(sum + nums[0], &nums[1..])
    }
}
