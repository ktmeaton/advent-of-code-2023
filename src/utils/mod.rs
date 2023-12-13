use color_eyre::eyre::{Report, Result};
use std::path::PathBuf;

pub fn read_to_string(path: &str) -> Result<String, Report> {
    let path = PathBuf::from(path);
    let mut input = std::fs::read_to_string(path)?;

    if input.ends_with('\n') || input.ends_with('\r') {
        input.pop();
    }

    Ok(input)
}
