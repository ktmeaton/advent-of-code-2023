use clap::Parser;
use color_eyre::eyre::{Report, Result};

fn main() -> Result<(), Report> {
    // ------------------------------------------------------------------------
    // CLI Setup

    // Parse CLI Parameters
    let args = advent_of_code_2023::Cli::parse();

    // initialize color_eyre crate for colorized logs
    color_eyre::install()?;

    // Set logging/verbosity level via RUST_LOG
    std::env::set_var("RUST_LOG", args.verbosity.to_string());

    // initialize env_logger crate for logging/verbosity level
    env_logger::init();

    advent_of_code_2023::run(&args)?;

    Ok(())
}
