use crate::day::Part;
use crate::game::Map;

use crate::utils;
use color_eyre::eyre::{Report, Result};
use itertools::Itertools;
use log::debug;
use log::info;
use std::str::FromStr;

/// Day 10 - Pipes
/// 1. Find the single giant loop starting at S.
/// 2. How many steps along the loop does it take to get
///    from the starting position to the point farthest from
///    the starting position?
///
/// 1. [x] Parse the input pipes into a matrix, maybe our game map from day 3?
/// 2. [x] Find the starting position 'S'.
/// 3. [x] The character dictates neighbors (F = SE neighbors)
/// 4. [ ] Walk away from S through all possible pipes
pub fn run(part: &Part) -> Result<usize, Report> {
    let input = utils::read_to_string("data/day_10.txt")?;
    let pipe_map = Map::from_str(&input)?;

    // find start location
    let y = pipe_map.tiles.iter().position(|row| row.contains(&'S')).unwrap();
    let x = pipe_map.tiles[y].iter().position(|x| *x == 'S').unwrap();
    let c = pipe_map.tiles[y][x];
    let mut current = vec![((0, 0, '.'), (x, y, c))];

    // keep track of coordinates we've seen
    let mut travel_history = vec![(x, y, c)];
    let mut loop_found = false;
    let mut counter = 0;

    while !loop_found {
        counter += 1;
        debug!("counter: {counter}");
        current = current
            .into_iter()
            .flat_map(|(prev, curr)| {
                debug!("\tprevious: {prev:?}, current: {curr:?}");
                //let (x, y, c) = curr;
                // get next node connections
                let next_nodes = pipe_map
                    .get_pipe_neighbors(curr.0, curr.1)
                    .into_iter()
                    // don't backtrack to previous node
                    .filter(|n| *n != (prev.0, prev.1))
                    // keep valid connections back to current
                    .filter(|(x, y)| {
                        let n = pipe_map.get_pipe_neighbors(*x, *y);
                        n.contains(&(curr.0, curr.1))
                    })
                    .map(|(x, y)| (x, y, pipe_map.tiles[y][x]))
                    .collect_vec();

                // loop break check
                for n in next_nodes.clone() {
                    if loop_found {
                        break;
                    }
                    match travel_history.contains(&n) {
                        true => loop_found = true,
                        false => travel_history.push(n),
                    };
                    debug!("\t\tnext: {n:?}, loop_found: {loop_found}");
                }
                next_nodes.into_iter().map(|n| (curr, n)).collect_vec()
            })
            .collect_vec();

        if loop_found {
            break;
        }
    }

    debug!("counter: {counter}");

    let result = match *part {
        Part::Part1 => counter,
        Part::Part2 => 2,
    };

    info!("Answer: {result}");
    Ok(result)
}

#[test]
fn part_1() -> Result<(), Report> {
    let expected = 6717;
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
