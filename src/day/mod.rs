pub mod d1;

use clap::ValueEnum;
use serde::Serialize;
use strum::EnumIter;

/// Calendar Day
#[derive(Clone, Copy, Debug, EnumIter, Serialize, PartialEq, ValueEnum)]
pub enum Day {
    All = 0,
    D1 = 1,
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Convert to lowercase for RUST_LOG env var compatibility
        let lowercase = format!("{:?}", self).to_lowercase();
        write!(f, "{lowercase}")
    }
}
