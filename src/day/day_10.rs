use crate::day::Part;
use crate::game::Map;

use crate::utils;
use color_eyre::eyre::{Report, Result};
//use itertools::Itertools;
use log::debug;
use log::info;
use std::str::FromStr;

/// Day 10 - Pipe Maze
pub fn run(part: &Part) -> Result<usize, Report> {
    let input = utils::read_to_string("data/day_10.txt")?;
    let mut pipe_map = Map::from_str(&input)?;

    // part 2, zoom in (ie. push pipes apart to go in between)
    if *part == Part::Part2 {
        pipe_map.zoom_in();
    }

    // find start location
    let y = pipe_map.tiles.iter().position(|row| row.contains(&'S')).unwrap();
    let x = pipe_map.tiles[y].iter().position(|x| *x == 'S').unwrap();

    // find the loop by flood fill, starting at 'S'
    debug!("Finding pipe loop.");
    let follow_pipes = true;
    let pipe_loop = pipe_map.flood_fill(x, y, follow_pipes);

    // in part 2, find all tiles inside and outside the loop
    let (mut insiders, mut outsiders) = (Vec::new(), Vec::new());
    if *part == Part::Part2 {
        let (x_max, y_max) = (pipe_map.tiles[0].len() - 1, pipe_map.tiles.len() - 1);

        // replace all non-pipe coords with '+', identify candidate I/O on even
        // numbered rows and columns (because we zoomed)
        debug!("Finding candidates.");
        let mut candidates = Vec::new();
        (0..=y_max).for_each(|y| {
            (0..=x_max).for_each(|x| {
                if !pipe_loop.contains(&(x, y)) {
                    pipe_map.tiles[y][x] = '.';
                    if x % 2 == 0 && y % 2 == 0 {
                        candidates.push((x, y));
                    }
                };
            })
        });

        debug!("Flood filling candidates.");
        while !candidates.is_empty() {
            let (x, y) = candidates.first().cloned().unwrap();
            let follow_pipes = false;
            let mut filled = pipe_map.flood_fill(x, y, follow_pipes);
            filled.retain(|(x, y)| candidates.contains(&(*x, *y)));

            // check if fill went to the edge (ie outside)
            let fill_to_edge =
                filled.iter().any(|(x, y)| *x == 0 || *x == x_max || *y == 0 || *y == y_max);

            match fill_to_edge {
                true => outsiders.append(&mut filled),
                false => insiders.append(&mut filled),
            };

            candidates.retain(|c| !outsiders.contains(c) && !insiders.contains(c));
        }

        // // debugging, change chars to 'O' and 'I'
        // pipe_map.tiles.iter_mut().enumerate().for_each(|(y, row)| {
        //     *row = row
        //         .iter_mut()
        //         .enumerate()
        //         .map(|(x, c)| {
        //             if outsiders.contains(&(x, y)) {
        //                 'O'
        //             } else if insiders.contains(&(x, y)) {
        //                 'I'
        //             } else {
        //                 *c
        //             }
        //         })
        //         .collect_vec();
        // });

        // pipe_map.zoom_out();
        // pipe_map.tiles.iter().for_each(|row| {
        //     println!("{}", row.iter().join(""));
        // });
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
    let expected = 381;
    let observed = run(&Part::Part2)?;
    assert_eq!(observed, expected);
    Ok(())
}
