pub mod cli;
pub mod day;

use crate::cli::Cli;
use crate::day::*;
use color_eyre::eyre::{Report, Result};
use log::info;
use strum::IntoEnumIterator;

/// Run calendar day.
pub fn run(args: &Cli) -> Result<(), Report> {
    if args.day == Day::All {
        for day in Day::iter() {
            if day == Day::All {
                continue;
            }
            let mut day_args = (*args).clone();
            day_args.day = day;
            run(&day_args)?;
        }
    } else {
        info!("Day {}", args.day as u8);
    }

    match args.day {
        Day::D1 => d1::run(),
        _ => Ok(()),
    }
}
