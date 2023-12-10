use crate::day::Part;

use color_eyre::eyre::{Report, Result};
use itertools::Itertools;
use log::{debug, info};
use std::collections::BTreeMap;

const GEAR: char = '*';

/// Day 3
///
/// The engine schematic (your puzzle input) consists of a visual representation
/// of the engine. There are lots of numbers and symbols you don't really
/// understand, but apparently any number adjacent to a symbol, even diagonally,
/// is a "part number" and should be included in your sum. (Periods (.) do not
/// count as a symbol.)
pub fn run(part: &Part) -> Result<usize, Report> {
    // input parsing
    let mut input = std::fs::read_to_string("data/day_3.txt")?;
    if input.ends_with('\n') {
        input.pop();
    }

    let input = "467..114..
...*......
..35..633."
        .to_string();

    let matrix = input.split('\n').map(|l| l.chars().collect_vec()).collect_vec();

    // get all part numbers
    let mut gears = BTreeMap::new();
    let mut part_numbers = BTreeMap::new();
    let mut symbols = BTreeMap::new();

    (0..matrix.len()).for_each(|y| {
        (0..matrix[0].len()).for_each(|x| {
            let coord = (x, y);
            let c = matrix[y][x];
            if c == GEAR {
                gears.insert(coord, c);
            }
            if !c.is_ascii_digit() && c != '.' {
                symbols.insert(coord, c);
            }
            if let Some(part_number) = get_part_number(&matrix, x, y) {
                part_numbers.insert(coord, part_number);
            }
        });
    });

    debug!("part_numbers: {part_numbers:?}");
    debug!("gears: {gears:?}");
    debug!("symbols: {symbols:?}");

    symbols.into_iter().for_each(|((x, y), _)| {
        //let neighbors =
        debug!("x: {x}, y: {y}");
    });

    return Ok(0);

    // // ------------------------------------------------------------------------
    // // Schematic Matrix

    // let schematic = Schematic::new(&input);

    // let mut part_number_sum = 0;
    // let mut gear_ratio_sum = 0;

    // for y in 0..(schematic.rows) {
    //     let mut x = 0;
    //     while x < (schematic.columns) {
    //         debug!("{}", "-".repeat(80));
    //         let c = schematic.matrix[y][x];
    //         let part_number = schematic.get_part_number(x, y);

    //         debug!("x: {x}, y: {y}, c: {c}, part_number: {part_number:?}");

    //         // determine x coords we should search for neighbors
    //         let search_x: Vec<_> = if let Some((part_number, _start)) = &part_number {
    //             let digits = part_number.to_string();
    //             (x..(x + digits.len())).collect()
    //         } else {
    //             vec![x]
    //         };

    //         // get neighbors (coords and chars)
    //         let neighbors = search_x
    //             .iter()
    //             .flat_map(|x| schematic.get_neighbors(*x, y))
    //             .collect_vec();
    //         debug!("neighbors: {neighbors:?}");

    //         // in part 1, focus on neighbors that are symbols
    //         let symbols = neighbors
    //             .iter()
    //             .filter_map(|(_, c)| (!c.is_ascii_digit() && *c != '.').then_some(c))
    //             .collect_vec();
    //         debug!("symbols: {symbols:?}");

    //         if let Some((part_number, _start)) = part_number {
    //             if !symbols.is_empty() {
    //                 part_number_sum += part_number;
    //             }
    //         }

    //         // in part 2, focus on neighbors that are part_numbers
    //         let part_numbers = neighbors
    //             .iter()
    //             // extract part number from neighbor
    //             .filter_map(|((x, y), _)| {
    //                 let result = schematic.get_part_number(*x, *y);
    //                 result.map(|(part_number, start)| ((start, y), part_number))
    //             })
    //             .unique()
    //             .map(|(_, n)| n)
    //             .collect_vec();
    //         debug!("part_numbers: {part_numbers:?}");

    //         if c == GEAR && part_numbers.len() == 2 {
    //             gear_ratio_sum += part_numbers[0] * part_numbers[1];
    //         }
    //         // increment search coords
    //         x += search_x.len();
    //     }
    // }

    // let result = match *part {
    //     Part::Part1 => part_number_sum,
    //     Part::Part2 => gear_ratio_sum,
    // };

    // info!("Answer: {result}");
    // Ok(result)
}

