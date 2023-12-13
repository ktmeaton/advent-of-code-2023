use crate::day::Part;
use crate::utils;

use aho_corasick::AhoCorasick;
use color_eyre::eyre::{Report, Result};
use itertools::Itertools;
use log::{debug, info};
use std::collections::BTreeMap;

/// Day 2. Cubes of Power
///
/// The Elf will reach into the bag, grab a handful of random cubes, show
/// them to you, and then put them back in the bag. He'll do this a few
/// times per game.
///
/// Part 1. The Elf would first like to know which games would have been
/// possible if the bag contained only 12 red cubes, 13 green cubes,
/// and 14 blue cubes?
///
/// Part 2. What is the fewest number of cubes of each color that could
/// have been in the bag to make the game possible? The power of a set
/// of cubes is equal to the numbers of red, green, and blue cubes
/// multiplied together. What is the sum of the power of these sets?
pub fn run(part: &Part) -> Result<usize, Report> {
    // part 1 hypothesis, max cube counts for a possible game
    let hypothesis: BTreeMap<&str, usize> =
        vec![("red", 12), ("green", 13), ("blue", 14)].into_iter().collect();

    // read in the input, remove delimiter chars other than space
    // parse into lines: "Game 1: 2 green, 6 blue, ..."
    // parse into space delimited lists ["Game", "1", "2", "green", ...]

    let path = std::path::PathBuf::from("data/day_2.txt");
    let document = utils::read_to_string(&path)?;
    let ac = AhoCorasick::builder().build([":", ",", ";"]).unwrap();
    let content = ac.replace_all(&document, &["", "", ""]);
    let lines = content.split('\n').collect_vec();
    let lines_split = lines.iter().map(|line| line.split(' ').collect_vec()).collect_vec();

    let mut possible_games = 0;
    let mut power_sum = 0;

    for line in lines_split {
        // parse the game ID (Game 1 => 1)
        let id: usize = line[1].parse()?;

        let mut possible = true;
        let mut max_counts: BTreeMap<&str, usize> = BTreeMap::new();

        let observations = (2..line.len())
            .step_by(2)
            .map(|i| {
                let cube = line[i + 1];
                let observed: usize = line[i].parse().unwrap();
                let expected = hypothesis.get(&cube).unwrap();

                // check part 1, possible game
                if observed > *expected {
                    possible = false
                }
                // check part 2, max counts
                let max = max_counts.entry(cube).or_insert(observed);
                if observed > *max {
                    max_counts.insert(cube, observed);
                }
                (cube, observed)
            })
            .collect_vec();

        // check part 1, possible game hypothesis
        if possible {
            possible_games += id
        }

        // check part 2, power (max counts multiple)
        let mut power: usize = 1;
        max_counts.iter().for_each(|(_cube, count)| power *= count);
        power_sum += power;

        debug!("Game {id}; possible: {possible}, power: {power}, max: {max_counts:?}, {observations:?}");
    }

    let answer = match *part {
        Part::Part1 => possible_games,
        Part::Part2 => power_sum,
    };

    info!("Answer: {answer}");
    Ok(answer)
}

#[test]
fn part_1() -> Result<(), Report> {
    let expected = 2076;
    let observed = run(&Part::Part1)?;
    assert_eq!(observed, expected);
    Ok(())
}

#[test]
fn part_2() -> Result<(), Report> {
    let expected = 70950;
    let observed = run(&Part::Part2)?;
    assert_eq!(observed, expected);
    Ok(())
}
