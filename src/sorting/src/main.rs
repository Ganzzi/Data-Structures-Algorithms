use std::time::Instant;

use sorting::{
    bubble_sort::{bubble_sort, bubble_sort2},
    cocktail_sort::cocktail_sort,
};

fn main() {
    let nums = [
        45, 56, 57, 69, 73, 75, 1, 4, 6, 7, 8, 9, 10, 15, 16, 17, 2, 3, 19, 21, 23, 24, 27, 28, 29,
    ];

    // BUBBLE SORT
    println!("\n\n***BUBBLE SORT***");
    let mut test_nums = nums.clone();
    let start = Instant::now();
    bubble_sort(&mut test_nums);
    println!(">bubble sort 1: {:?}", test_nums);
    let duration = start.elapsed();
    println!("-->Time: {:?} ns", duration.as_nanos());

    let mut test_nums = nums.clone();
    let start = Instant::now();
    bubble_sort2(&mut test_nums);
    println!(">bubble sort 2: {:?}", test_nums);
    let duration = start.elapsed();
    println!("-->Time: {:?} ns", duration.as_nanos());

    // COCKTAIL SORT
    println!("\n\n***COCKTAIL SORT***");
    let mut test_nums = nums.clone();
    let start = Instant::now();
    cocktail_sort(&mut test_nums);
    println!(">cocktail sort: {:?}", test_nums);
    let duration = start.elapsed();
    println!("-->Time: {:?} ns", duration.as_nanos());
}
