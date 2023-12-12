use crate::utils;

use color_eyre::eyre::{eyre, ContextCompat, Report, Result};
use color_eyre::Help;
use itertools::Itertools;
use log::debug;
use std::str::FromStr;
use strum::{EnumIter, IntoEnumIterator};

pub fn read(path: &std::path::Path) -> Result<(), Report> {
    let mut input = utils::read_to_string(path)?;
    input = input.replace("-to-", " ");

    // parse the seeds as the first source
    let lines = input.split("\n\n").filter(|l| *l != "").collect_vec();
    let mut sources: Vec<usize> =
        lines[0].split(" ").filter_map(|s| s.parse().ok()).collect();

    for line in &lines[1..] {

        let line_split = line.split(" map:\n").collect_vec();
        debug!("connection: {}", line_split[0]);
        debug!("\tsources: {sources:?}");        

        let lookup = line_split[1]
            .split("\n")
            .map(|set| set.split(" ").filter_map(|c| c.parse::<usize>().ok()).collect_vec())
            .collect_vec();

        let source_ranges = lookup.iter().map(|l| (l[1]..l[1]+l[2])).collect_vec();
        let dest_ranges =  lookup.iter().map(|l| (l[0]..l[0]+l[2])).collect_vec();

        let destinations = sources.iter().map(|n| {
            let result = source_ranges.iter().enumerate().filter(|(_i, r)| r.contains(n)).next();
            match result {
                Some((i, s)) => {
                    let d = &dest_ranges[i];
                    let s_min = s.clone().min().unwrap();
                    let d_min = d.clone().min().unwrap();
                    let conversion = d_min as isize - s_min as isize;
                    //debug!("source_range: {s:?}, dest_range: {d:?}, n: {n}, s_min: {s_min}, d_min: {d_min}, conversion: {conversion}");
                    ((*n as isize) + conversion) as usize
                },
                None => *n,
            }
        }).collect_vec();
        debug!("\tdestinations: {destinations:?}");
        sources = destinations;
    }
    

    Ok(())
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
pub enum Component {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl FromStr for Component {
    type Err = Report;

    fn from_str(component: &str) -> Result<Self, Report> {
        let components: Vec<_> =
            Component::iter().map(|c| format!("{c:?}").to_lowercase()).collect();

        let component = Component::iter()
            .filter_map(|c| {
                let s = format!("{c:?}").to_lowercase();
                (component == s).then_some(c)
            })
            .next()
            .wrap_err(eyre!("Unknown Almanac component: {component:?}"))
            .suggestion(format!("Implemented Almanac components: {components:?}"))?;

        Ok(component)
    }
}
