use hash_map::hash_map::HashMap;
// --- region: imports
use searching::{
    sequential_search::{
        ordered_sequential_search, 
        sequential_search
    },
    binary_search::{
        binary_search, 
        recursive_binary_search
    }, 
    exponential_search::exponential_search, 
    interpolation_search::interpolation_search, 
    hash_search::hash_search, 
};
use std::time::Instant;
// --- endregion: imports

fn main() {
    let nums = [
        1, 2, 3, 4, 6, 7, 8, 9, 10, 15, 16, 17, 19, 21, 23, 24, 27, 28, 29, 30, 32, 35, 38, 39, 42,
        45, 56, 57, 58, 59, 61, 69, 73, 75,
    ];
    let target = 56;

    // SEQUENTIAL SEARCH
    println!("\n\n***SEQUENTIAL SEARCH***");
    let start = Instant::now();
    let sequential_search_result = sequential_search(&nums, target);
    println!(
        ">sequential search - sequential_search_result: {:?}",
        sequential_search_result
    );
    let duration = start.elapsed();
    println!("-->Time: {:?} ns", duration.as_nanos());

    let start = Instant::now();
    let ordered_sequential_search_result = ordered_sequential_search(&nums, target);
    println!(
        ">sequential search - ordered_sequential_search_result: {:?}",
        ordered_sequential_search_result
    );
    let duration = start.elapsed();
    println!("-->Time: {:?} ns", duration.as_nanos());

    // BINARY SEARCH
    println!("\n\n***BINARY SEARCH***");
    let start = Instant::now();
    let binary_search_result = binary_search(&nums, target);
    println!(
        ">binary search - binary_search_result: {:?}",
        binary_search_result
    );
    let duration = start.elapsed();
    println!("-->Time: {:?} ns", duration.as_nanos());

    let recursive_binary_search_result = recursive_binary_search(&nums, target);
    println!(
        ">binary search - recursive_binary_search_result: {:?}",
        recursive_binary_search_result
    );
    let duration = start.elapsed();
    println!("-->Time: {:?} ns", duration.as_nanos());

    // INTERPOLATION SEARCH
    println!("\n\n***INTERPOLATION SEARCH***");
    let start = Instant::now();
    let interpolation_search_result = interpolation_search(&nums, target);
    println!(
        ">interpolation search - interpolation_search_result: {:?}",
        interpolation_search_result
    );
    let duration = start.elapsed();
    println!("-->Time: {:?} ns", duration.as_nanos());

    // EXPONENTIAL SEARCH
    println!("\n\n***EXPONENTIAL SEARCH***");
    let start = Instant::now();
    let exponential_search_result = exponential_search(&nums, target);
    println!(
        ">exponential search - exponential_search_result: {:?}",
        exponential_search_result
    );
    let duration = start.elapsed();
    println!("-->Time: {:?} ns", duration.as_nanos());

    // HASH SEARCH
    println!("\n\n***HASH SEARCH***");
    let mut hash_map = HashMap::new(nums.len());
    for (index, &num) in nums.iter().enumerate() {
        hash_map.insert(&num, &index);
    }
    
    let start = Instant::now();
    let hash_search_result = hash_search(&hash_map, target);
    println!(
        "> Hash search - hash_search_result: {:?}",
        hash_search_result
    );
    let duration = start.elapsed();
    println!("--> Time: {:?} ns", duration.as_nanos());
}
