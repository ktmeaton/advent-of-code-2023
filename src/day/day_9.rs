use crate::day::Part;
use crate::utils;
use crate::utils::table::Table;
use color_eyre::eyre::{Report, Result};
use itertools::Itertools;
use log::debug;
use log::info;

/// Day 9 - Oasi
pub fn run(part: &Part) -> Result<usize, Report> {
    let input = utils::read_to_string("data/day_9.txt")?;

    let extrapolated_values = input
        .split('\n')
        .map(|l| {
            // parse into vector of numbers
            let mut v = l
                .split(' ')
                .flat_map(|s| {
                    let n = s.parse::<isize>().unwrap();
                    vec![Some(n), None::<isize>]
                })
                .collect_vec();

            // expand out to left and right
            match *part {
                Part::Part1 => v.push(None),
                Part::Part2 => {
                    v.insert(0, None);
                    v.insert(0, None);
                }
            };

            let mut rows = vec![v.clone()];
            let mut y = 0;
            let max_x = rows[y].len() - 1;

            // extend table downward with diffs
            loop {
                let mut next_row = vec![None::<isize>; max_x + 1];

                rows[y].iter().enumerate().filter(|(x, _)| *x < max_x - 1).for_each(|(x, r)| {
                    let next_r = rows[y][x + 2];
                    let diff = match r.is_some() && next_r.is_some() {
                        true => Some(next_r.unwrap() - r.unwrap()),
                        false => None,
                    };
                    next_row[x + 1] = diff;
                });

                // check if all our diffs are the same
                let diffs = next_row.iter().filter(|r| r.is_some()).unique().cloned().collect_vec();

                // on the final row, extrapolate on element out
                //rows.push(next_row);
                y += 1;
                rows.push(next_row);
                if diffs.len() == 1 {
                    break;
                }
            }

            // extrapolate out bottom row: part 1 => R, L <= part 2
            match *part {
                Part::Part1 => {
                    let last_x = max_x - rows[y].iter().rev().position(|r| r.is_some()).unwrap();
                    rows[y][last_x + 2] = rows[y][last_x];
                }
                Part::Part2 => {
                    let first_x = rows[y].iter().position(|r| r.is_some()).unwrap();
                    rows[y][first_x - 2] = rows[y][first_x];
                }
            };

            // extrapolate
            while y > 0 {
                let curr_row = &rows[y];
                let mut prev_row = rows[y - 1].clone();

                let curr_x = match *part {
                    Part::Part1 => max_x - curr_row.iter().rev().position(|r| r.is_some()).unwrap(),
                    Part::Part2 => curr_row.iter().position(|r| r.is_some()).unwrap(),
                };
                let curr_n = curr_row[curr_x].unwrap();
                let prev_n = match *part {
                    Part::Part1 => prev_row[curr_x - 1].unwrap(),
                    Part::Part2 => prev_row[curr_x + 1].unwrap(),
                };

                match *part {
                    Part::Part1 => prev_row[curr_x + 1] = Some(prev_n + curr_n),
                    Part::Part2 => prev_row[curr_x - 1] = Some(prev_n - curr_n),
                };

                rows[y - 1] = prev_row;

                y -= 1;
            }

            // create debug table
            let mut table = Table::new();
            table.headers = v.iter().enumerate().map(|(i, _)| i.to_string()).collect_vec();
            table.rows = rows
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|r| match r {
                            Some(n) => n.to_string(),
                            None => "".to_string(),
                        })
                        .collect_vec()
                })
                .collect_vec();
            debug!("{l}\n{}", table.to_markdown()?);

            // get final extrapolated value
            let final_value = match *part {
                Part::Part1 => rows[0][max_x].unwrap(),
                Part::Part2 => rows[0][0].unwrap(),
            };
            Ok(final_value)
        })
        .collect::<Result<Vec<_>, Report>>()?;

    // risky conversion!
    let result = extrapolated_values.iter().sum::<isize>() as usize;

    info!("Answer: {result}");
    Ok(result)
}

#[test]
fn part_1() -> Result<(), Report> {
    let expected = 1938731307;
    let observed = run(&Part::Part1)?;
    assert_eq!(observed, expected);
    Ok(())
}

#[test]
fn part_2() -> Result<(), Report> {
    let expected = 948;
    let observed = run(&Part::Part2)?;
    assert_eq!(observed, expected);
    Ok(())
}
