//! Advent of Code Day One Part Two

use lib::{read_two_columns, Result};

fn main() -> Result<()> {
    // Load the data
    let columns = read_two_columns()?;
    // Iterate through the second vector. If the value exists in the first vector, then increment the
    // frequency counter for that value; otherwise, ignore the second vector number.
    let mut frequency: Vec<usize> = Vec::new();
    for num in columns.0.iter() {
        frequency.push(columns.1.iter().filter(|x| **x == *num).count())
    }
    // Calculate the answer by zipping the first column values together with their frequencies then iterating
    // over them multiplying the two together. Finally, calculate the summation of the total.
    println!(
        "{}",
        columns
            .0
            .iter()
            .zip(frequency.iter())
            .map(|(a, b)| *a * *b as u32)
            .collect::<Vec<u32>>()
            .into_iter()
            .sum::<u32>()
    );
    Ok(())
}
