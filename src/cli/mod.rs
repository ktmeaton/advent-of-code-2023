use crate::day::Day;
use clap::{Parser, ValueEnum};
use serde::Serialize;
use std::default::Default;

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

    /// Run puzzle from day number
    #[clap(short = 'd', long, required = true)]
    #[clap(hide_possible_values = false)]
    pub day: Day,
}

// -----------------------------------------------------------------------------
// Verbosity
// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Default, Serialize, ValueEnum)]
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
