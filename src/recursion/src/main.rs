use std::time::Instant;

use recursion::{
    fibonacci::{dp_fibonacci_bottom_up, dp_fibonacci_top_down, fibonacci},
    make_change_with_least_coin::{
        dp_make_change_with_least_coin, make_change_with_least_coin, mm_make_change_with_least_coin,
    },
    num_to_string::num_to_string,
    sum_of_nums::{sum_of_nums, tail_sum_of_nums},
    tower_of_hanoi::tower_of_hanoi,
};
use stack::stack::Stack;
use vec::linked_vec;

fn main() {
    // SUM OF NUMBERS
    println!("\n\n***SUM OF NUMBERS***");
    let nums = [1, 3, 9, 2, 4, 5];

    let rc_sum = sum_of_nums(&nums);
    println!(">recursion - sum of numbers {:?}", rc_sum);

    let tail_rc_sum = tail_sum_of_nums(0, &nums);
    println!(">tail recursion - sum of numbers {:?}", tail_rc_sum);

    // NUMBER TO STRING
    println!("\n\n***NUMBER TO STRING***");
    let num = 97794334;
    let b_num = num_to_string(num, 2);
    let h_num = num_to_string(num, 16);
    println!("{} to binary is {:?}", num, "b".to_string() + &b_num);
    println!("{} to hexadecimal is {:?}", num, "0x".to_string() + &h_num);

    // TOWER OF HANOI
    println!("\n\n***TOWER OF HANOI***");
    let mut src_peg = Stack::new();
    src_peg.push(1);
    src_peg.push(2);
    src_peg.push(3);
    src_peg.push(4);
    src_peg.push(5);

    let mut mid_peg = Stack::new();
    let mut des_peg = Stack::new();
    println!("- before:");
    println!("source peg: {:?}", src_peg);
    println!("destination peg: {:?}", des_peg);

    tower_of_hanoi(
        src_peg.size() as u32,
        &mut src_peg,
        &mut mid_peg,
        &mut des_peg,
    );

    println!("- after:");
    println!("source peg: {:?}", src_peg);
    println!("destination peg: {:?}", des_peg);

    // FIBONACCI
    println!("\n\n***FIBONACCI***");
    let start = Instant::now();
    println!(">recursion - fibonacci: {}", fibonacci(15));
    let duration = start.elapsed();
    println!("-->Time: {:?} ns", duration.as_nanos());

    let start = Instant::now();
    println!(
        ">dynamic programming - fibonacci (bottom-up): {}",
        dp_fibonacci_bottom_up(15)
    );
    let duration = start.elapsed();
    println!("-->Time: {:?} ns", duration.as_nanos());

    let start = Instant::now();
    println!(
        ">dynamic programming - fibonacci (top-down): {}",
        dp_fibonacci_top_down(15)
    );
    let duration = start.elapsed();
    println!("-->Time: {:?} ns", duration.as_nanos());

    // MINIMUM NUMBER OF COINS TO MAKE CHANGE
    println!("\n\n***MINIMUM NUMBER OF COINS TO MAKE CHANGE***");
    let coins = linked_vec![1, 2, 5, 10, 20];
    let change = 34;

    let start = Instant::now();
    let min_coin = make_change_with_least_coin(&coins, change);
    print!(
        ">recursion - min coin to make {} change is {}",
        change, min_coin
    );
    let duration = start.elapsed();
    print!("\n-->Time: {:?} ns", duration.as_nanos());

    let start = Instant::now();
    let mut min_num_coin_mem = linked_vec![0; (change + 1) as usize];
    let rc_mm_min_coin = mm_make_change_with_least_coin(&coins, change, &mut min_num_coin_mem);
    print!(
        "\n>recursion & memorization - min coin to make {} change is {}",
        change, rc_mm_min_coin
    );
    let duration = start.elapsed();
    print!("\n-->Time: {:?} ns", duration.as_nanos());

    let start = Instant::now();
    let dp_min_coin = dp_make_change_with_least_coin(&coins, change);
    print!(
        "\n>dynamic programming - min coin to make {} change is {}",
        change, dp_min_coin
    );
    let duration = start.elapsed();
    println!("\n-->Time: {:?} ns", duration.as_nanos());
}
