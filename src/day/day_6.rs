use crate::{day::Part, utils};

use color_eyre::eyre::{Report, Result};
use itertools::Itertools;
use log::info;
//use log::debug;
use std::path::PathBuf;

/// Day 6 - Boat Races
pub fn run(_part: &Part) -> Result<usize, Report> {
    // parse puzzle input into times and distances
    let path = PathBuf::from("data/day_6.txt");
    let input = utils::read_to_string(&path)?;
    let lines = input.split('\n').collect_vec();

    let times: Vec<usize> = lines[0].split(' ').filter_map(|c| c.parse().ok()).collect();
    let dists: Vec<usize> = lines[1].split(' ').filter_map(|c| c.parse().ok()).collect();

    let mut result = None;
    times.into_iter().zip(dists).for_each(|(t, d)| {
        // d = h * (t - h) = ht -h^2
        let solutions = (1..t).filter(|h| h * t - h.pow(2) > d).count();
        result = match result {
            Some(r) => Some(r * solutions),
            None => Some(solutions),
        };
    });

    let result = result.unwrap_or_default();

    info!("Answer: {result}");
    Ok(result)
}

#[test]
fn part_1() -> Result<(), Report> {
    let expected = 5133600;
    let observed = run(&Part::Part1)?;
    assert_eq!(observed, expected);
    Ok(())
}

#[test]
fn part_2() -> Result<(), Report> {
    let expected = 5133600;
    let observed = run(&Part::Part2)?;
    assert_eq!(observed, expected);
    Ok(())
}
