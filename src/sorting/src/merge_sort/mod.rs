///
/// # Merge Sort
///
/// Merge sort is a divide-and-conquer algorithm that divides a list into equal halves until it has two single elements and then merges the sub-lists until the entire list has been reassembled in order.
///
/// # Arguments
///
/// * `nums` - A mutable slice of i32 elements
///
/// # Example
///
/// ```
/// use crate::sorting::merge_sort::merge_sort;
///
/// let mut nums = vec![4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
///
/// merge_sort(&mut nums);
///
/// assert_eq!(nums, vec![-31, 0, 1, 2, 2, 4, 65, 83, 99, 782]);
/// ```
pub fn merge_sort(nums: &mut [i32]) {
    if nums.len() > 1 {
        let mid = nums.len() >> 1;

        merge_sort(&mut nums[..mid]);

        merge_sort(&mut nums[mid..]);

        merge(nums, mid);
    }

    fn merge(nums: &mut [i32], mid_index: usize) {
        let mut first_half_index = 0;
        let mut last_half_index = mid_index;
        let mut sorted_nums = Vec::new();

        while first_half_index < mid_index && last_half_index < nums.len() {
            if nums[first_half_index] < nums[last_half_index] {
                sorted_nums.push(nums[first_half_index]);
                first_half_index += 1;
            } else {
                sorted_nums.push(nums[last_half_index]);
                last_half_index += 1;
            }
        }

        while first_half_index < mid_index {
            sorted_nums.push(nums[first_half_index]);
            first_half_index += 1;
        }
        while last_half_index < nums.len() {
            sorted_nums.push(nums[last_half_index]);
            last_half_index += 1;
        }

        nums.copy_from_slice(&sorted_nums);
    }
}
