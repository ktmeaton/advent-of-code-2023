use crate::day::Part;
use crate::game::Map;
use crate::utils;
use color_eyre::eyre::{Report, Result};
use itertools::Itertools;
use log::debug;
use log::info;
use std::str::FromStr;

/// Day 11
pub fn run(part: &Part) -> Result<usize, Report> {
    let input = utils::read_to_string("data/day_11.txt")?;
    let mut galaxy_map = Map::from_str(&input)?;

    // expand empty space horizontally (x)
    let galaxy_x = galaxy_map
        .tiles
        .iter()
        .filter(|row| row.contains(&'#'))
        .flat_map(|row| {
            row.iter().enumerate().filter_map(|(x, c)| (*c == '#').then_some(x)).collect_vec()
        })
        .unique()
        .collect_vec();
    let empty_x = (0..galaxy_map.tiles[0].len()).filter(|x| !galaxy_x.contains(x)).collect_vec();
    galaxy_map.tiles.iter_mut().for_each(|row| {
        empty_x.iter().enumerate().for_each(|(i, x)| row.insert(x + i, '.'));
    });
    // expand empty space vertically (y)
    galaxy_map.tiles = galaxy_map
        .tiles
        .into_iter()
        .flat_map(|row| match row.contains(&'#') {
            true => vec![row],
            false => vec![row.clone(), row.clone()],
        })
        .collect_vec();

    // pairwise distances between new expanded galaxies
    let galaxies = galaxy_map
        .tiles
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(|(x, c)| (*c == '#').then_some((x, y))).collect_vec()
        })
        .collect_vec();
    let distances = galaxies[0..galaxies.len() - 1]
        .iter()
        .enumerate()
        .flat_map(|(i1, g1)| {
            debug!("g{}: {g1:?}", i1 + 1);
            galaxies[i1 + 1..galaxies.len()]
                .iter()
                .enumerate()
                .map(|(i2, g2)| {
                    let dist = ((g2.0 as isize - g1.0 as isize).abs()
                        + (g2.1 as isize - g1.1 as isize).abs())
                        as usize;
                    debug!("\tg{}: {g2:?}, dist: {dist}", i1 + i2 + 2);
                    dist
                })
                .collect_vec()
        })
        .collect_vec();

    galaxy_map.tiles.iter().for_each(|row| {
        debug!("{}", row.iter().join(""));
    });

    let result = match *part {
        Part::Part1 => distances.into_iter().sum::<usize>(),
        Part::Part2 => 2,
    };

    info!("Answer: {result}");
    Ok(result)
}

#[test]
fn part_1() -> Result<(), Report> {
    let expected = 9769724;
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
