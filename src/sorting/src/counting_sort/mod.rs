///
/// Counting sort
///
/// Counting sort is a sorting algorithm that sorts the elements of an array by counting the number of occurrences of each unique element in the array. The count is stored in an auxiliary array and the sorting is done by mapping the count as an index of the auxiliary array.
///
/// # Arguments
///
/// * `nums` - A mutable slice of i32 elements to be sorted
///
/// # Examples
///
/// Basic usage:
/// ```
/// use crate::sorting::counting_sort::counting_sort;
///
/// let mut arr = [4, 2, 2, 8, 3, 3, 1];
///
/// counting_sort(&mut arr);
///
/// assert_eq!(arr, [1, 2, 2, 3, 3, 4, 8]);
/// ```
///
/// Sorting an array with negative numbers:
/// ```
/// use crate::sorting::counting_sort::counting_sort;
///
/// let mut arr = [-4, -65, -2, -32, 0, -99, -2, -83, -782, -1];
///
/// counting_sort(&mut arr);
///
/// assert_eq!(arr, [-782, -99, -83, -65, -32, -4, -2, -2, -1, 0]);
/// ```
pub fn counting_sort(nums: &mut [i32]) {
    let &min = nums.iter().min().unwrap();
    let &max = nums.iter().max().unwrap();

    let mut counter = vec![0; (max - min + 1) as usize];
    for &num in nums.iter() {
        counter[(num - min) as usize] += 1;
    }

    let mut j = 0;
    for i in min..=max {
        while counter[(i - min) as usize] > 0 {
            nums[j] = i;
            counter[(i - min) as usize] -= 1;
            j += 1;
        }
    }
}
