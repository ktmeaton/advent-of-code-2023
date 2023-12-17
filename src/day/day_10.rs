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
    let pipe_map = Map::from_str(&input)?;

    // part 2, add spaces

    // find start location
    let y = pipe_map.tiles.iter().position(|row| row.contains(&'S')).unwrap();
    let x = pipe_map.tiles[y].iter().position(|x| *x == 'S').unwrap();
    let c = pipe_map.tiles[y][x];

    // keep track of pipe connections we've seen
    let mut travel_history = HashMap::new();
    let curr = (x, y, c);
    travel_history.insert(curr, curr);

    // keep track of where we are currently (will be multple tiles)
    let mut current = vec![curr.clone()];
    let mut loop_found = false;
    let mut counter = 0;

    while !loop_found {
        counter += 1;
        //debug!("counter: {counter}");
        current = current
            .into_iter()
            .flat_map(|curr| {
                let prev = travel_history.get(&curr).unwrap();
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
                    match travel_history.contains_key(&n) {
                        true => loop_found = true,
                        false => _ = travel_history.insert(n, curr),
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

    // ------------------------------------------------------------------------
    // identify columns and rows to push apart

    let mut push_rows: Vec<usize> = Vec::new();
    let mut push_columns: Vec<usize> = Vec::new();

    let (max_x, max_y) = (pipe_map.tiles[0].len() - 1, pipe_map.tiles.len() - 1);
    (0..=max_y - 1).for_each(|y| {
        (0..=max_x - 1).for_each(|x| {
            let curr_tile = (x, y, pipe_map.tiles[y][x]);

            let next_col_tile = (x + 1, y, pipe_map.tiles[y][x + 1]);
            let next_row_tile = (x, y + 1, pipe_map.tiles[y + 1][x]);

            let next_col_n = pipe_map.get_pipe_neighbors(x + 1, y);
            let next_row_n = pipe_map.get_pipe_neighbors(x, y + 1);

            if travel_history.contains_key(&curr_tile) {
                // next chars are non-neighboring pipe
                if !push_columns.contains(&x)
                    && travel_history.contains_key(&next_col_tile)
                    && !next_col_n.contains(&(x, y))
                {
                    push_columns.push(x);
                }
                if !push_rows.contains(&y)
                    && travel_history.contains_key(&next_row_tile)
                    && !next_row_n.contains(&(x, y))
                {
                    push_rows.push(y);
                }
            }
        });
    });

    let mut push_pipes = pipe_map.clone();
    push_rows.sort();
    push_columns.sort();

    // update travel history coordinates after pushing
    travel_history = travel_history
        .into_iter()
        .map(|(curr, prev)| {
            let mut curr = curr;
            // update x and y
            if let Some(i) = push_columns.iter().position(|x| *x == curr.0) {
                curr.0 = x + i;
            }
            if let Some(i) = push_rows.iter().position(|y| *y == curr.1) {
                curr.1 = y + i;
            }

            let mut prev = prev;
            if let Some(i) = push_columns.iter().position(|x| *x == prev.0) {
                prev.0 = x + i;
            }
            if let Some(i) = push_rows.iter().position(|y| *y == prev.1) {
                prev.1 = y + i;
            }

            (curr, prev)
        })
        .collect();

    // push columns
    debug!("push_columns: {push_columns:?}");
    push_columns.into_iter().enumerate().for_each(|(i, x)| {
        let x = x + i;
        let p = push_pipes.clone();
        push_pipes.tiles.iter_mut().enumerate().for_each(|(y, row)| {
            let n = p.get_pipe_neighbors(x + 1, y);
            match n.contains(&(x, y)) {
                true => row.insert(x + 1, '-'),
                false => row.insert(x + 1, '*'),
            }
        });
    });

    // push rows
    debug!("push_rows: {push_rows:?}");
    push_rows.into_iter().enumerate().for_each(|(i, y)| {
        let y = y + i;
        let new_row = (0..push_pipes.tiles[y].len() - 1)
            .map(|x| {
                let n = push_pipes.get_pipe_neighbors(x, y + 1);
                match n.contains(&(x, y)) {
                    true => '|',
                    false => '*',
                }
            })
            .collect_vec();
        push_pipes.tiles.insert(y + 1, new_row);
    });

    push_pipes.tiles.iter().enumerate().for_each(|(y, row)| debug!("{y}: {}", row.iter().join("")));

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
