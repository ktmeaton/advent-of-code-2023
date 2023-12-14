use crate::day::Part;
use crate::utils;

use aho_corasick::AhoCorasick;
use color_eyre::eyre::{Report, Result};
use itertools::Itertools;
use log::debug;
use log::info;
use prime_factorization::Factorization;
use std::collections::BTreeMap;

/// Day X
pub fn run(part: &Part) -> Result<usize, Report> {
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

    // everything below is travel nodes
    let mut travel = BTreeMap::new();

    // For part 1, start and end are AAA, ZZZ
    let (mut current, mut destination) = match *part {
        Part::Part1 => (vec!["AAA"], vec!["ZZZ"]),
        Part::Part2 => (vec![], vec![]),
    };

    // For part 2, start is all nodes ending with A
    //             end is all nodes ending with Z
    lines[1..].iter().for_each(|v| {
        travel.insert(v[0], vec![v[1], v[2]]);
        if *part == Part::Part2 {
            v.iter().for_each(|n| {
                if n.ends_with('A') && !current.contains(n) {
                    current.push(n);
                } else if n.ends_with('Z') && !destination.contains(n) {
                    destination.push(n)
                }
            });
        };
    });

    debug!("current: {current:?}");
    debug!("destination: {destination:?}");

    // find the steps from each current to dest, we will find destination by LCM
    let mut steps: BTreeMap<_, _> = current.iter().enumerate().map(|(i, _c)| (i, 0)).collect();

    for (counter, d) in directions.into_iter().enumerate().cycle() {
        current = current
            .iter()
            .enumerate()
            .map(|(i, n)| {
                let c = travel.get(n).unwrap()[d];
                if destination.contains(&c) {
                    steps.insert(i, counter);
                }
                c
            })
            .collect_vec();

        // if we found the destination for all current
        if !steps.values().contains(&0) {
            break;
        };
    }

    // Calculate Lowest Common Multiple (LCM) of steps to destination
    debug!("steps {steps:?}");

    // Least Common Multiple (LCM) by prime factorization and exponents
    let mut prime_factors: BTreeMap<u64, usize> = BTreeMap::new();
    steps.values().map(|v| Factorization::run(*v as u32).factors).for_each(|v| {
        v.iter()
            .map(|f1| {
                let f1 = *f1 as u64;
                let count = v.iter().filter(|f2| f1 == **f2 as u64).count();
                let current = prime_factors.entry(f1).or_insert(count);
                if count > *current {
                    prime_factors.insert(f1, count);
                }
            })
            .count();
    });
    debug!("prime_factors: {prime_factors:?}");

    let mut result = 0;
    prime_factors.into_iter().for_each(|(f, e)| {
        let p = f.pow(e as u32);
        debug!("{f}^{e} {p} {result}");
        match result {
            0 => result = p,
            _ => result *= p,
        };
    });
    let result = result as usize;

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
    let expected = 21366921060721;
    let observed = run(&Part::Part2)?;
    assert_eq!(observed, expected);
    Ok(())
}
