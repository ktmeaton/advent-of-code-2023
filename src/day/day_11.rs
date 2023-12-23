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
    let galaxy_map = Map::from_str(&input)?;

    // find empty space, not occupied by galaxies
    let mut galaxies = galaxy_map.search(&'#');
    let (gx, gy): (Vec<usize>, Vec<usize>) = galaxies.clone().into_iter().unzip();

    let x_max = galaxy_map.tiles[0].len() - 1;
    let empty_x = (0..=x_max).filter(|x| !gx.contains(x)).unique().collect_vec();
    let empty_y = (0..galaxy_map.tiles.len()).filter(|y| !gy.contains(y)).unique().collect_vec();

    // expand empty space, at different rates
    let e = match *part {
        Part::Part1 => 2,
        Part::Part2 => 1000000,
    };

    galaxies.iter_mut().enumerate().for_each(|(i, (gx, gy))| {
        let (gx_orig, gy_orig) = (*gx, *gy);
        let xn = empty_x.iter().filter(|x| *x < gx).count();
        let yn = empty_y.iter().filter(|y| *y < gy).count();
        if xn > 0 {
            *gx = *gx - xn + (xn * e);
        }
        if yn > 0 {
            *gy = *gy - yn + (yn * e);
        }
        debug!("g{}: {gx_orig}, {gy_orig} => {gx}, {gy}", i + 1);
    });

    // sum of pairwise distances
    let result = galaxies[0..galaxies.len() - 1]
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
        .sum();

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
    let expected = 603020563700;
    let observed = run(&Part::Part2)?;
    assert_eq!(observed, expected);
    Ok(())
}
