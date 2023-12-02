use color_eyre::eyre::{Report, Result};
use log::info;
use itertools::Itertools;

/// Day 1 - Sum of calibration values.
/// 
/// Part 1. On each line of the input, the calibration value can be found by combining
/// the first digit and the last digit (in that order) to form a single 
/// two-digit number.
/// 
/// Part 2. It looks like some of the digits are actually spelled out with letters: 
/// one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".
/// Be warned, names can overlap! Ex. oneight should be both "one" and "eight"
pub fn run() -> Result<(), Report> {

    let digits = vec![  ("one", "1"), ("two", "2"), ("three", "3"), ("four",  "4"),
        ("five",  "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9")];

    // read in the input (calibration document)
    let content = std::fs::read_to_string("data/d1.txt")?;
    let lines = content.split("\n").collect_vec();

    let mut p1_total = 0;
    let mut p2_total = 0;

    for line in lines {

        let mut search = Vec::new();

        // for both part 1 and part 2 we search by digit
        for (_n, d) in &digits {
            let mut digit_search = line.match_indices(d).collect_vec();
            search.append(&mut digit_search);
        }
    
        for part in 1..=2 {

            // in part two, we also search by name ("one" => "1")
            if part == 2 {
                for (n, d) in &digits {
                    let mut name_search  = line.match_indices(n).map(|(i, _n)| (i, *d)).collect_vec();
                    search.append(&mut name_search);
                }
            }

            // combine first and last numbers into digit
            let first = search.iter().min_by(|a, b| a.0.cmp(&b.0)).map(|(_i, d)| d).unwrap();
            let last  = search.iter().max_by(|a, b| a.0.cmp(&b.0)).map(|(_i, d)| d).unwrap();
            let digit: usize = format!("{first}{last}").parse()?;

            match part {
                1 => p1_total += digit,
                2 => p2_total += digit,
                _ => (),
            }        
        }

    }

    info!("Part 1: {p1_total}");
    info!("Part 2: {p2_total}");

    Ok(())
}
