use crate::day::Part;
use crate::utils;

use color_eyre::eyre::{Report, Result};
use itertools::Itertools;
use log::info;

/// Day 1 - Sum of calibration values.
///
/// Part 1. On each line of the input, the calibration value can be found by combining
/// the first digit and the last digit (in that order) to form a single
/// two-digit number.
///
/// Part 2. It looks like some of the digits are actually spelled out with letters:
/// one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".
/// Be warned, names can overlap! Ex. oneight should be both "one" and "eight"
pub fn run(part: &Part) -> Result<usize, Report> {
    let digits = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    // read in the input (calibration document)
    let content = utils::read_to_string("data/day_1.txt")?;
    let lines = content.split('\n').collect_vec();

    let mut total = 0;

    for line in lines {
        let mut search = Vec::new();

        // for both part 1 and part 2 we search by digit
        for (_n, d) in &digits {
            let mut digit_search = line.match_indices(d).collect_vec();
            search.append(&mut digit_search);
        }

        // in part two, we also search by name ("one" => "1")
        if *part == Part::Part2 {
            for (n, d) in &digits {
                let mut name_search = line.match_indices(n).map(|(i, _n)| (i, *d)).collect_vec();
                search.append(&mut name_search);
            }
        }

        // combine first and last numbers into digit
        let first = search.iter().min_by(|a, b| a.0.cmp(&b.0)).map(|(_i, d)| d).unwrap();
        let last = search.iter().max_by(|a, b| a.0.cmp(&b.0)).map(|(_i, d)| d).unwrap();
        let digit: usize = format!("{first}{last}").parse()?;

        total += digit;
    }

    info!("Answer: {total}");

    Ok(total)
}

#[test]
fn part_1() -> Result<(), Report> {
    let expected = 53194;
    let observed = run(&Part::Part1)?;
    assert_eq!(observed, expected);
    Ok(())
}

#[test]
fn part_2() -> Result<(), Report> {
    let expected = 54249;
    let observed = run(&Part::Part2)?;
    assert_eq!(observed, expected);
    Ok(())
}
