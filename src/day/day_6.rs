use crate::{day::Part, utils};

use color_eyre::eyre::{Report, Result};
use itertools::Itertools;
use log::debug;
use log::info;

/// Day 6 - Boat Races
pub fn run(part: &Part) -> Result<usize, Report> {
    // parse puzzle input into times and distances
    let input = utils::read_to_string("data/day_6.txt")?;
    let lines = input.split('\n').collect_vec();

    let mut times: Vec<usize> = lines[0].split(' ').filter_map(|c| c.parse().ok()).collect();
    let mut dists: Vec<usize> = lines[1].split(' ').filter_map(|c| c.parse().ok()).collect();

    if *part == Part::Part2 {
        let t = times.iter().map(|n| n.to_string()).join("");
        let d = dists.iter().map(|n| n.to_string()).join("");
        times = vec![t.parse().unwrap()];
        dists = vec![d.parse().unwrap()];
    }

    let mut result = None;

    times.into_iter().zip(dists).for_each(|(t, d)| {
        // d = h * (t - h) = ht -h^2

        debug!("time: {t}, distance: {d}");
        let mut min_solution = 0;
        // get minimum hold solution
        for h in 1..t {
            let win = h * t - h.pow(2) > d;
            if win {
                min_solution = h;
                break;
            }
        }
        debug!("\tmin_solution: {min_solution}");
        let mut max_solution = 0;
        // maximum hold solution
        for h in (1..t).rev() {
            let win = h * t - h.pow(2) > d;
            if win {
                max_solution = h;
                break;
            }
        }
        debug!("\tmax_solution: {max_solution}");

        let solutions = 1 + max_solution - min_solution;
        debug!("\tsolutions: {solutions}");

        result = match result {
            Some(r) => Some(r * solutions),
            None => Some(solutions),
        };
    });

    let result = result.unwrap_or_default();

    info!("Answer: {result}");
    Ok(result)
}

#[test]
fn part_1() -> Result<(), Report> {
    let expected = 5133600;
    let observed = run(&Part::Part1)?;
    assert_eq!(observed, expected);
    Ok(())
}

#[test]
fn part_2() -> Result<(), Report> {
    let expected = 40651271;
    let observed = run(&Part::Part2)?;
    assert_eq!(observed, expected);
    Ok(())
}
