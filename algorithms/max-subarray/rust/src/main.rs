mod max_subarray;

use max_subarray::{brute_force, kadane, SubArrayResult};

fn main() {
    let numbers = vec![0, 1, 2, 3, -4, -5];

    if let Some(SubArrayResult { sum, start, end }) = brute_force(&numbers) {
        println!("Brute force method...");
        println!("Maximum subarray sum: {sum}");
        println!("Array: {numbers:?}");
        println!("Maximum subarray: {:?}", &numbers[start..=end]);
    }

    if let Some(SubArrayResult { sum, start, end }) = kadane(&numbers) {
        println!("Kadane's algorithm...");
        println!("Maximum subarray sum: {sum}");
        println!("Array: {numbers:?}");
        println!("Maximum subarray: {:?}", &numbers[start..=end]);
    }
}
