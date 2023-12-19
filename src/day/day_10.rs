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
    let input = utils::read_to_string("data/day_10.txt")?;
    let mut pipe_map = Map::from_str(&input)?;

    // part 2, zoom in (ie. push pipes apart)
    if *part == Part::Part2 {
        pipe_map.zoom_in();
    }

    // find start location
    let y = pipe_map.tiles.iter().position(|row| row.contains(&'S')).unwrap();
    let x = pipe_map.tiles[y].iter().position(|x| *x == 'S').unwrap();

    // fill the loop from start
    debug!("Identifying pipe loop.");
    let pipe_loop = pipe_map.flood_fill(x, y).into_iter().map(|(x, y, _c)| (x, y)).collect_vec();

    // in part 2, find all tiles inside loop
    let (mut insiders, mut outsiders) = (Vec::new(), Vec::new());
    if *part == Part::Part2 {
        // exclude odd number rows and columns (zoom), and pipe loop
        debug!("Identifying inside candidates.");
        let candidates = (0..pipe_map.tiles.len())
            .step_by(2)
            .flat_map(|y| {
                (0..pipe_map.tiles[y].len())
                    .step_by(2)
                    .filter_map(|x| (!pipe_loop.contains(&(x, y))).then_some((x, y)))
                    .collect_vec()
            })
            .collect_vec();

        // replace '*' and '.' with '+'
        pipe_map.tiles.iter_mut().for_each(|row| {
            *row = row
                .iter()
                .map(|c| match *c == '*' || *c == '.' {
                    true => '+',
                    false => *c,
                })
                .collect_vec()
        });

        for coord in &candidates {
            if outsiders.contains(coord) || insiders.contains(coord) {
                continue;
            }
            debug!("coord: {coord:?}");
            let (x, y) = coord;
            let mut filled = pipe_map
                .flood_fill(*x, *y)
                .into_iter()
                .map(|(x, y, _c)| (x, y))
                //.filter(|n| candidates.contains(n))
                .collect_vec();
            debug!("\tfilled: {filled:?}");

            // if filled contains an edge tile, this is outside
            // or if it's a self-contained loop...
            let is_outside = filled.iter().find(|(x, y)| {
                *x < 1
                    || *x >= pipe_map.tiles[*y].len() - 1
                    || *y < 1
                    || *y >= pipe_map.tiles.len() - 1
            });

            match is_outside.is_some() {
                true => outsiders.append(&mut filled),
                false => insiders.append(&mut filled),
            };
        }

        insiders.retain(|i| candidates.contains(i));

        //insiders.iter().for_each(|i| debug!("insider: {i:?}"));

        pipe_map.tiles.iter_mut().enumerate().for_each(|(y, row)| {
            *row = row
                .iter_mut()
                .enumerate()
                .map(|(x, c)| {
                    if outsiders.contains(&(x, y)) {
                        'O'
                    } else if insiders.contains(&(x, y)) {
                        'I'
                    } else {
                        *c
                    }
                })
                .collect_vec();
        });

        pipe_map.zoom_out();
        //debug!("\n{}", pipe_map.pretty_print()?);
        pipe_map.tiles.iter().for_each(|row| {
            println!("{}", row.iter().join(""));
        });
    }

    let result = match *part {
        Part::Part1 => pipe_loop.len() / 2,
        Part::Part2 => insiders.len(),
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
