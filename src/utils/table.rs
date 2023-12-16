use color_eyre::eyre::{eyre, Report, Result};
use itertools::Itertools;
use std::default::Default;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Table {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub path: PathBuf,
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl Table {
    pub fn new() -> Self {
        Table {
            path: PathBuf::new(),
            headers: Vec::new(),
            rows: Vec::new(),
        }
    }

    pub fn header_position(&self, header: &str) -> Result<usize, Report> {
        let pos =
            self.headers.iter().position(|h| h == header).ok_or_else(|| {
                eyre!("Column '{header}' was not found in table: {:?}.", self.path)
            })?;

        Ok(pos)
    }

    pub fn filter(&self, header: &str, pattern: &str) -> Result<Table, Report> {
        let mut table = Table::new();
        let header_i = self.header_position(header)?;
        table.headers = self.headers.clone();
        table.rows = self.rows.iter().filter(|row| row[header_i] == pattern).cloned().collect_vec();
        Ok(table)
    }

    /// Convert table to markdown format
    ///
    /// TBD: error handling for empty rows!
    pub fn to_markdown(&self) -> Result<String, Report> {
        // get the maximum width of each column
        let col_widths = self
            // iterate through columns/headers
            .headers
            .iter()
            .enumerate()
            .map(|(col_i, header)| {
                self
                    // iterate through this column's rows,
                    // get max string width, +2 to add space on either side
                    .rows
                    .iter()
                    .map(|row| {
                        let cell_width = (*row[col_i]).len();
                        if cell_width >= header.len() {
                            cell_width + 2
                        } else {
                            header.len() + 2
                        }
                    })
                    .max()
                    .unwrap_or(header.len() + 2)
            })
            .collect_vec();

        let mut markdown = String::from("|");
        // frame in between headers and rows
        let mut header_frame = String::from("|");

        // Create the header line
        for it in self.headers.iter().zip(col_widths.iter()) {
            let (header, col_width) = it;
            let cell = format!("{:^width$}|", header, width = col_width);
            markdown.push_str(&cell);

            let frame = format!("{}|", "-".repeat(*col_width));
            header_frame.push_str(&frame);
        }
        markdown.push('\n');
        markdown.push_str(&header_frame);
        markdown.push('\n');

        // Create the row lines
        for row in &self.rows {
            markdown.push('|');
            for (col_i, col_width) in col_widths.iter().enumerate() {
                let cell = format!("{:^width$}|", row[col_i], width = col_width);
                markdown.push_str(&cell);
            }
            markdown.push('\n');
        }

        Ok(markdown)
    }
}
