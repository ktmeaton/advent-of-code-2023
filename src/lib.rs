pub mod day;

use clap::{Parser, ValueEnum};
use crate::day::*;
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
