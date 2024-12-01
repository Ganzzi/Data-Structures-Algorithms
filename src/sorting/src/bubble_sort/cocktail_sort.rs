/// # Cocktail Sort
///
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
///
/// crate::sorting::bubble_sort::cocktail_sort::cocktail_sort(&mut nums);
///
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
