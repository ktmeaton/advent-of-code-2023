use crate::day::Part;
use crate::utils;

use color_eyre::eyre::{Report, Result};
use core::ops::RangeInclusive;
use itertools::{iproduct, Itertools};
use log::debug;
use log::info;
use onig::Regex;

/// Day 12 - Hot Springs
pub fn run(_part: &Part) -> Result<usize, Report> {
    let input = utils::read_to_string("data/day_12.txt")?;
    let result = input
        .split('\n')
        .enumerate()
        .map(|(i, l)| {
            let split = l.split(' ').collect_vec();
            let operational =
                split[1].split(',').filter_map(|s| s.parse::<usize>().ok()).collect_vec();
            let springs = split[0];
            debug!("i: {i}, springs: {springs:?}, operational: {operational:?}");
            let local = local_locations(springs, &operational);
            debug!("\tlocal:");
            local.iter().for_each(|l| {
                debug!("\t\t{l:?}");
            });
            let known = springs
                .chars()
                .enumerate()
                .filter_map(|(i, c)| (c == '#').then_some(i))
                .collect_vec();
            let global = global_locations(&local, &known);
            debug!("\tglobal: {}", global.len());
            debug!("\t\toriginal: {springs}");
            global.iter().for_each(|pos| {
                let mut solution = vec!['.'; springs.len()];
                pos.iter().for_each(|r| {
                    r.clone().for_each(|i| {
                        solution[i] = '#';
                    });
                });
                debug!("\t\tsolution: {}", solution.iter().join(""));
            });
            global.len()
        })
        .sum();

    info!("Answer: {result}");
    Ok(result)
}

pub fn local_locations(springs: &str, operational: &[usize]) -> Vec<Vec<RangeInclusive<usize>>> {
    operational
        .iter()
        .enumerate()
        .map(|(i, o)| {
            //debug!("i: {i}, o: {o}");
            // find the operational springs '#' or '?'
            // a specific number of times consecutive (o)
            // negative lookahead and negative lookbehind for '#'
            let mut patterns = format!("[#|\\?]{{{o}}}");
            // no operational spring directly before or after
            patterns = "(?<!#)".to_string() + &patterns + "(?!\\#)";
            // At the beginning, no springs may come before
            patterns = if i == 0 {
                "(?<!.*#.*)".to_string() + &patterns
            }
            // N spring(s) must come before
            else {
                let before = i + operational[0..i].iter().sum::<usize>();
                format!("(?<=.{{{before},}})") + &patterns
            };

            // At the end, no springs may come after
            patterns = if i == operational.len() - 1 {
                patterns + "(?!.*#.*)"
            }
            // N springs must come after
            else {
                let after =
                    (operational.len() - 1 - i) + operational[i + 1..].iter().sum::<usize>();
                patterns + format!("(?=.{{{after},}})").as_str()
            };

            // wrap full expression in a lookahead, to get overlaps
            patterns = "(?=".to_string() + &patterns + ")";

            //debug!("\tpatterns: {patterns:?}");
            // construct the regular expression to find these patterns
            let re = Regex::new(&patterns).unwrap();
            let matches = re.find_iter(springs);
            let positions = matches
                // convert to vector (6..9) = [6,7,8,9]
                .map(|m| (m.0..m.0 + o).collect_vec())
                // split into sliding windows: [6,7,8], [7,8,9]
                .flat_map(|v| v.windows(*o).map(|o| o.to_owned()).collect_vec())
                // convert sliding windows to range inclusive
                .map(|v| v.iter().min().copied().unwrap()..=v.iter().max().copied().unwrap())
                //.inspect(|v| debug!("\tv: {v:?}"))
                .unique()
                .collect_vec();
            positions
        })
        .collect()
}

/// Global spring locations.
///
/// Combinations,
///     [1..=1, 5..=5, 10..=12],
///     [1..=1, 6..=6, 10..=12],
///     [2..=2, 5..=5, 10..=12],
///     [2..=2, 6..=6, 10..=12],
pub fn global_locations(
    local: &[Vec<RangeInclusive<usize>>],
    known: &[usize],
) -> Vec<Vec<RangeInclusive<usize>>> {
    // check location of known springs, these must be acounted for
    let mut combinations = Vec::new();

    for i in 0..local.len() {
        if combinations.is_empty() {
            combinations = local[0].iter().cloned().map(|pos| vec![pos]).collect_vec();
            continue;
        }

        let pos = &local[i];

        combinations = iproduct!(combinations.clone(), pos)
            .filter(|(curr, new)| curr.last().unwrap().end() + 1 < *new.start())
            .map(|(mut curr, new)| {
                curr.push(new.to_owned());
                curr
            })
            .collect_vec();
    }

    // make sure all known springs were found
    combinations
        .into_iter()
        .filter(|c| {
            let k = known.iter().filter_map(|i| c.iter().find(|r| r.contains(i))).collect_vec();
            k.len() == known.len()
        })
        .collect_vec()
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
    let expected = 7633;
    let observed = run(&Part::Part2)?;
    assert_eq!(observed, expected);
    Ok(())
}
