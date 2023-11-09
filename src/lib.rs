pub mod cli;

use clap::ValueEnum;
use serde::Serialize;

/// Calendar Day
#[derive(Clone, Debug, Serialize, ValueEnum)]
pub enum Day {
    D1,
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Convert to lowercase for RUST_LOG env var compatibility
        let lowercase = format!("{:?}", self).to_lowercase();
        write!(f, "{lowercase}")
    }
}
