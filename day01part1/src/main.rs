//! Advent of Code Day One Part One

use lib::{read_two_columns, Result};

fn main() -> Result<()> {
    // Load the data
    let mut columns = read_two_columns()?;
    // Sort them then use zip to create a single vector of (a, b) tuples from the first vector. Calculate the
    // absolute difference between the two. Iterate through the difference vector summing the differences to
    // reach the final result.
    columns.0.sort();
    columns.1.sort();
    println!(
        "{}",
        columns
            .0
            .iter()
            .zip(columns.1.iter())
            .map(|(a, b)| a.abs_diff(*b))
            .collect::<Vec<u32>>()
            .into_iter()
            .sum::<u32>()
    );
    Ok(())
}
