use color_eyre::eyre::{eyre, Report, Result, WrapErr};
use itertools::Itertools;
use log::info;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Day 1 - Sum of calibration values.
/// 
/// On each line of the input, the calibration value can be found by combining
/// the first digit and the last digit (in that order) to form a single 
/// two-digit number.
pub fn run() -> Result<(), Report> {

    let mut total: usize = 0;

    // parse the puzzle input into lines
    let input = "data/d1.txt";
    let file = File::open(input).wrap_err_with(|| eyre!("Failed to read file: {input:?}"))?;
    let lines = BufReader::new(file).lines().flatten();

    for line in lines {
        // extract characters that are numbers
        let numbers = line.chars().filter(|c| c.is_numeric()).collect_vec();

        // combine first and last numeric char to form a two-digit number
        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();
        let digit: usize = format!("{first}{last}").parse()?;

        // add digit to total sum
        total += digit;
    }

    info!("Answer: {total}");

    Ok(())
}
