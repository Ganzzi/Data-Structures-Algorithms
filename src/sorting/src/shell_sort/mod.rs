///
/// Shell Sort
///
/// Shell sort is a generalization of insertion sort that allows the exchange of items that are far apart.
///
/// # Arguments
///
/// * `nums` - A mutable reference to a vector of i32 integers
///
/// # Examples
///
/// ```
/// use crate::sorting::shell_sort::shell_sort;
///
/// let mut nums = [5, 3, 8, 4, 2, 1];
///
/// shell_sort(&mut nums);
///
/// assert_eq!(nums, [1, 2, 3, 4, 5, 8]);
/// ```
pub fn shell_sort(nums: &mut [i32]) {
    let mut gap = nums.len() / 2;

    while gap >= 1 {
        for i in gap..nums.len() {
            let mut j = i;
            while j >= gap && nums[j - gap] > nums[j] {
                nums.swap(j - gap, j);
                j -= gap;
            }
        }
        gap /= 2;
    }
}
