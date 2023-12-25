pub mod day_1;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;
pub mod template;
use std::str::FromStr;

use clap::ValueEnum;
use color_eyre::eyre::{eyre, Report, Result};

#[derive(Clone, Copy, Debug, PartialEq, ValueEnum)]
pub enum Part {
    Part1,
    Part2,
}

impl FromStr for Part {
    type Err = Report;

    fn from_str(part: &str) -> Result<Self, Report> {
        let part = match part {
            "1" => Part::Part1,
            "2" => Part::Part2,
            _ => return Err(eyre!("Unknown part {part:?}")),
        };

        Ok(part)
    }
}
