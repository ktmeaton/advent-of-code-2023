use crate::day::Part;
// use crate::utils;
// use itertools::Itertools;
// use log::debug;
use color_eyre::eyre::{Report, Result};
use log::info;

/// Day X
pub fn run(part: &Part) -> Result<usize, Report> {
    let result = match *part {
        Part::Part1 => 1,
        Part::Part2 => 2,
    };

    info!("Answer: {result}");
    Ok(result)
}

#[test]
fn part_1() -> Result<(), Report> {
    let expected = 1;
    let observed = run(&Part::Part1)?;
    assert_eq!(observed, expected);
    Ok(())
}

#[test]
fn part_2() -> Result<(), Report> {
    let expected = 2;
    let observed = run(&Part::Part2)?;
    assert_eq!(observed, expected);
    Ok(())
}
