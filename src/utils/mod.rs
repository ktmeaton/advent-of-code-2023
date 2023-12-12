use color_eyre::eyre::{Report, Result};

pub fn read_to_string(path: &std::path::Path) -> Result<String, Report> {
    let mut input = std::fs::read_to_string(path)?;

    if input.ends_with('\n') || input.ends_with('\r') {
        input.pop();
    }

    Ok(input)
}
