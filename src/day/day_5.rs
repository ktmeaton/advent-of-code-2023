use crate::day::Part;
use crate::utils;

use color_eyre::eyre::{Report, Result};
use core::ops::Range;
use itertools::Itertools;
use log::info;
use std::cmp::{max, min};
use std::path::PathBuf;

/// Day 5
///
/// Planting seeds with the help of the Almanac.
pub fn run(part: &Part) -> Result<usize, Report> {
    // Read in puzzle input
    let path = PathBuf::from("data/day_5.txt");
    let input = utils::read_to_string(&path)?;
    let lines = input.split("\n\n").filter(|l| !l.is_empty()).collect_vec();

    // Parse seeds as the first source
    let seeds = lines[0].split(' ').filter_map(|s| s.parse::<usize>().ok()).collect_vec();

    // convert to ranges, varies based on Part 1 or Part 2
    let mut sources = match *part {
        Part::Part1 => seeds.into_iter().map(|n| n..n).collect_vec(),
        Part::Part2 => seeds.chunks(2).map(|v| v[0]..v[0] + v[1]).collect_vec(),
    };

    // iterate through mappings of range(source) => range(destination)
    let maps = lines[1..].iter();
    maps.for_each(|line| {
        // split line into connection (ex. seed-to-soil) and numbers
        let line_split = line.split(" map:\n").collect_vec();

        // parse mapping, into sources (s) and destinations (s)
        let (map_s, map_d): (Vec<_>, Vec<_>) = line_split[1]
            .split('\n')
            .map(|l| {
                let m =
                    l.split(' ').filter_map(|c| c.parse::<usize>().ok()).collect_vec();
                ((m[1]..m[1] + m[2]), (m[0]..m[0] + m[2]))
            })
            .unzip();

        // split up sources into ranges that overlap/don't overlap with map
        let sources_split =
            sources.iter().flat_map(|s| split_source(s, &map_s).unwrap()).collect_vec();

        // get the new destinations for each source
        let destinations = sources_split
            .into_iter()
            .map(|s| {
                let result = map_s
                    .iter()
                    .enumerate()
                    .find(|(_i, r)| r.contains(&s.start) && r.contains(&(s.end - 1)));

                if let Some((i, map_s)) = result {
                    let map_d = &map_d[i];
                    let conversion = map_d.start as isize - map_s.start as isize;
                    let start = ((s.start as isize) + conversion) as usize;
                    let end = ((s.end as isize) + conversion) as usize;
                    start..end
                } else {
                    s
                }
            })
            .collect_vec();
        sources = destinations;
    });

    let result = sources.into_iter().map(|r| r.start).min().unwrap();

    info!("Answer: {result}");

    Ok(result)
}

pub fn split_source(
    source: &Range<usize>,
    target: &Vec<Range<usize>>,
) -> Result<Vec<Range<usize>>, Report> {
    let split = vec![source.clone()];
    // check if source and target are exactly the same
    if target.len() == 1 && *source == target[0] {
        return Ok(split);
    }
    let s = source;
    let mut target = target.clone();
    target.sort_by(|a, b| a.start.cmp(&b.start));

    //debug!("source: {source:?}, target: {target:?}");
    // get overlaps between source and target
    let overlaps = target
        .iter()
        .filter(|t| !(t.end < s.start || t.start > s.end))
        .map(|t| max(s.start, t.start)..min(s.end, t.end))
        .collect_vec();
    //debug!("overlaps: {overlaps:?}");

    if overlaps.is_empty() {
        return Ok(split);
    }

    // add non_overlapping regions at beginning, in between and at end
    let mut non_overlaps = Vec::new();
    let mut end = s.start;
    overlaps.iter().for_each(|o| {
        if o.start > end {
            non_overlaps.push(end..o.start);
        }
        end = o.end;
    });
    // add non_overlapping regions at end
    let overlap_max = overlaps.iter().map(|r| r.end).max().unwrap_or(s.end);
    if overlap_max < s.end {
        non_overlaps.push(overlap_max..s.end);
    }

    // combine overlaps and non-overlaps
    let mut split = overlaps;
    split.append(&mut non_overlaps);
    split.sort_by(|a, b| a.start.cmp(&b.start));

    Ok(split)
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
    let expected = 57451709;
    let observed = run(&Part::Part2)?;
    assert_eq!(observed, expected);
    Ok(())
}
