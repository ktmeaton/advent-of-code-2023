use crate::day::Part;
use crate::game::{Map, Tile};

use color_eyre::eyre::{Report, Result};
use itertools::Itertools;
use log::info;
use std::str::FromStr;

/// Day 3
///
/// The original puzzle involved an "engine schematic". I thought it would be more
/// fun to represent this as a map of a text game.
/// Part Number => Character
/// Gear => Trap
/// Symbol => Enemy
/// Period = Base
pub fn run(part: &Part) -> Result<usize, Report> {
    // parse input file into game map
    let input = std::fs::read_to_string("data/day_3.txt")?;
    let map = Map::from_str(&input)?;

    // locate all our map tiles of interest
    let traps = map.find_tile(&Tile::Trap).into_iter().flatten().collect_vec();
    let enemies = map.find_tile(&Tile::Enemy).into_iter().flatten().collect_vec();
    let characters = map.find_tile(&Tile::Character);

    let mut result = 0;

    match *part {
        // --------------------------------------------------------------------
        // Part 1: Characters (part number) next to enemy (symbol)
        Part::Part1 => {
            for coords in &characters {
                for (x, y) in coords {
                    let mut n = map.get_neighbors(*x, *y);
                    n.retain(|coord| enemies.contains(coord));
                    if !n.is_empty() {
                        let (number, _) = map.get_character(*x, *y).unwrap();
                        result += number;
                        break;
                    }
                }
            }
        }
        // --------------------------------------------------------------------
        // Part 2: Traps (gear) next to exactly 2 characters (part numbers)
        Part::Part2 => {
            for (x, y) in &traps {
                let n = map.get_neighbors(*x, *y);
                let n_characters = characters
                    .iter()
                    .filter_map(|coords| {
                        let mut overlap = n.clone();
                        overlap.retain(|c| coords.contains(c));
                        if !overlap.is_empty() {
                            let (char_x, char_y) = overlap[0];
                            let (n, _) = map.get_character(char_x, char_y).unwrap();
                            Some(n)
                        } else {
                            None
                        }
                    })
                    .collect_vec();
                if n_characters.len() == 2 {
                    result += n_characters[0] * n_characters[1]
                }
            }
        }
    }

    info!("Answer: {result}");
    Ok(result)
}

#[test]
fn part_1() -> Result<(), Report> {
    let expected = 539590;
    let observed = run(&Part::Part1)?;
    assert_eq!(observed, expected);
    Ok(())
}

#[test]
fn part_2() -> Result<(), Report> {
    let expected = 80703636;
    let observed = run(&Part::Part2)?;
    assert_eq!(observed, expected);
    Ok(())
}
