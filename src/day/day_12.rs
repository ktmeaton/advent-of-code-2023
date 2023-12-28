use crate::day::Part;
use crate::utils;

use cached::{proc_macro::cached, UnboundCache};
use color_eyre::eyre::{Report, Result};
use itertools::Itertools;
use log::{debug, info};

/// Day 12 - Hot Springs
///
/// Recursions + memoization
pub fn run(part: &Part) -> Result<usize, Report> {
    // Parse puzzle input into lines
    let input = utils::read_to_string("data/day_12.txt")?;
    let lines = input.split('\n').map(|l| l.split(' ').collect_vec()).collect_vec();

    // sum up the possible arrangements
    let result = lines
        .into_iter()
        .enumerate()
        //.take_while(|(i, _l)| *i == 0)
        .map(|(i, l)| {
            let springs = l[0];
            let damage = l[1].split(',').filter_map(|s| s.parse::<usize>().ok()).collect_vec();

            // set unfold level for part 2
            let unfold = match *part {
                Part::Part1 => 1,
                Part::Part2 => 5,
            };
            let springs = (0..unfold).map(|_| springs).join("?").chars().join("");
            let damage = damage.iter().cycle().take(damage.len() * unfold).cloned().collect_vec();

            // let springs = ".??..??...?##.";
            // let damage = [1, 1, 3];
            debug!("i: {i}, springs: {springs}, damage: {damage:?}");
            arrangements(&springs, &damage, "", "")
        })
        .sum();

    info!("Answer: {result}");
    Ok(result)
}

/// Recursive spring damage function, for learning memoization.
///
/// Inspired by the following pieces of code.
///
/// Author: Diderkdm
/// Source:  https://www.reddit.com/r/adventofcode/comments/18ge41g/comment/kd221yp
///
/// Author: HyperNeutrino
/// Source: https://www.youtube.com/watch?v=g3Ms5e7Jdqo
#[cached(
    type = "UnboundCache<String, usize>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ format!("{springs} {damage:?}") }"#
)]
fn arrangements(springs: &str, damage: &[usize], prev: &str, indent: &str) -> usize {
    let mut result = 0;

    // check for recursion bottoming out
    if damage.is_empty() {
        return !springs.contains('#') as usize;
    }
    // this test might not be necessary?
    // if springs == "" {
    //     return damage.is_empty() as usize
    // }

    // separate the damage into what we are currently evaluting, and what comes next
    let (curr_dmg, next_dmg) = (&damage[0], &damage[1..]);
    // the remaining springs must be at least this long, otherwise there is more expected
    // damage than is possible, factors in damage springs and at least one gap (. or ?)
    let l = springs.len() - next_dmg.iter().sum::<usize>() - next_dmg.len() - curr_dmg;
    //debug!("{indent}prev: {prev}, springs: {springs}, damage: {damage:?}, l: {l}");
    let indent = format!("{indent}\t");
    for i in 0..=l {
        //debug!("{indent}i: {i}");
        if springs[..i].contains('#') {
            break;
        }
        let nxt = i + curr_dmg;
        // focus on arrangements where springs are long enough to fulfill damage pattern
        // and don't contain any operational springs (.)
        if nxt <= springs.len() && !springs[i..nxt].contains('.') {
            match nxt == springs.len() {
                true => {
                    let prev = format!("{prev}{}", &springs[..nxt].replace('?', "#"));
                    let springs = "";
                    result += arrangements(springs, next_dmg, &prev, &indent);
                    if result > 0 {
                        debug!("{indent}{prev}");
                    }
                }
                false => {
                    if springs[nxt..nxt + 1] != *"#" {
                        let prev = prev.to_string()
                            + &springs[..i].replace('?', ".")
                            + &springs[i..nxt].replace('?', "#")
                            + ".";
                        let springs = &springs[nxt + 1..];
                        // cache inspection, simply for nice debugging output
                        let c = ARRANGEMENTS.lock().unwrap();
                        let seen = c.get_store().get(&format!("{springs} {next_dmg:?}")).is_some();
                        std::mem::drop(c);
                        result += arrangements(springs, next_dmg, &prev, &indent);

                        if result > 0 && next_dmg.len() == 1 && seen {
                            debug!("{indent}{prev}{}", &springs.replace('?', "#"));
                        }
                    }
                }
            }
        }
    }
    return result;
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
    let expected = 23903579139437;
    let observed = run(&Part::Part2)?;
    assert_eq!(observed, expected);
    Ok(())
}
