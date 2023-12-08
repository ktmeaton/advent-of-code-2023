use std::collections::BTreeMap;

use crate::day::Part;
use color_eyre::eyre::{Report, Result};

#[derive(Clone, Debug)]
struct Coord {
    pub x: i32,
    pub y: i32,
    pub symbol: Option<char>,
    pub part_number: Option<u32>,
}
/// Day 3
///
/// The engine schematic (your puzzle input) consists of a visual representation
/// of the engine. There are lots of numbers and symbols you don't really
/// understand, but apparently any number adjacent to a symbol, even diagonally,
/// is a "part number" and should be included in your sum. (Periods (.) do not
/// count as a symbol.)
pub fn run(part: &Part) -> Result<usize, Report> {
    let result = if part == &Part::Part1 { 1 } else { 2 };

    // read the puzzle input line by line, char by char
    // store coordinates (x, y)

    let input = "467..114..\n...*......".to_string();
    //let input = std::fs::read_to_string("data/day_3.txt")?;

    Ok(result)
}

#[allow(dead_code)]
fn part_1() -> Result<(), Report> {
    let expected = 0;
    let observed = run(&Part::Part1)?;
    assert_eq!(observed, expected);
    Ok(())
}

#[test]
fn part_2() -> Result<(), Report> {
    let expected = 0;
    let observed = run(&Part::Part2)?;
    assert_eq!(observed, expected);
    Ok(())
}
