//! Common code

use std::{
    error,
    io::{self, BufRead},
};

/// Generic error type
type Error = Box<dyn error::Error>;

/// Custom result
pub type Result<T> = std::result::Result<T, Error>;

/// Read the data from stdin into two vectors with the first column of numbers placed into the first vector
/// and the second column into the second vector. The vectors are then returned as a tuple.
pub fn read_two_columns() -> Result<(Vec<u32>, Vec<u32>)> {
    let mut column_1: Vec<u32> = Vec::new();
    let mut column_2: Vec<u32> = Vec::new();
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

/// Read the data from stdin row-by-row. Return a vector of vectors where the inner vector contains
/// all the values read in from a single row separated by whitespace and the outer vector contains
/// one of these inner vectors for each row.
pub fn read_rows() -> Result<Vec<Vec<u32>>> {
    let mut outer_vec: Vec<Vec<u32>> = Vec::new();
    for (row_index, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        let mut inner_vec: Vec<u32> = Vec::new();
        for (column_index, value) in line.split_whitespace().enumerate() {
            match value.parse::<u32>() {
                Ok(value) => inner_vec.push(value),
                Err(e) => return Err(format!("row {row_index}, column {column_index}: unable to parse input value {value}: {e}").into()),
            }
        }
        outer_vec.push(inner_vec);
    }
    Ok(outer_vec)
}
