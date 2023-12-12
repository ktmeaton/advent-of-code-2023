use crate::day::Part;
use crate::utils;

use color_eyre::eyre::{Report, Result};
use itertools::Itertools;
use log::info;

/// Day 5
///
/// The Almanac contains mappings of seed => soil => fertilizer ... => location
///
pub fn run(part: &Part) -> Result<usize, Report> {
    let path = std::path::PathBuf::from("data/day_5.txt");
    let input = utils::read_to_string(&path)?;

    // parse the seeds as the first source
    let lines = input.split("\n\n").filter(|l| !l.is_empty()).collect_vec();
    let mut sources: Vec<usize> =
        lines[0].split(' ').filter_map(|s| s.parse().ok()).collect();

    // iterate through each mapping of source => destination
    lines[1..].iter().for_each(|line| {
        let line_split = line.split(" map:\n").collect_vec();

        let lookup = line_split[1]
            .split('\n')
            .map(|set| {
                set.split(' ').filter_map(|c| c.parse::<usize>().ok()).collect_vec()
            })
            .collect_vec();

        let source_ranges = lookup.iter().map(|l| (l[1]..l[1] + l[2])).collect_vec();
        let dest_ranges = lookup.iter().map(|l| (l[0]..l[0] + l[2])).collect_vec();

        let destinations = sources
            .iter()
            .map(|n| {
                let result =
                    source_ranges.iter().enumerate().find(|(_i, r)| r.contains(n));
                match result {
                    Some((i, s)) => {
                        let d = &dest_ranges[i];
                        let s_min = s.clone().min().unwrap();
                        let d_min = d.clone().min().unwrap();
                        let conversion = d_min as isize - s_min as isize;
                        ((*n as isize) + conversion) as usize
                    }
                    None => *n,
                }
            })
            .collect_vec();
        sources = destinations;
    });

    let destinations = sources;

    let result = match *part {
        Part::Part1 => destinations.into_iter().min().unwrap(),
        Part::Part2 => 2,
    };

    info!("Answer: {result}");
    Ok(result)
}

#[test]
fn part_1() -> Result<(), Report> {
    let expected = 551761867;
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