/// Check if a coordinate is a part number fragment. If so return the full
/// number and the left most coordinate (start).
pub fn get_part_number(
    matrix: &Vec<Vec<char>>,
    x: usize,
    y: usize,
) -> Option<(usize, usize)> {
    // check if the coordinate is a digit
    if !matrix[y][x].is_ascii_digit() {
        return None;
    }

    // find start coordinate (left most digit of part number)
    let mut start = x;
    for x_i in (0..=x).rev() {
        match matrix[y][x_i].is_ascii_digit() {
            true => start = x_i,
            false => break,
        }
    }

    // read left to right from start
    let mut digits = Vec::new();
    for x_i in start..(matrix[y].len()) {
        let c = matrix[y][x_i];
        match c.is_ascii_digit() {
            true => digits.push(c),
            false => break,
        }
    }

    // parse digits into number
    let part_number: usize = digits.into_iter().join("").parse().unwrap();
    Some((part_number, start))
}

pub fn get_neighbors(matrix: Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let (x, y) = (x as i32, y as i32);

    let x_max = matrix[0].len();
    let y_max = matrix.len();

    let x_coords = (x - 1..=x + 1).filter(|x_i| *x_i >= 0 && *x_i < x_max as i32);
    let y_coords = (y - 1..=y + 1).filter(|y_i| *y_i >= 0 && *y_i < y_max as i32);

    let mut neighbors = Vec::new();
    x_coords.for_each(|x_i| {
        y_coords.clone().filter(|y_i| x_i != x || *y_i != y).for_each(|y_i| {
            let (x_i, y_i) = (x_i as usize, y_i as usize);
            let c = matrix[y_i][x_i];
            neighbors.push((x_i, y_i))
        });
    });

    neighbors
}

pub struct Schematic {
    matrix: Vec<Vec<char>>,
    rows: usize,
    columns: usize,
}

impl Schematic {
    pub fn new(input: &str) -> Self {
        let mut input = input.to_string();
        if input.ends_with('\n') {
            input.pop();
        }
        let matrix =
            input.split('\n').map(|line| line.chars().collect_vec()).collect_vec();

        let rows = matrix.len();
        let columns = matrix[0].len();

        Schematic {
            matrix,
            rows,
            columns,
        }
    }

    // /// Check if a coordinate is a part number fragment. If so return the full
    // /// number and the left most coordinate (start).
    // pub fn get_part_number(&self, x: usize, y: usize) -> Option<(usize, usize)> {
    //     let c = self.matrix[y][x];
    //     if !c.is_ascii_digit() {
    //         return None;
    //     }

    //     // find start coordinate (left most digit of part number)
    //     let mut start = x;
    //     for x_i in x..=0 {
    //         let c = self.matrix[y][x_i];
    //         if c.is_ascii_digit() {
    //             start = x_i;
    //         } else {
    //             break;
    //         }
    //     }

    //     // read left to right from start
    //     let mut digits = Vec::new();
    //     let x_max = self.matrix[y].len();
    //     for x_i in start..x_max {
    //         let c = self.matrix[y][x_i];
    //         if c.is_ascii_digit() {
    //             digits.push(c)
    //         } else {
    //             break;
    //         }
    //     }

    //     // parse digits into number
    //     let part_number: usize = digits.into_iter().join("").parse().unwrap();
    //     Some((part_number, start))
    // }

    pub fn get_neighbors(&self, x: usize, y: usize) -> BTreeMap<(usize, usize), char> {
        let (x, y) = (x as i32, y as i32);
        let mut neighbors = BTreeMap::new();

        (x - 1..=x + 1).filter(|x_i| *x_i >= 0 && *x_i < self.columns as i32).for_each(
            |x_i| {
                (y - 1..=y + 1)
                    .filter(|y_i| *y_i >= 0 && *y_i < self.rows as i32)
                    .for_each(|y_i| {
                        if x_i != x || y_i != y {
                            let (x_i, y_i) = (x_i as usize, y_i as usize);
                            let c = self.matrix[y_i][x_i];
                            neighbors.insert((x_i, y_i), c);
                        }
                    });
            },
        );
        neighbors
    }
}

#[allow(dead_code)]
#[test]
fn part_1() -> Result<(), Report> {
    let expected = 539590;
    let observed = run(&Part::Part1)?;
    assert_eq!(observed, expected);
    Ok(())
}

#[allow(dead_code)]
#[test]
fn part_2() -> Result<(), Report> {
    let expected = 80703636;
    let observed = run(&Part::Part2)?;
    assert_eq!(observed, expected);
    Ok(())
}
