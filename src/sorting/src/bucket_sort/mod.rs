pub mod bucket;

use bucket::Bucket;

use crate::{insertion_sort::insertion_sort, quick_sort::quick_sort, shell_sort::shell_sort};

macro_rules! hasher {
    ($value:expr, $bucket_count:expr) => {
        $value / $bucket_count
    };
}

const SMALL_SIZE_BUCKETS: f32 = 0.25;
const MEDIUM_SIZE_BUCKETS: f32 = 0.35;
const LARGE_SIZE_BUCKETS: f32 = 0.42;

///
/// Bucket Sort
///
/// Bucket sort is a hybrid sorting algorithm that distributes elements into a number of buckets, based on a hashing function that maps elements to their respective buckets. The goal is to reduce the sorting complexity by partitioning the input data into smaller groups (buckets) and sorting each bucket individually.
///
/// In this implementation, the number of buckets is determined dynamically based on the size of the input array:
/// - For small arrays (2 to 20 elements): 0.25 * array length
/// - For medium-sized arrays (21 to 1000 elements): 0.35 * array length
/// - For larger arrays: 0.42 * array length
///
/// Each bucket is sorted using different sorting algorithms based on its size:
/// - **Insertion Sort** is employed for small buckets (2 to 20 elements). It is efficient for small datasets due to its low overhead and effectiveness with nearly sorted arrays.
/// - **Shell Sort** is applied to medium-sized buckets (21 to 1000 elements). Shell sort offers a good balance of simplicity and speed for this range of sizes.
/// - **Quick Sort** is used for larger buckets (over 1000 elements), as it provides better average-case performance for sorting larger collections.
///
///
/// # Arguments
///
/// * `nums` - A mutable slice of i32 elements to be sorted
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use crate::sorting::bucket_sort::bucket_sort;
///
/// let mut nums = vec![4, 65, 2, -32, 0, 99, 2, 83, 782, 1];
///
/// bucket_sort(&mut nums);
///
/// assert_eq!(nums, vec![-32, 0, 1, 2, 2, 4, 65, 83, 99, 782]);
/// ```
/// Sorting an array with duplicates:
///
/// ```
/// use crate::sorting::bucket_sort::bucket_sort;
///
/// let mut nums = vec![5, 1, 3, 3, 2, 1, 4];
///
/// bucket_sort(&mut nums);
///
/// assert_eq!(nums, vec![1, 1, 2, 3, 3, 4, 5]);
/// ```
///
/// Sorting an empty array:
///
/// ```
/// use crate::sorting::bucket_sort::bucket_sort;
///
/// let mut nums: Vec<i32> = vec![];
///
/// bucket_sort(&mut nums);
///
/// assert_eq!(nums, vec![]);
/// ```
/// Sorting an array with negative numbers:
///
/// ```
/// use crate::sorting::bucket_sort::bucket_sort;
///
/// let mut nums = vec![-4, -65, -2, -32, 0, -99, -2, -83, -782, -1];
///
/// bucket_sort(&mut nums);
///
/// assert_eq!(nums, vec![-782, -99, -83, -65, -32, -4, -2, -2, -1, 0]);
/// ```
pub fn bucket_sort(nums: &mut [i32]) {
    if nums.len() < 2 {
        return;
    }

    let number_of_buckets = calculate_bucket_count(nums.len());

    let mut buckets = Vec::<Bucket>::new();

    distribute_to_buckets(nums, &mut buckets, number_of_buckets);

    merge_buckets(nums, buckets);

    fn calculate_bucket_count(len: usize) -> i32 {
        match len {
            2..=20 => (SMALL_SIZE_BUCKETS * len as f32).ceil() as i32,
            21..=1000 => (MEDIUM_SIZE_BUCKETS * len as f32).ceil() as i32,
            _ => (LARGE_SIZE_BUCKETS * len as f32).ceil() as i32,
        }
    }

    fn distribute_to_buckets(nums: &mut [i32], buckets: &mut Vec<Bucket>, number_of_buckets: i32) {
        for &val in nums.iter() {
            let bucket_index = hasher!(val, number_of_buckets);
            match buckets.binary_search_by(|bucket| bucket.hasher.cmp(&bucket_index)) {
                Ok(i) => buckets[i].add(val),
                Err(i) => buckets.insert(i, Bucket::with_value(bucket_index, val)),
            }
        }
    }

    fn merge_buckets(nums: &mut [i32], buckets: Vec<Bucket>) {
        let sorted_values = buckets
            .into_iter()
            .flat_map(|mut bucket| {
                match bucket.values.len() {
                    2..=20 => insertion_sort(bucket.values.as_mut()),
                    21..=1000 => shell_sort(bucket.values.as_mut()),
                    _ => quick_sort(nums, 0, bucket.values.len() - 1),
                }
                bucket.values
            })
            .collect::<Vec<i32>>();

        nums.copy_from_slice(&sorted_values);
    }
}
