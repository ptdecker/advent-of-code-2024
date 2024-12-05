//! Advent of Code Day Two Part Two

use lib::{read_rows, Result};

use crate::Direction::NotEstablished;

/// The direction the report vector is going
#[derive(Default)]
enum Direction {
    #[default]
    NotEstablished,
    Down,
    Up,
}

/// Check to see if the "report" (represented by the row data vector) is safe
fn safety_check(row_data: &Vec<u32>) -> bool {
    let mut direction: Direction = NotEstablished;
    let mut prior_sentinel: Option<&u32> = None;
    for value in row_data {
        if let Some(prior) = prior_sentinel {
            // Make sure were going in the same direction. Or, if we haven't established direction,
            // then establish it.
            match direction {
                // If we haven't established a direction yet, establish it
                NotEstablished if value > &prior => direction = Direction::Up,
                NotEstablished if value < &prior => direction = Direction::Down,
                // If we're going in the same direction as before, then pass
                Direction::Down if value < &prior => (),
                Direction::Up if value > &prior => (),
                // In any other case, we aren't safe and can short-circuit fail.
                _ => return false,
            }
            // We checked above to see if we were moving at least once. But, we still need to make
            // sure we do not jump by more than three.
            if value.abs_diff(*prior) > 3 {
                return false;
            }
        }
        prior_sentinel = Some(&value);
    }
    true
}

// Credit for this function goes to [rafald1](https://github.com/rafald1/advent_of_code_2024/blob/0e9d7b2ba4723080b738e85436a3fcba7e63e549/src/day_02_red_nosed_reports/part_2.rs)
fn level_combinations_without_one(report: &[u32]) -> impl Iterator<Item = Vec<u32>> + '_ {
    (0..report.len()).map(move |i| [&report[..i], &report[i + 1..]].concat())
}

fn problem_damper(row_data: &Vec<u32>) -> bool {
    if safety_check(row_data) {
        return true;
    }
    // The whole report is unsafe. So now check the report again, but iterate through dropping each
    // report in turn then checking the modified report that dropped one of the values
    for level_combo in level_combinations_without_one(row_data.as_slice()) {
        if safety_check(&level_combo) {
            return true;
        }
    }
    false
}

fn main() -> Result<()> {
    println!(
        "{}",
        read_rows()?
            .into_iter()
            // convert the row of the report into a true or false value
            .map(|row| problem_damper(&row))
            // `safety_result as u32` gives u32 value of 0 to false and 1 to true
            .map(|safety_result| safety_result as u32)
            .sum::<u32>()
    );
    Ok(())
}
