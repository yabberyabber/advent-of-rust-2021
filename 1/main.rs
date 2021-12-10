// Parse standard in as a sequence of integers.
fn input_ints() -> Vec<i32> {
	use std::io::prelude::*;
	std::io::stdin().lock().lines().map(
		|line| line.unwrap().trim().parse::<i32>().unwrap()
	).collect()
}

// Sum inputs in windows of 3.
fn boxcar_sum(signal : Vec<i32>, window : usize) -> Vec<i32> {
    signal.windows(window).map(|slice| slice.iter().sum()).collect()
}

// Count how many subsequent inputs are increasing.
fn count_increasing_depths() -> i32 {
	boxcar_sum(input_ints(), 3,
    ).windows(2).map( // Inspect every pair of elements.
        |window| (window[1] > window[0]) as i32
    ).sum()
}

fn main() {
	println!("{}", count_increasing_depths());
}
