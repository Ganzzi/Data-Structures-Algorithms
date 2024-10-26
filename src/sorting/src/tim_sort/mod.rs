use crate::insertion_sort::binary_insertion_sort;

pub mod balanced_tim_sort;

const MIN_MERGE: usize = 64;

/// # Tim Sort
///
/// Tim Sort is a hybrid sorting algorithm derived from merge sort and insertion sort. It is designed to perform well on many kinds of real-world data. It was designed by Tim Peters in 2002 for use in the Python programming language. It is the default sorting algorithm in Python, Java, and Android.
///
/// # Arguments
///
/// * `nums` - A mutable reference to a slice of integers.
///
/// # Example
///
/// ```
/// use crate::sorting::tim_sort::tim_sort;
///
/// let mut nums = vec![4, 2, 3, 1];
///
/// tim_sort(&mut nums);
///
/// assert_eq!(nums, [1, 2, 3, 4]);
///
/// let mut nums = vec![
///      2, 4, 7, 8, 23, 19, 16, 14, 13, 12, 10, 20, 18, 17, 15, 11, 9, -1, 5, 6, 1, 3, 21, 40, 22,
///      39, 38, 37, 36, 35, 34, 33, 24, 30, 31, 32, 25, 26, 27, 28, 29, 41, 42, 43, 44, 45, 46, 47,
///      48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 80, 79, 78, 77, 76, 75, 74, 73, 72, 71,
///      70, 61, 62, 63, 64, 65, 66, 67, 68, 69, 95, 94, 93, 92, 91, 90, 85, 82, 83, 84, 81, 86, 87,
///      88, 89,
/// ];
///
/// tim_sort(&mut nums);
///
/// assert_eq!( nums, [
///     -1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
///     24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
///     47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69,
///     70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92,
///     93, 94, 95,
/// ]);
///
/// ```
pub fn tim_sort(nums: &mut [i32]) {
    let len = nums.len();

    if len < MIN_MERGE {
        binary_insertion_sort(nums);
        return;
    }

    let min_run = calc_min_run(len);

    let mut i = 0;
    while i < len {
        let run_size = (i + min_run).min(len) - i;
        binary_insertion_sort(&mut nums[i..i + run_size]);
        i += run_size;
    }

    let mut size = min_run;
    while size < len {
        let mut left = 0;

        while left < len {
            let mid = (left + size).min(len);
            let right = (left + 2 * size).min(len);

            if mid < right {
                merge(&mut nums[left..right], mid - left);
            }

            left += 2 * size;
        }

        size *= 2;
    }

    fn calc_min_run(len: usize) -> usize {
        let mut r = 0;
        let mut len: usize = len;

        while len >= MIN_MERGE {
            r |= len & 1;
            len >>= 1;
        }

        len + r
    }

    fn merge(nums: &mut [i32], mid: usize) {
        let left_nums = nums[..mid].to_vec();
        let right_nums = nums[mid..].to_vec();

        let mut left = 0;
        let mut right = 0;
        let mut i = 0;

        while left < left_nums.len() && right < right_nums.len() {
            if left_nums[left] <= right_nums[right] {
                nums[i] = left_nums[left];
                left += 1;
            } else {
                nums[i] = right_nums[right];
                right += 1;
            }

            i += 1;
        }

        while left < left_nums.len() {
            nums[i] = left_nums[left];
            left += 1;
            i += 1;
        }

        while right < right_nums.len() {
            nums[i] = right_nums[right];
            right += 1;
            i += 1;
        }
    }
}
