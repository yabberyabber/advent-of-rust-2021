// Parse standard in as a sequence of integers.
fn input_ints() -> Vec<i32> {
	use std::io::prelude::*;
	std::io::stdin().lock().lines().map(
		|line| line.unwrap().trim().parse::<i32>().unwrap()
	).collect()
}

// Sum inputs in windows of 3.
fn boxcar_sum(signal : Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for n in 0..(signal.len() - 2) {
        result.push(signal[n] + signal[n+1] + signal[n+2])
    }
    result
}

// Count how many subsequent inputs are increasing.
fn count_increasing_depths() -> i32 {
	let depths = boxcar_sum(input_ints());
	let mut prev = depths[0];
	let mut increases_count = 0;
	for depth in depths.iter() {
		if *depth > prev {
			increases_count += 1;
		}
		prev = *depth;
	}
	increases_count
}

fn main() {
	println!("{}", count_increasing_depths());
}
