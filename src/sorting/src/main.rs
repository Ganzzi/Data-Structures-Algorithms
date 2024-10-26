use std::time::Instant;

use sorting::{
    bubble_sort::{
        bubble_sort, cocktail_sort::cocktail_sort, comb_sort::comb_sort, naive_sort::naive_sort,
        optimized_bubble_sort::optimized_bubble_sort,
    },
    bucket_sort::bucket_sort,
    counting_sort::counting_sort,
    heap_sort::heap_sort,
    insertion_sort::{binary_insertion_sort::binary_insertion_sort, insertion_sort},
    merge_sort::merge_sort,
    quick_sort::quick_sort,
    radix_sort::{radix_sort, radix_sort_by_sign::radix_sort_by_sign},
    selection_sort::{bidirectional_selection_sort::bidirectional_selection_sort, selection_sort},
    shell_sort::shell_sort,
    tim_sort::{balanced_tim_sort::balanced_tim_sort, tim_sort},
};

fn main() {
    let nums: Vec<i32> = generate_random_data(1000);

    println!("{:<30} {:<10} {:>15}", "Algorithm", "Status", "Time (ns)");
    println!("{:-<57}", "");

    let mut test_nums = nums.clone();
    let start = Instant::now();
    bubble_sort(&mut test_nums);
    print_sorting_results("Bubble Sort", start.elapsed(), is_sorted(&test_nums));

    let mut test_nums = nums.clone();
    let start = Instant::now();
    optimized_bubble_sort(&mut test_nums);
    print_sorting_results(
        "Optimized Bubble Sort",
        start.elapsed(),
        is_sorted(&test_nums),
    );

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
    naive_sort(&mut test_nums);
    print_sorting_results("Naive Sort", start.elapsed(), is_sorted(&test_nums));

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

    let mut test_nums = nums.clone();
    let start = Instant::now();
    radix_sort(&mut test_nums);
    print_sorting_results("Radix Sort", start.elapsed(), is_sorted(&test_nums));

    let mut test_nums = nums.clone();
    let start = Instant::now();
    radix_sort_by_sign(&mut test_nums);
    print_sorting_results("Radix Sort by Sign", start.elapsed(), is_sorted(&test_nums));

    let mut test_nums = nums.clone();
    let start = Instant::now();
    tim_sort(&mut test_nums);
    print_sorting_results("Tim Sort", start.elapsed(), is_sorted(&test_nums));

    let mut test_nums = nums.clone();
    let start = Instant::now();
    balanced_tim_sort(&mut test_nums);
    print_sorting_results("Balanced Tim Sort", start.elapsed(), is_sorted(&test_nums));
}

fn generate_random_data(size: usize) -> Vec<i32> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(0..1_000_000)).collect()
}

fn print_sorting_results(name: &str, duration: std::time::Duration, sorted: bool) {
    println!(
        "{:<30} {:<10} {:>12} ns",
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
