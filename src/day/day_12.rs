use crate::day::Part;
use crate::utils;

use color_eyre::eyre::{Report, Result};
use core::ops::RangeInclusive;
use itertools::{iproduct, Itertools};
use log::debug;
use log::info;
use onig::Regex;

pub fn local_locations(springs: &str, operational: &[usize]) -> Vec<Vec<RangeInclusive<usize>>> {
    operational
        .iter()
        .enumerate()
        .map(|(i, o)| {
            debug!("i: {i}, o: {o}");

            // find the operational springs '#' or '?'
            // a specific number of times consecutive (o)
            // negative lookahead and negative lookbehind for '#'
            // let mut patterns = format!("(?<!#)([#|\\?]{{{o}}})(?!\\#)");
            // // At the beginning, no spring may come before
            // if i == 0 { patterns = "(?<!.*#.*)".to_string() + &patterns}
            // // In the middle, # must come before and after
            // // if i != 0 && i != operational.len() - 1 {
            // //     patterns = "(?<=.*[\\?|#]+.*)".to_string() + &patterns + "(?=.*[\\?|#]+.*)";
            // // }
            // // At the end, no spring may come after
            // if i == operational.len() - 1 { patterns += "(?!.*#.*)"}

            let mut patterns = format!("[#|\\?]{{{o}}}");
            // no operational spring directly before or after
            patterns = "(?<!#)".to_string() + &patterns + "(?!\\#)";
            // N spring(s) must come before
            if i > 0 {
                let before = i + operational[0..i].iter().sum::<usize>();                
                debug!("\tbefore: {before}");
                patterns = format!("(?<=.{{{before},}})") + &patterns;
            }
            // N springs must come after
            if i < operational.len() - 1 {
                let after = (operational.len() - 2 - i) + operational[i+1..].iter().sum::<usize>();
                patterns += format!("(?=.{{{after},}})").as_str();
            }

            // // At the beginning, no spring may come before
            // if i == 0 {
            //     patterns = "(?<!.*#.*)".to_string() + &patterns
            // }
            // // In the middle, i spring(s) MUST come before AND after
            // if i != 0 && i != operational.len() - 1 {
            //     // numeric filtering
            //     let num_broken_before = operational[0..i].iter().sum::<usize>();
            //     patterns = format!("(?<=.*[\\?|#]{{{num_springs_before},}}.*)")
            //         + &patterns
            //         + "(?=.*[\\?|#]+.*)";
            //         patterns = 
            // }
            // // At the end, no spring may come after
            // if i == operational.len() - 1 {
            //     patterns += "(?!.*#.*)"
            // }

            // wrap full expression in a lookahead, to get overlaps
            patterns = "(?=".to_string() + &patterns + ")";

            debug!("\tpatterns: {patterns:?}");
            // construct the regular expression to find these patterns
            let re = Regex::new(&patterns).unwrap();
            let matches = re.find_iter(&springs);
            let positions = matches
                // convert to vector (6..9) = [6,7,8,9]
                .map(|m| (m.0..m.0 + o).collect_vec())
                // split into sliding windows: [6,7,8], [7,8,9]
                .flat_map(|v| v.windows(*o).map(|o| o.to_owned()).collect_vec())
                // convert sliding windows to range inclusive
                .map(|v| v.iter().min().copied().unwrap()..=v.iter().max().copied().unwrap())
                .inspect(|v| debug!("\tv: {v:?}"))
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
pub fn global_locations(local: &[Vec<RangeInclusive<usize>>]) -> Vec<Vec<RangeInclusive<usize>>> {
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

        debug!("{i} combinations:");
        combinations.iter().for_each(|c| {
            debug!("\tcombination: {c:?}");
        });
    }

    combinations.iter().for_each(|c| {
        debug!("final combination: {c:?}");
    });

    combinations
}
/// Day 12 - Hot Springs
///
/// Part 1
///
/// Springs:     ".??..??...?##."
/// Operational: [1, 1, 3]
/// Valid Local Locations:
///   1: [1], [2], [5], [6] = 4 locations
///   1: [1], [2], [6], [6] = 4 locations
///   3: [10, 11, 12] = 1 locations
/// Valid Global Locations:
///   [1], [5], [10,11,12]
///   [2], [6], [10,11,12]
pub fn run(part: &Part) -> Result<usize, Report> {
    // let input = utils::read_to_string("data/day_12.txt")?;
    // let result = input.split("\n").map(|l| {
    //     let split = l.split(" ").collect_vec();
    //     let operational = split[1].split(",").filter_map(|s| s.parse::<usize>().ok()).collect_vec();
    //     let springs = split[0];
    //     debug!("springs: {springs:?}, operational: {operational:?}");
    //     let local = local_locations(&springs, &operational);
    //     debug!("\tlocal: {local:?}");
    //     let global = global_locations(&local);
    //     debug!("\tglobal: {}", global.len());
    //     global.len()
    // }).sum();

    let springs = "?#?#?#?#?#?#?#?";
    let operational = [1, 3, 1, 6];
    // let springs = "?#?#?#?#";
    // let operational = [1, 3, 1];
    debug!("springs: {springs:?}, operational: {operational:?}");
    debug!("expected: .#.###.#.######");
    let local = local_locations(springs, &operational);
    local.iter().for_each(|l| {
        debug!("local: {l:?}");
    });
    //let global = global_locations(&local);

    let result = 0;

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
