use color_eyre::eyre::{Report, Result};
use itertools::Itertools;
use std::str::FromStr;

pub struct Map {
    tiles: Vec<Vec<char>>,
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

impl Map {
    pub fn new() -> Self {
        Map { tiles: Vec::new() }
    }

    pub fn rows(&self) -> usize {
        self.tiles.len()
    }

    pub fn columns(&self) -> usize {
        if self.tiles.is_empty() {
            0
        } else {
            self.tiles[0].len()
        }
    }

    /// Check if a map coordinate is a character. If so return the full
    /// number and the coordinate range it spans.
    pub fn get_character(
        &self,
        x: usize,
        y: usize,
    ) -> Option<(usize, Vec<(usize, usize)>)> {
        // check if the coordinate is a digit
        if !self.tiles[y][x].is_ascii_digit() {
            return None;
        }

        let mut start = x;

        // walk backwards to find start coordinate
        for x_i in (0..x).rev() {
            match self.tiles[y][x_i].is_ascii_digit() {
                true => start = x_i,
                false => break,
            }
        }

        // read left to right from start
        let mut digits = Vec::new();

        for x_i in start..self.columns() {
            let c = self.tiles[y][x_i];
            match c.is_ascii_digit() {
                true => digits.push(c),
                false => break,
            }
        }

        // get coordinate range of digit
        let coordinates = (start..(start + digits.len())).map(|x| (x, y)).collect();

        // parse digits into number
        let part_number: usize = digits.into_iter().join("").parse().unwrap();
        Some((part_number, coordinates))
    }

    pub fn find_tile(&self, tile: &Tile) -> Vec<Vec<(usize, usize)>> {
        let coordinates = self
            .tiles
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        let coords = vec![(x, y)];
                        match *tile {
                            Tile::Character => {
                                self.get_character(x, y).map(|(_, coords)| coords)
                            }
                            Tile::Enemy => (!c.is_ascii_digit()
                                && c.to_string() != Tile::Base.to_string())
                            .then_some(coords),
                            Tile::Base | Tile::Trap => {
                                (c.to_string() == tile.to_string()).then_some(coords)
                            }
                        }
                    })
                    .collect_vec()
            })
            .unique()
            .collect_vec();

        coordinates
    }

    // Get coordinates of all neighboring cells.
    pub fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let (x, y) = (x as i32, y as i32);
        (x - 1..=x + 1)
            .flat_map(|x| (y - 1..=y + 1).map(|y| (x, y)).collect_vec())
            .filter(|(x_i, y_i)| (*x_i != x || *y_i != y) && (*x_i >= 0 && *y_i >= 0))
            .map(|(x, y)| (x as usize, y as usize))
            .filter(|(x, y)| *x < self.columns() && *y < self.rows())
            .collect_vec()
    }
}

// ----------------------------------------------------------------------------
// Tiles

#[derive(PartialEq, Eq)]
pub enum Tile {
    Character,
    Trap,
    Enemy,
    Base,
}

impl FromStr for Map {
    type Err = Report;

    fn from_str(input: &str) -> Result<Self, Report> {
        let mut input = input.to_string();

        if input.ends_with('\n') || input.ends_with('\r') {
            input.pop();
        }
        let tiles = input.split('\n').map(|l| l.chars().collect_vec()).collect_vec();
        let map = Map { tiles };
        Ok(map)
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let component = match self {
            Tile::Character => "character".to_string(),
            Tile::Enemy => "enemy".to_string(),
            Tile::Trap => "*".to_string(),
            Tile::Base => ".".to_string(),
        };

        write!(f, "{}", component)
    }
}
