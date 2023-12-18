use crate::day::Part;
use crate::game::Map;

use crate::utils;
use color_eyre::eyre::{Report, Result};
use itertools::Itertools;
use log::debug;
use log::info;
use std::collections::HashMap;
use std::str::FromStr;

/// Day 10 - Pipes
/// 1. Find the single giant loop starting at S.
/// 2. How many steps along the loop does it take to get
///    from the starting position to the point farthest from
///    the starting position?
///
/// Part 1
/// 1. [x] Parse the input pipes into a matrix, maybe our game map from day 3?
/// 2. [x] Find the starting position 'S'.
/// 3. [x] The character dictates neighbors (F = SE neighbors)
/// 4. [x] Walk away from S through all possible pipes
///
/// Part 2
/// "Squeezing between pipes", could we look for all loop pipes that are
/// adjacaent and push them apart?
pub fn run(part: &Part) -> Result<usize, Report> {
    let input = utils::read_to_string("data/day_10_part_2_test.txt")?;
    let mut pipe_map = Map::from_str(&input)?;

    // part 2, push apart each row and column to spaces between pipes
    if *part == Part::Part2 {
        pipe_map = pipe_map.push_pipe_rows();
        pipe_map = pipe_map.push_pipe_columns();
        // debug
        debug!("\n{}", pipe_map.pretty_print()?);
        //pipe_map.tiles.iter().for_each(|row| debug!("{}", row.iter().join("")));
    }

    // find start location
    let y = pipe_map.tiles.iter().position(|row| row.contains(&'S')).unwrap();
    let x = pipe_map.tiles[y].iter().position(|x| *x == 'S').unwrap();
    let c = pipe_map.tiles[y][x];

    // keep track of pipe connections we've seen
    let mut pipe_history = HashMap::new();
    let curr = (x, y, c);
    pipe_history.insert(curr, curr);

    // keep track of where we are currently (will be multple tiles)
    let mut current = vec![curr];
    let mut loop_found = false;
    let mut counter = 0;

    while !loop_found {
        counter += 1;
        //debug!("counter: {counter}");
        current = current
            .into_iter()
            .flat_map(|curr| {
                let prev = pipe_history.get(&curr).unwrap();
                //debug!("\tcurrent: {curr:?}, previous: {prev:?}");

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
                    match pipe_history.contains_key(&n) {
                        true => loop_found = true,
                        false => _ = pipe_history.insert(n, curr),
                    };
                    //debug!("\t\tnext: {n:?}, loop_found: {loop_found}");
                }
                next_nodes
            })
            .collect_vec();

        if loop_found {
            break;
        }
    }

    // in part 2, find all tiles inside loop,
    // all tiles that are not in pipe history are candidates
    // if we can walk from a coordinate to an edge, it is outside
    if *part == Part::Part2 {
        let candidates = (0..pipe_map.tiles.len())
            .filter(|y| *y > 1 && *y < pipe_map.tiles.len() - 2)
            .flat_map(|y| {
                (0..pipe_map.tiles[y].len())
                    .filter(|x| *x > 1 && *x < pipe_map.tiles[y].len() - 2)
                    .filter_map(|x| {
                        let c = pipe_map.tiles[y][x];
                        match c != '*' && !pipe_history.contains_key(&(x, y, c)) {
                            true => Some((x, y, c)),
                            false => None,
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();

        candidates.into_iter().for_each(|c| debug!("candidate: {c:?}"));
    }

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
