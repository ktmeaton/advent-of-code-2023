use crate::day::Part;
use crate::utils;

use aho_corasick::AhoCorasick;
use color_eyre::eyre::{Report, Result};
use itertools::Itertools;
//use log::debug;
use log::info;
use std::collections::BTreeMap;

/// Day X
pub fn run(_part: &Part) -> Result<usize, Report> {
    // read in puzzle input, standardized for easy splitting
    let input = utils::read_to_string("data/day_8.txt")?;
    let ac = AhoCorasick::builder().build(["(", ")", ",", "= "]).unwrap();
    let input = ac.replace_all(&input, &["", "", "", ""]);
    let lines = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.split(' ').collect_vec())
        .collect_vec();

    // The first line is the directions, convert L=> 0, R=> 1
    let directions = lines[0][0]
        .chars()
        .map(|c| match c {
            'L' => 0,
            _ => 1,
        })
        .collect_vec();

    // The remaining lines are the travel options
    let travel: BTreeMap<_, _> = lines[1..].iter().map(|v| (v[0], vec![v[1], v[2]])).collect();

    let mut current = "AAA";
    let destination = "ZZZ";
    let mut steps = 0;

    for d in directions.into_iter().cycle() {
        current = travel.get(current).unwrap()[d];
        steps += 1;
        if current == destination {
            break;
        }
    }

    let result = steps;

    info!("Answer: {result}");
    Ok(result)
}

#[test]
fn part_1() -> Result<(), Report> {
    let expected = 20569;
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
