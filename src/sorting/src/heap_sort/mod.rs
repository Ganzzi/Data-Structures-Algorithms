macro_rules! left_child {
    ($parent:expr) => {
        $parent << 1
    };
}

macro_rules! right_child {
    ($parent:expr) => {
        ($parent << 1) + 1
    };
}

///
/// # Heap Sort
///
/// Heap sort is a comparison-based sorting algorithm that uses a binary heap data structure to sort elements.
///
/// # Arguments
///
/// * `nums` - A mutable reference to a vector of i32 integers
///
/// # Examples
///
/// ```
/// use crate::sorting::heap_sort::heap_sort;
///
/// let mut nums = vec![4, 65, 2, -31, 0, 99, 2, 83, 1];
/// heap_sort(&mut nums);
/// assert_eq!(nums, vec![-31, 0, 1, 2, 2, 4, 65, 83, 99]);
///
/// let mut nums = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
/// heap_sort(&mut nums);
/// assert_eq!(nums, vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
///
/// let mut nums = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
/// heap_sort(&mut nums);
/// assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
///
/// let mut nums = vec![1];
/// heap_sort(&mut nums);
/// assert_eq!(nums, vec![1]);
///
/// let mut nums: Vec<i32> = vec![];
/// heap_sort(&mut nums);
/// assert_eq!(nums, vec![]);
/// ```
pub fn heap_sort(nums: &mut [i32]) {
    for i in (0..=nums.len() >> 1).rev() {
        heapify(nums, i);
    }

    for i in (0..nums.len()).rev() {
        nums.swap(0, i);
        heapify(&mut nums[..i], 0);
    }

    fn heapify(nums: &mut [i32], root_index: usize) {
        let mut max_index = root_index;
        let left_index = left_child!(root_index);
        let right_index = right_child!(root_index);

        if left_index < nums.len() && nums[max_index] < nums[left_index] {
            max_index = left_index;
        }

        if right_index < nums.len() && nums[max_index] < nums[right_index] {
            max_index = right_index;
        }

        if max_index != root_index {
            nums.swap(max_index, root_index);
            heapify(nums, max_index);
        }
    }
}
