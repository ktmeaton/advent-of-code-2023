use crate::day::Part;
use crate::utils;

use color_eyre::eyre::{Report, Result};
use core::ops::Range;
use itertools::Itertools;
use log::{debug, info};
use std::cmp::{max, min};

/// Day 5
///
/// The Almanac contains mappings of seed => soil => fertilizer ... => location
///
pub fn run(part: &Part) -> Result<usize, Report> {
    let path = std::path::PathBuf::from("data/day_5.txt");
    let input = utils::read_to_string(&path)?;

    // parse the seeds as the first source
    let lines = input.split("\n\n").filter(|l| !l.is_empty()).collect_vec();
    let sources: Vec<usize> =
        lines[0].split(' ').filter_map(|s| s.parse().ok()).collect();

    // convert to ranges
    let mut sources = match *part {
        Part::Part1 => sources.into_iter().map(|n| n..n).collect_vec(),
        Part::Part2 => {
            sources.chunks(2).into_iter().map(|v| v[0]..v[0] + v[1]).collect_vec()
        }
    };
       
    // iterate through each mapping of range(source) => range(destination)
    lines[1..].iter().for_each(|line| {
        let line_split = line.split(" map:\n").collect_vec();

        debug!("{}", "-".repeat(80)); 
        debug!("connection: {:?}", line_split[0]);

        sources.sort_by(|a, b| a.start.cmp(&b.start));
        debug!("\tsources: {sources:?}");

        // map sources to new destinations
        let (map_sources, map_destinations): (Vec<_>, Vec<_>) = line_split[1]
            .split('\n')
            .map(|l| {
                let m = l.split(' ').filter_map(|c| c.parse::<usize>().ok()).collect_vec();
                ((m[1]..m[1]+m[2]), (m[0]..m[0]+m[2]))
            })
            .unzip();

        debug!("\tmap: {map_sources:?} => {map_destinations:?}");

        // split up sources into ranges that overlap/don't overlap with map
        let sources_split = sources
            .iter()
            .flat_map(|s| split_source(s, &map_sources).unwrap())
            .collect_vec();
        debug!("\tsources_split: {sources_split:?}");

        let destinations = sources_split
            .into_iter()
            .map(|s| {
                debug!("\tsource: {s:?}");
                let result = map_sources.iter().enumerate().find(|(_i, r)| r.contains(&s.start) && r.contains(&(s.end-1)));

                if let Some((i, map_s)) = result {
                    let map_d = &map_destinations[i];
                    let conversion = map_d.start as isize - map_s.start as isize;
                    debug!("\t\tmap_s: {map_s:?} => map_d: {map_d:?}, conversion: {conversion:?}");    
                    let start = ((s.start as isize) + conversion) as usize;
                    let end = ((s.end as isize) + conversion) as usize;
                    start..end
                }
                else {
                    s
                }
            })
            .collect_vec();
        debug!("\tdestinations: {destinations:?}");
        sources = destinations;
    });

    let destinations = sources;

    let result = destinations.into_iter().map(|r| r.start).min().unwrap();

    //let result = 0;
    info!("Answer: {result}");

    // // let s = 90..99;
    // // let t = vec![56..93, 93..97];

    // let s = 55..68;
    // let t = vec![98..100, 50..98];

    // let s = 82..83;
    // let t = vec![15..52, 52..54, 0..15];
    // let split = split_source(&s, &t)?;
    // debug!("split: {split:?}");

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
