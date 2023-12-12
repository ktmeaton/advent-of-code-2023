use crate::day::Part;
use crate::utils;

use color_eyre::eyre::{Report, Result};
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

        // parsing the mapping of source to new destination
        let maps = line_split[1]
            .split('\n')
            .map(|set| {
                set.split(' ').filter_map(|c| c.parse::<usize>().ok()).collect_vec()
            })
            .collect_vec();

        let map_sources = maps.iter().map(|l| (l[1]..l[1] + l[2])).collect_vec();
        let map_destinations = maps.iter().map(|l| (l[0]..l[0] + l[2])).collect_vec();
        debug!("\tmap: {map_sources:?} => {map_destinations:?}");

        // split up source ranges based on available source mapping
        let mut sources_split = sources.clone().into_iter().flat_map(|s| {
            let matches = map_sources.iter().filter(|r| r.contains(&s.start) || r.contains(&(s.end- 1))).collect_vec();
            let overlaps = matches.iter().map(|m| max(s.start, m.start)..min(s.end, m.end)).collect_vec();
            //debug!("\ts: {s:?}, overlaps: {overlaps:?}");
            if overlaps.is_empty() || overlaps == vec![s.clone()] {
                vec![s]
            } else {
                let mut uniq_start = overlaps.iter().filter_map(|o| (o.start > s.start).then_some(s.start..o.start)).collect_vec();
                let mut uniq_end = overlaps.iter().filter_map(|o| (o.end < s.end).then_some(o.end..s.end)).collect_vec();
                //debug!("\t\tuniq_start: {uniq_start:?}, uniq_end: {uniq_end:?}");
                let mut ranges = overlaps;
                ranges.append(&mut uniq_start);
                ranges.append(&mut uniq_end);
                ranges
            }
        })
        .unique()
        .collect_vec();
        sources_split.sort_by(|a, b| a.start.cmp(&b.start));
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
