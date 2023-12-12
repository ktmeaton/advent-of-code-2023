//use crate::almanac::Component;
use crate::day::Part;

use color_eyre::eyre::{Report, Result};
use log::info;
//use petgraph::graph::Graph;
//use std::str::FromStr;

/// Day 5
///
/// The Almanac contains mappings of seed => soil => fertilizer ... => location
///
pub fn run(part: &Part) -> Result<usize, Report> {
    //let mut graph: Graph<(Component, usize), usize> = Graph::new();

    let result = if part == &Part::Part1 { 1 } else { 2 };
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
