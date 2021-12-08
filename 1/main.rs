// Parse standard in as a sequence of integers.
fn input_ints() -> Vec<i32> {
	use std::io::prelude::*;
	std::io::stdin().lock().lines().map(
		|line| line.unwrap().trim().parse::<i32>().unwrap()
	).collect()
}

// Count how many subsequent inputs are increasing.
fn count_increasing_depths() -> i32 {
	let depths = input_ints();
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
