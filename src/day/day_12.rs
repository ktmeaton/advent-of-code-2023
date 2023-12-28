use crate::day::Part;
use crate::utils;

use cached::proc_macro::cached;
use cached::UnboundCache;
use color_eyre::eyre::{Report, Result};
use itertools::Itertools;
use log::{debug, info};
use onig::Regex;

/// Day 12 - Hot Springs
///
/// Recursions + memoization
pub fn run(part: &Part) -> Result<usize, Report> {
    // Parse puzzle input
    let input = utils::read_to_string("data/day_12_test.txt")?;
    let lines = input.split('\n').map(|l| l.split(' ').collect_vec()).collect_vec();

    // trim leading and trailing periods
    // let re = Regex::new("^[\\.]*|[\\.]*$").unwrap();
    let springs = "???.###";
    // actual =   ".#.###.#.######";
    let expected = [1,1, 3];
    debug!("springs: {springs}, nums: {expected:?}, i: 0");
    let result = count(&springs, &expected, "");

    // let cache = ARRANGEMENTS.lock().unwrap();
    // debug!("cache: {cache:?}");
    // std::mem::drop(cache);

    // let result = lines
    //     .into_iter()
    //     .enumerate()
    //     //.take_while(|(i, l)| *i == 0)
    //     .map(|(i, l)| {
    //         let springs = l[0];
    //         let damaged = l[1].split(',').filter_map(|s| s.parse::<usize>().ok()).collect_vec();

    //         // set unfold level for part 2
    //         let unfold = match *part {
    //             Part::Part1 => 1,
    //             Part::Part2 => 5,
    //         };
    //         let springs = (0..unfold).map(|_| springs).join("?").chars().join("");
    //         let damaged =
    //             damaged.iter().cycle().take(damaged.len() * unfold).cloned().collect_vec();

    //         debug!("i: {i}, springs: {springs}, damaged: {damaged:?}");
    //         count(&springs, &damaged)
    //     })
    //     .sum();

    info!("Answer: {result}");
    Ok(result)
}

/// Recursive counting function, for learning memoization.
///
/// Author: @HyperNeutrino
/// Source: https://www.youtube.com/watch?v=g3Ms5e7Jdqo
// #[cached(
//     type = "UnboundCache<String, usize>",
//     create = "{ UnboundCache::new() }",
//     convert = r#"{ format!("{cfg} {nums:?}") }"#
// )]
fn count(cfg: &str, nums: &[usize], prev: &str) -> usize {
    // easy exits
    let valid = is_valid(cfg, nums);
    debug!("    hyp: {}, valid: {valid}, prev: {prev}, cfg: {cfg}, nums: {nums:?}", prev.to_owned() + cfg);
    if !valid || cfg == "" || nums.is_empty() || !cfg.contains("?"){
        return valid as usize
    }

    let mut hypotheses = Vec::new();
    let s = &cfg[..1];

    if s == "." || s == "?" {
        let cfg = &cfg[1..];
        let prev = prev.to_owned() + ".";
        hypotheses.push((cfg, nums, prev));
    }
    if s == "#" || s == "?" {
        let d = nums[0];
        let prev = prev.to_string() + "#".repeat(d).as_str();
        let (cfg, nums, prev) = match cfg.len() > d {
            true => (&cfg[d + 1..], &nums[1..], prev.to_string() + &cfg[d..=d]),
            false => ("", &nums[nums.len()..], prev),
        };
        hypotheses.push((cfg, nums, prev));
    };

    hypotheses
        .into_iter()
        .filter_map(|(cfg, nums, prev)| {
            let valid = is_valid(cfg, nums);
            if !valid {
                debug!("    hyp: {}, valid: {valid}", prev.to_owned() + cfg);
            }
            valid.then(|| count(cfg, nums, &prev))
        })
        .sum()
}

// #[cached(
//     type = "UnboundCache<String, bool>",
//     create = "{ UnboundCache::new() }",
//     convert = r#"{ format!("{springs} {expected:?}") }"#
// )]
fn is_valid(springs: &str, expected: &[usize]) -> bool {
    let mut pattern = expected.iter().map(|e| format!("[#|\\?]{{{e}}}")).join("[\\.|\\?]+");
    pattern = "(?<!.*#.*)".to_string() + &pattern + "(?!.*#.*)";
    Regex::new(&pattern).unwrap().find(springs).is_some()
}

// fn is_valid(cfg: &str, nums: &[usize]) -> bool {

//     if cfg == "" {
//         return nums.is_empty()
//     }

//     if nums.is_empty() {
//         return !cfg.contains("#")
//     }

//     // trim leading and trailing periods
//     let re = Regex::new("^[\\.]*|[\\.]*$").unwrap();
//     let cfg = re.replace_all(cfg, "");

//     // check expected damage (next and total)
//     let d = nums[0];
//     let total_damage = nums.iter().sum::<usize>();
//     let potential_damage = cfg.chars().filter(|c| *c != '.').count();
//     if d > cfg.len() || potential_damage < total_damage || cfg[..d].contains(".") || (cfg.len() > d && cfg[d..d+1] == *"#") {
//         return false
//     }

//     true

// }

#[test]
fn part_1() -> Result<(), Report> {
    let expected = 7633;
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
