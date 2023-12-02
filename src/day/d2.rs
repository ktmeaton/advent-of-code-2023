use aho_corasick::AhoCorasick;
use color_eyre::eyre::{Report, Result};
use itertools::Itertools;
use log::{debug, info};
use std::collections::BTreeMap;

/// Part 1. Colorful Cubes
/// 
/// The Elf will reach into the bag, grab a handful of random cubes, show 
/// them to you, and then put them back in the bag. He'll do this a few 
/// times per game.
pub fn run() -> Result<(), Report> {

    // part 1 hypothesis, max cube counts for a possible game
    let hypothesis: BTreeMap<&str, usize> = vec![ ("red", 12), ("green", 13), ("blue", 14)].into_iter().collect();

    // read in the input, remove delimiter chars other than space
    // parse into lines: "Game 1: 2 green, 6 blue, ..."   
    // parse into space delimited lists ["Game", "1", "2", "green", ...]
    let document = std::fs::read_to_string("data/d2.txt")?;
    let ac = AhoCorasick::builder().build([":", ",", ";"]).unwrap();
    let content = ac.replace_all(&document, &["", "", ""]);   
    let lines = content.split("\n").collect_vec();
    let lines_split = lines.iter().map(|line| line.split(" ").collect_vec()).collect_vec();

    let mut possible_games = 0;
    let mut power_sum = 0;

    for line in lines_split {

        // parse the game ID (Game 1 => 1)
        let id: usize = line[1].parse()?;

        let mut possible = true;
        let mut max_counts: BTreeMap<&str, usize> = BTreeMap::new();

        let observations = (2..line.len()).step_by(2)
            .into_iter()
            .map(|i| {
                let cube = line[i+1];
                let observed: usize = line[i].parse().unwrap();
                let expected = hypothesis.get(&cube).unwrap();

                // check part 1, possible game
                if observed > *expected { possible = false }
                // check part 2, max counts
                let max = max_counts.entry(cube).or_insert(observed);
                if observed > *max { max_counts.insert(cube, observed); }
                (cube, observed)
            })
            .collect_vec();

        // check part 2 power (max counts multiple)
        let mut power: usize = 1;
        max_counts.iter().for_each(|(_cube, count)| power = power * count);
        power_sum += power;

        debug!("Game {id}; possible: {possible}, power: {power}, max: {max_counts:?}, {observations:?}");

        if possible { possible_games += id }

    }

    info!("Part 1: {possible_games}");
    info!("Part 2: {power_sum}");

    Ok(())
}