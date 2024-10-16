use std::time::Instant;

use sorting::{
    bubble_sort::{bubble_sort, bubble_sort2, cocktail_sort, comb_sort, simplest_sort},
    bucket_sort::bucket_sort,
    counting_sort::counting_sort,
    heap_sort::heap_sort,
    insertion_sort::{binary_insertion_sort, insertion_sort},
    merge_sort::merge_sort,
    quick_sort::quick_sort,
    selection_sort::{bidirectional_selection_sort, selection_sort},
    shell_sort::shell_sort,
};

fn main() {
    let nums: Vec<i32> = vec![
        2, 4, 7, 8, 23, 19, 16, 14, 13, 12, 10, 20, 18, 17, 15, 11, 9, -1, 5, 6, 1, 3, 21, 40, 22,
        39, 38, 37, 36, 35, 34, 33, 24, 30, 31, 32, 25, 26, 27, 28, 29, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 80, 79, 78, 77, 76, 75, 74, 73, 72, 71,
        70, 61, 62, 63, 64, 65, 66, 67, 68, 69, 95, 94, 93, 92, 91, 90, 85, 82, 83, 84, 81, 86, 87,
        88, 89,
    ];

    println!("{:<30} {:<10} {:>10}", "Algorithm", "Status", "Time (ns)");
    println!("---------------------------------------------------");

    let mut test_nums = nums.clone();
    let start = Instant::now();
    bubble_sort(&mut test_nums);
    print_sorting_results("Bubble Sort 1", start.elapsed(), is_sorted(&test_nums));

    let mut test_nums = nums.clone();
    let start = Instant::now();
    bubble_sort2(&mut test_nums);
    print_sorting_results("Bubble Sort 2", start.elapsed(), is_sorted(&test_nums));

    let mut test_nums = nums.clone();
    let start = Instant::now();
    cocktail_sort(&mut test_nums);
    print_sorting_results("Cocktail Sort", start.elapsed(), is_sorted(&test_nums));

    let mut test_nums = nums.clone();
    let start = Instant::now();
    comb_sort(&mut test_nums);
    print_sorting_results("Comb Sort", start.elapsed(), is_sorted(&test_nums));

    let mut test_nums = nums.clone();
    let start = Instant::now();
    simplest_sort(&mut test_nums);
    print_sorting_results("Simplest Sort", start.elapsed(), is_sorted(&test_nums));

    let mut test_nums = nums.clone();
    let start = Instant::now();
    let len = test_nums.len();
    quick_sort(&mut test_nums, 0, len - 1);
    print_sorting_results("Quick Sort", start.elapsed(), is_sorted(&test_nums));

    let mut test_nums = nums.clone();
    let start = Instant::now();
    insertion_sort(&mut test_nums);
    print_sorting_results("Insertion Sort", start.elapsed(), is_sorted(&test_nums));

    let mut test_nums = nums.clone();
    let start = Instant::now();
    binary_insertion_sort(&mut test_nums);
    print_sorting_results(
        "Binary Insertion Sort",
        start.elapsed(),
        is_sorted(&test_nums),
    );

    let mut test_nums = nums.clone();
    let start = Instant::now();
    shell_sort(&mut test_nums);
    print_sorting_results("Shell Sort", start.elapsed(), is_sorted(&test_nums));

    let mut test_nums = nums.clone();
    let start = Instant::now();
    merge_sort(&mut test_nums);
    print_sorting_results("Merge Sort", start.elapsed(), is_sorted(&test_nums));

    let mut test_nums = nums.clone();
    let start = Instant::now();
    selection_sort(&mut test_nums);
    print_sorting_results("Selection Sort", start.elapsed(), is_sorted(&test_nums));

    let mut test_nums = nums.clone();
    let start = Instant::now();
    bidirectional_selection_sort(&mut test_nums);
    print_sorting_results(
        "Bidirectional Selection Sort",
        start.elapsed(),
        is_sorted(&test_nums),
    );

    let mut test_nums = nums.clone();
    let start = Instant::now();
    heap_sort(&mut test_nums);
    print_sorting_results("Heap Sort", start.elapsed(), is_sorted(&test_nums));

    let mut test_nums = nums.clone();
    let start = Instant::now();
    bucket_sort(&mut test_nums);
    print_sorting_results("Bucket Sort", start.elapsed(), is_sorted(&test_nums));

    let mut test_nums = nums.clone();
    let start = Instant::now();
    counting_sort(&mut test_nums);
    print_sorting_results("Counting Sort", start.elapsed(), is_sorted(&test_nums));
}

fn print_sorting_results(name: &str, duration: std::time::Duration, sorted: bool) {
    println!(
        "{:<30} {:<10} {:>6} ns",
        name,
        if sorted { "Sorted" } else { "Not Sorted" },
        duration.as_nanos()
    );
}

fn is_sorted(nums: &[i32]) -> bool {
    let mut sorted = true;

    for i in 0..nums.len() - 1 {
        if nums[i] > nums[i + 1] {
            sorted = false;
        }
    }

    sorted
}
