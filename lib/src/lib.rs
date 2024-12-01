//! Common code

use std::{
    error,
    io::{self, BufRead},
};

type Error = Box<dyn error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

/// Read the data from stdin into two vectors with the first column of numbers placed into the first vector
/// and the second column into the second vector. The vectors are then returned as a tuple.
pub fn read_two_columns() -> Result<(Vec<u32>, Vec<u32>)> {
    let mut column_1: Vec<u32> = Vec::new();
    let mut column_2: Vec<u32> = Vec::new();
    // Read the data into two vectors with the first column of numbers placed into the first vector and the
    // second column into the second vector
    for line in io::stdin().lock().lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        if let (Some(first), Some(second)) = (parts.next(), parts.next()) {
            if let (Ok(first_num), Ok(second_num)) = (first.parse::<u32>(), second.parse::<u32>()) {
                column_1.push(first_num);
                column_2.push(second_num);
            }
        }
    }
    Ok((column_1, column_2))
}
