pub mod day;
pub mod game;
pub mod poker;
pub mod utils;

use crate::day::*;
use clap::{Parser, ValueEnum};
use color_eyre::eyre::{eyre, Report, Result};
use log::info;
use std::default::Default;
use std::str::FromStr;

// ----------------------------------------------------------------------------
// Puzzle dispatcher
// ----------------------------------------------------------------------------

/// Run puzzle day and part.
pub fn run(args: &Cli) -> Result<(), Report> {
    info!("Day {}", args.day);

    let part = <Part as FromStr>::from_str(&args.part)?;

    match args.day.as_ref() {
        "1" => day_1::run(&part)?,
        "2" => day_2::run(&part)?,
        "3" => day_3::run(&part)?,
        "4" => day_4::run(&part)?,
        "5" => day_5::run(&part)?,
        "6" => day_6::run(&part)?,
        "7" => day_7::run(&part)?,
        "8" => day_8::run(&part)?,
        "9" => day_9::run(&part)?,
        "10" => day_10::run(&part)?,
        "11" => day_11::run(&part)?,
        _ => return Err(eyre!("Day {} is not implemented yet.", args.day)),
    };

    Ok(())
}

// ----------------------------------------------------------------------------
// CLI Entry Point
// ----------------------------------------------------------------------------

// Text Art Generator: https://fsymbols.com/generators/carty/
/// ░█████╗░██████╗░██╗░░░██╗███████╗███╗░░██╗████████╗  ░█████╗░███████╗  ░█████╗░░█████╗░██████╗░███████╗  ██████╗░░█████╗░██████╗░██████╗░
/// ██╔══██╗██╔══██╗██║░░░██║██╔════╝████╗░██║╚══██╔══╝  ██╔══██╗██╔════╝  ██╔══██╗██╔══██╗██╔══██╗██╔════╝  ╚════██╗██╔══██╗╚════██╗╚════██╗
/// ███████║██║░░██║╚██╗░██╔╝█████╗░░██╔██╗██║░░░██║░░░  ██║░░██║█████╗░░  ██║░░╚═╝██║░░██║██║░░██║█████╗░░  ░░███╔═╝██║░░██║░░███╔═╝░█████╔╝
/// ██╔══██║██║░░██║░╚████╔╝░██╔══╝░░██║╚████║░░░██║░░░  ██║░░██║██╔══╝░░  ██║░░██╗██║░░██║██║░░██║██╔══╝░░  ██╔══╝░░██║░░██║██╔══╝░░░╚═══██╗
/// ██║░░██║██████╔╝░░╚██╔╝░░███████╗██║░╚███║░░░██║░░░  ╚█████╔╝██║░░░░░  ╚█████╔╝╚█████╔╝██████╔╝███████╗  ███████╗╚█████╔╝███████╗██████╔╝
/// ╚═╝░░╚═╝╚═════╝░░░░╚═╝░░░╚══════╝╚═╝░░╚══╝░░░╚═╝░░░  ░╚════╝░╚═╝░░░░░  ░╚════╝░░╚════╝░╚═════╝░╚══════╝  ╚══════╝░╚════╝░╚══════╝╚═════╝░
#[derive(Clone, Parser, Debug)]
#[clap(name = "advent-of-code-2023", trailing_var_arg = true)]
#[clap(author, version)]
#[clap(verbatim_doc_comment)]
#[clap(arg_required_else_help = true)]
pub struct Cli {
    /// Output verbosity level.
    #[clap(short = 'v', long)]
    #[clap(value_enum, default_value_t = Verbosity::default())]
    #[clap(hide_possible_values = false)]
    pub verbosity: Verbosity,

    /// Puzzle day number.
    #[clap(short = 'd', long, required = true)]
    pub day: String,

    /// Puzzle part.
    #[clap(short = 'p', long, required = true)]
    pub part: String,
}

// -----------------------------------------------------------------------------
// Verbosity
// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Default, ValueEnum)]
pub enum Verbosity {
    #[default]
    Info,
    Warn,
    Debug,
    Error,
}

impl std::fmt::Display for Verbosity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Convert to lowercase for RUST_LOG env var compatibility
        let lowercase = format!("{:?}", self).to_lowercase();
        write!(f, "{lowercase}")
    }
}
