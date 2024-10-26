use crate::insertion_sort::binary_insertion_sort::binary_insertion_sort;

/// # Balanced Tim Sort
///
/// Balanced Tim Sort is a hybrid sorting algorithm derived from Tim Sort. It is designed to be more efficient for small arrays.
///
/// # Arguments
///
/// * `nums` - A mutable reference to a slice of integers.
///
/// # Example
///
/// ```
/// let mut nums = vec![
///      2, 4, 7, 8, 23, 19, 16, 14, 13, 12, 10, 20, 18, 17, 15, 11, 9, -1, 5, 6, 1, 3, 21, 40, 22,
///      39, 38, 37, 36, 35, 34, 33, 24, 30, 31, 32, 25, 26, 27, 28, 29, 41, 42, 43, 44, 45, 46, 47,
///      48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 80, 79, 78, 77, 76, 75, 74, 73, 72, 71,
///      70, 61, 62, 63, 64, 65, 66, 67, 68, 69, 95, 94, 93, 92, 91, 90, 85, 82, 83, 84, 81, 86, 87,
///      88, 89,
/// ];
///
/// crate::sorting::tim_sort::balanced_tim_sort::balanced_tim_sort(&mut nums);
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
pub fn balanced_tim_sort(nums: &mut [i32]) {
    let len = nums.len();

    if len < crate::tim_sort::MIN_MERGE {
        binary_insertion_sort(nums);
        return;
    }

    let min_run = calc_min_run(len);
    let mut runs = vec![];

    let mut i = 0;
    while i < len {
        let mut run_size = calc_run_size(&mut nums[i..]);
        let remaining = len - i;

        if remaining < min_run && remaining > run_size {
            run_size = remaining;
            binary_insertion_sort(&mut nums[i..]);
        }

        runs.push((i, run_size));
        i += run_size;

        merge_collapse(&mut runs, nums);
    }

    merge_force_collapse(&mut runs, nums);

    /// Calculate the minimum run size.
    /// # Arguments
    /// * `len` - The length of the slice.
    /// # Returns
    /// The minimum run size.
    fn calc_min_run(len: usize) -> usize {
        let mut r = 0;
        let mut len = len;

        while len >= crate::tim_sort::MIN_MERGE {
            r |= len & 1;
            len >>= 1;
        }

        len + r
    }

    /// Calculate the size of the next run, and reverse it if it is descending.
    /// # Arguments
    /// * `nums` - A mutable reference to a slice of integers.
    /// # Returns
    /// The size of the next run.
    fn calc_run_size(nums: &mut [i32]) -> usize {
        let mut run_size = 1;

        if nums[run_size] < nums[0] {
            while run_size < nums.len() && nums[run_size] < nums[run_size - 1] {
                run_size += 1;
            }
            nums[0..run_size].reverse();
        } else {
            while run_size < nums.len() && nums[run_size] >= nums[run_size - 1] {
                run_size += 1;
            }
        }

        run_size
    }

    /// Merge runs until the stack is in the correct state.
    ///
    /// Rules:  
    /// 1. If the stack has at least three runs, A, B, and C. If A <= B + C, then merge A if A < C, or merge B and C otherwise.
    /// 2. Else if the stack has two runs, A and B, and A <= B, merge A and B.
    /// 3. Else, the stack has two runs, A and B, and A > B or only one run, leave it.
    ///
    /// # Arguments
    /// * `runs` - A mutable reference to a vector of runs.
    /// * `nums` - A mutable reference to a slice of integers.
    fn merge_collapse(runs: &mut Vec<(usize, usize)>, nums: &mut [i32]) {
        while runs.len() > 1 {
            let n = runs.len();

            if n >= 3 && runs[n - 3].1 <= runs[n - 2].1 + runs[n - 1].1 {
                if runs[n - 3].1 < runs[n - 1].1 {
                    merge_at(runs, n - 3, nums);
                } else {
                    merge_at(runs, n - 2, nums);
                }
            } else if runs[n - 2].1 <= runs[n - 1].1 {
                merge_at(runs, n - 2, nums);
            } else {
                break;
            }
        }
    }

    /// Merge all remaining runs in the stack.
    /// # Arguments
    /// * `runs` - A mutable reference to a vector of runs.
    /// * `nums` - A mutable reference to a slice of integers.
    fn merge_force_collapse(runs: &mut Vec<(usize, usize)>, nums: &mut [i32]) {
        while runs.len() > 1 {
            let n = runs.len();

            if n >= 3 && runs[n - 3].1 < runs[n - 1].1 {
                merge_at(runs, n - 3, nums);
            } else {
                merge_at(runs, n - 2, nums);
            }
        }
    }

    /// Merge two consecutive runs.
    /// # Arguments
    /// * `runs` - A mutable reference to a vector of runs.
    /// * `i` - The index of the first run.
    /// * `nums` - A mutable reference to a slice of integers.
    fn merge_at(runs: &mut Vec<(usize, usize)>, i: usize, nums: &mut [i32]) {
        let (start1, len1) = runs[i];
        let (_, len2) = runs[i + 1];

        merge(&mut nums[start1..start1 + len1 + len2], len1);
        runs[i] = (start1, len1 + len2);
        runs.remove(i + 1);
    }

    /// Merge two consecutive subarrays.
    /// # Arguments
    /// * `nums` - A mutable reference to a slice of integers.
    /// * `mid` - The middle index.
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
