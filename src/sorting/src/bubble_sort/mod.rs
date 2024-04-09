/// Sorts a slice of integers using the bubble sort algorithm.
///
/// This function iterates through the slice, comparing adjacent elements and swapping them
/// if they are in the wrong order. It repeats this process until the slice is fully sorted.
///
/// # Arguments
///
/// * `nums` - A mutable reference to a slice of integers to be sorted.
///
/// # Examples
///
/// ```
/// let mut nums = [4, 2, 3, 1];
/// bubble_sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4]);
/// ```
pub fn bubble_sort(nums: &mut [i32]) {
    let len = nums.len();

    if 2 > len {
        return;
    }

    for i in 1..len {
        for j in 0..len - i {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
            }
        }
    }
}

/// Sorts a slice of integers using an optimized version of the bubble sort algorithm.
///
/// This function iterates through the slice, comparing adjacent elements and swapping them
/// if they are in the wrong order. It repeats this process until the slice is fully sorted.
/// Unlike the standard bubble sort, it includes an optimization that stops iterating if no
/// swaps are made in a pass, indicating that the slice is already sorted.
///
/// # Arguments
///
/// * `nums` - A mutable reference to a slice of integers to be sorted.
///
/// # Examples
///
/// ```
/// let mut nums = [4, 2, 3, 1];
/// bubble_sort2(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4]);
/// ```
pub fn bubble_sort2(nums: &mut [i32]) {
    let mut unordered = true;
    let mut len = nums.len() - 1;

    while len > 0 && unordered {
        unordered = false;

        for i in 0..len {
            if nums[i] > nums[i + 1] {
                nums.swap(i, i + 1);
                unordered = true;
            }
        }

        len -= 1;
    }
}

/// Sorts a slice of integers using the cocktail sort algorithm.
///
/// This function iterates through the slice, performing a bidirectional bubble sort.
/// It starts by traversing the slice from left to right, bubbling the largest element to the
/// rightmost position. Then it traverses the slice from right to left, bubbling the smallest
/// element to the leftmost position. It repeats this process until the slice is fully sorted.
///
/// # Arguments
///
/// * `nums` - A mutable reference to a slice of integers to be sorted.
///
/// # Examples
///
/// ```
/// let mut nums = [4, 2, 3, 1];
/// cocktail_sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4]);
/// ```
pub fn cocktail_sort(nums: &mut [i32]) {
    let len = nums.len();

    let mut c = 0;
    let mut bubble = true;

    while c < (len >> 1) && bubble {
        bubble = false;

        // Bubble from left to right
        for j in c..(len - c - 1) {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
                bubble = true;
            }
        }

        // Bubble from right to left
        for j in (c + 1..=(len - c - 1)).rev() {
            if nums[j] < nums[j - 1] {
                nums.swap(j - 1, j);
                bubble = true;
            }
        }

        c += 1;
    }
}

/// Sorts a slice of integers using the comb sort algorithm.
///
/// This function iteratively shrinks the gap between elements to reduce the number of inversions
/// in the array, thus improving performance. It starts with a large gap and shrinks it by a constant
/// factor until it becomes 1. At each iteration, it performs bubble sort-like comparisons and swaps
/// elements if they are in the wrong order.
///
/// # Arguments
///
/// * `nums` - A mutable reference to a slice of integers to be sorted.
///
/// # Examples
///
/// ```
/// let mut nums = [4, 2, 3, 1];
/// comb_sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4]);
/// ```
pub fn comb_sort(nums: &mut [i32]) {
    const SHRINK_FACTOR: f32 = 1.3;

    let len = nums.len();
    let mut gap = len;

    while gap > 1 {
        gap = (gap as f32 / SHRINK_FACTOR) as usize;

        for i in gap..len {
            if nums[i - gap] > nums[i] {
                nums.swap(i - gap, i);
            }
        }
    }
}
