use std::time::Instant;

use sorting::{
    bubble_sort::{bubble_sort, bubble_sort2, cocktail_sort, comb_sort, simplest_sort},
    bucket_sort::bucket_sort,
    counting_sort::counting_sort,
    heap_sort::heap_sort,
    insertion_sort::{binary_insertion_sort, insertion_sort},
    merge_sort::merge_sort,
    quick_sort::quick_sort,
    radix_sort::radix_sort,
    selection_sort::{bidirectional_selection_sort, selection_sort},
    shell_sort::shell_sort,
    tim_sort::{balanced_tim_sort::balanced_tim_sort, tim_sort},
};

fn main() {
    let nums: Vec<i32> = generate_random_data(1000);

    println!("{:<30} {:<10} {:>13}", "Algorithm", "Status", "Time (ns)");
    println!("-------------------------------------------------------");

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

    let mut test_nums = nums.clone();
    let start = Instant::now();
    radix_sort(&mut test_nums);
    print_sorting_results("Radix Sort", start.elapsed(), is_sorted(&test_nums));

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
        "{:<30} {:<10} {:>10} ns",
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
