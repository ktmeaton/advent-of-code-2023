use crate::day::Part;
use color_eyre::eyre::{Report, Result};

/// Day X
pub fn run(part: &Part) -> Result<usize, Report> {
    let result = if part == &Part::Part1 { 1 } else { 2 };
    Ok(result)
}
//#[test]
#[allow(dead_code)]
fn part_1() -> Result<(), Report> {
    let expected = 0;
    let observed = run(&Part::Part1)?;
    assert_eq!(observed, expected);
    Ok(())
}

#[allow(dead_code)]
//#[test]
fn part_2() -> Result<(), Report> {
    let expected = 0;
    let observed = run(&Part::Part2)?;
    assert_eq!(observed, expected);
    Ok(())
}