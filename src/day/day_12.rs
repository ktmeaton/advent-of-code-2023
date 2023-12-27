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

    // let springs = "???.###";
    // let damaged = [1,1,3];
    // let result = arrangements(springs, &damaged, 0);

    // let cache = ARRANGEMENTS.lock().unwrap();
    // debug!("cache: {cache:?}");
    // std::mem::drop(cache);

    let result = lines
        .into_iter()
        .enumerate()
        .map(|(i, l)| {
            let springs = l[0];
            let damaged = l[1].split(',').filter_map(|s| s.parse::<usize>().ok()).collect_vec();

            // set unfold level for part 2
            let unfold = match *part {
                Part::Part1 => 1,
                Part::Part2 => 5,
            };
            let springs = (0..unfold).map(|_| springs).join("?").chars().join("");
            let damaged =
                damaged.iter().cycle().take(damaged.len() * unfold).cloned().collect_vec();

            debug!("i: {i}, springs: {springs}, damaged: {damaged:?}");
            arrangements(&springs, &damaged, 0)
        })
        .sum();

    info!("Answer: {result}");
    Ok(result)
}

/// Use an explicit cache-type with a custom creation block and custom cache-key generating block
#[cached(
    type = "UnboundCache<String, usize>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ format!("{springs}_{damaged:?}") }"#
)]
fn arrangements(springs: &str, damaged: &[usize], i: usize) -> usize {

    // // cache inspection
    // debug!("{springs}, {i}");
    // let cache = ARRANGEMENTS.lock().unwrap();
    // debug!("cache: {cache:?}");
    // std::mem::drop(cache);

    // go to the final exact evaluation
    if i == springs.len() || !springs.contains('?') {
        match is_exact_match(springs, damaged) {
            true => return 1,
            false => return 0,
        }
    }

    // decision tree split point
    let s = springs.chars().collect_vec();

    //debug!("cache: {ARRANGEMENTS:?}");
    let mut result = 0;
    match s[i] {
        // try two hypotheses, ? is # or .
        '?' => {
            let hyp1 = s[0..i].iter().join("") + "#" + &s[i + 1..].iter().join("");
            //debug!("\thyp1: {hyp1}");             
            if is_approximate_match(&hyp1, damaged) {                
                result += arrangements(&hyp1, damaged, i + 1);
            }
            let hyp2 = s[0..i].iter().join("") + "." + &s[i + 1..].iter().join("");               
            //debug!("\thyp2: {hyp2}");            
            if is_approximate_match(&hyp2, damaged) {                
                result += arrangements(&hyp2, damaged, i + 1);
            }
        },
        // otherwise, move onto next
        _ => result += arrangements(springs, damaged, i + 1),
    };

    return result;
}

/// Check if the springs approximately match the expected damage pattern.
fn is_approximate_match(springs: &str, expected: &[usize]) -> bool {

    let matches = expected.iter().enumerate().map(|(i, o)| {
        //debug!("i: {i}, o: {o}, springs: {springs}");
        // find the operational springs '#' or '?', o times consecutive
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
            let before = i + expected[0..i].iter().sum::<usize>();
            format!("(?<=.{{{before},}})") + &patterns
        };

        // At the end, no springs may come after
        patterns = if i == expected.len() - 1 {
            patterns + "(?!.*#.*)"
        }
        // N springs must come after
        else {
            let after = (expected.len() - 1 - i) + expected[i + 1..].iter().sum::<usize>();
            patterns + format!("(?=.{{{after},}})").as_str()
        };

        // wrap full expression in a lookahead, to get overlaps
        //patterns = "(?=".to_string() + &patterns + ")";

        //debug!("\tpatterns: {patterns:?}");
        // construct the regular expression to find these patterns
        let re = Regex::new(&patterns).unwrap();
        re.find(springs).is_some()
    })
    .collect_vec();
    !matches.contains(&false)
}

/// Check if the springs exactly match the expected damage pattern.
fn is_exact_match(springs: &str, expected: &[usize]) -> bool {

    let mut observed = Vec::new();
    let mut damaged_spring: Option<usize> = None;

    springs.chars().for_each(|c| {
        damaged_spring = match damaged_spring {
            // previous char was a spring
            Some(l) => match c {
                // still in spring
                '#' => Some(l + 1),
                // exited spring
                _ => {
                    observed.push(l);
                    None
                }
            },
            // previous char was not spring
            None => match c {
                // new spring begins
                '#' => Some(1),
                _ => None,
            },
        };
    });

    if let Some(l) = damaged_spring {
        observed.push(l)
    }

    observed == expected
}

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
