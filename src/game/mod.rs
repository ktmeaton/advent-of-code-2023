use crate::utils::table::Table;

use color_eyre::eyre::{Report, Result};
use itertools::Itertools;
use std::str::FromStr;

#[derive(Clone)]
pub struct Map {
    pub tiles: Vec<Vec<char>>,
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

    /// Check if a map coordinate is a character. If so return the full
    /// number and the coordinate range it spans.
    pub fn get_character(&self, x: usize, y: usize) -> Option<(usize, Vec<(usize, usize)>)> {
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

        for x_i in start..self.tiles[y].len() {
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

    pub fn distance(&self, t1: (usize, usize), t2: (usize, usize)) -> usize {
        ((t2.0 as isize - t1.0 as isize).abs() + (t2.1 as isize - t1.1 as isize).abs()) as usize
    }

    /// Make this not single character
    pub fn search(&self, query: &char) -> Vec<(usize, usize)> {
        self.tiles
            .iter()
            .enumerate()
            .filter(|(_y, row)| row.contains(query))
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, c)| (c == query).then_some((x, y)))
                    .collect_vec()
            })
            .collect_vec()
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
                            Tile::Character => self.get_character(x, y).map(|(_, coords)| coords),
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
        let (x, y) = (x as isize, y as isize);
        (x - 1..=x + 1)
            .flat_map(|x| (y - 1..=y + 1).map(|y| (x, y)).collect_vec())
            .filter(|(x_i, y_i)| (*x_i != x || *y_i != y))
            .filter(|(x, y)| self.is_tile(*x, *y))
            .map(|(x, y)| (x as usize, y as usize))
            .collect_vec()
    }

    // Get neighbors cells in cross pattern, no diagnoal
    pub fn get_simple_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let (x, y) = (x as isize, y as isize);
        vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .into_iter()
            .filter(|(x, y)| self.is_tile(*x, *y))
            .map(|(x, y)| (x as usize, y as usize))
            .collect_vec()
    }

    // Get neighbors based on a pipe
    pub fn get_pipe_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let c = self.tiles[y][x];
        let (x, y) = (x as i32, y as i32);

        let n: Vec<(i32, i32)> = match c {
            'S' | '+' => vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)],
            '|' => vec![(x, y - 1), (x, y + 1)],
            '-' => vec![(x - 1, y), (x + 1, y)],
            'L' => vec![(x, y - 1), (x + 1, y)],
            'J' => vec![(x - 1, y), (x, y - 1)],
            '7' => vec![(x - 1, y), (x, y + 1)],
            'F' => vec![(x, y + 1), (x + 1, y)],
            _ => vec![],
        };

        // filter to valid coord
        n.into_iter()
            // filter to valid coordinates
            .filter_map(|(x, y)| (x >= 0 && y >= 0).then_some((x as usize, y as usize)))
            .filter(|(_x, y)| *y < self.tiles.len())
            .filter(|(x, y)| *x < self.tiles[*y].len())
            .collect_vec()
    }

    // check if coordiantes are in the tile map boundary
    pub fn is_tile(&self, x: isize, y: isize) -> bool {
        match x >= 0 && y >= 0 {
            true => {
                let (x, y) = (x as usize, y as usize);
                match y < self.tiles.len() {
                    true => x < self.tiles[y].len(),
                    false => false,
                }
            }
            false => false,
        }
    }

    /// flood fill algorithm, but following pipes
    pub fn flood_fill(&self, x: usize, y: usize, follow_pipes: bool) -> Vec<(usize, usize)> {
        let mut f = Vec::new();

        // if not following pipes, we can only fill ground ('.')
        if !follow_pipes && self.tiles[y][x] != '.' {
            return f;
        }
        let mut current = vec![(x, y)];

        while !current.is_empty() {
            current = current
                .into_iter()
                .flat_map(|(x, y)| {
                    let mut next_nodes = match follow_pipes {
                        true => self
                            .get_pipe_neighbors(x, y)
                            .into_iter()
                            .filter(|(xn, yn)| self.get_pipe_neighbors(*xn, *yn).contains(&(x, y)))
                            .collect_vec(),
                        false => self
                            .get_simple_neighbors(x, y)
                            .into_iter()
                            .filter(|(x, y)| self.tiles[*y][*x] == '.')
                            .collect_vec(),
                    };

                    // don't backtrack
                    next_nodes.retain(|(x, y)| !f.contains(&(*x, *y)));

                    f.extend(next_nodes.clone());
                    next_nodes
                })
                .collect_vec();
        }

        f
    }

    // /// flood fill algorithm
    // pub fn flood_fill(
    //     &self,
    //     x: usize,
    //     y: usize,
    //     mut f: Vec<(usize, usize)>,
    // ) -> Vec<(usize, usize)> {
    //     f.push((x, y));

    //     let next_nodes = self
    //         .get_pipe_neighbors(x, y)
    //         .into_iter()
    //         // don't backtrack
    //         .filter(|(xn, yn)| !f.contains(&(*xn, *yn)))
    //         // don't enter start node
    //         .filter(|(xn, yn)| self.tiles[*yn][*xn] != 'S')
    //         // make sure pipe goes both ways
    //         .filter(|(xn, yn)| self.get_pipe_neighbors(*xn, *yn).contains(&(x, y)))
    //         .collect_vec();

    //     for (x, y) in next_nodes {
    //         if f.contains(&(x, y)) {
    //             continue;
    //         }
    //         f = self.flood_fill(x, y, f.clone());
    //     }

    //     f
    // }

    pub fn zoom_in(&mut self) {
        self.push_pipe_columns();
        self.push_pipe_rows();
    }

    pub fn zoom_out(&mut self) {
        self.tiles = self
            .tiles
            .clone()
            .into_iter()
            .enumerate()
            .filter_map(|(y, row)| (y % 2 == 0).then_some(row))
            .map(|row| row.into_iter().step_by(2).collect_vec())
            .collect_vec();
    }

    /// push apart each pipe row
    pub fn push_pipe_rows(&mut self) {
        self.tiles = self
            .tiles
            .iter()
            .enumerate()
            .flat_map(|(y, row)| match y == self.tiles.len() - 1 {
                true => vec![row.clone()],
                false => {
                    let new_row = (0..row.len())
                        .map(|x| {
                            let n1 = self.get_pipe_neighbors(x, y);
                            let n2 = self.get_pipe_neighbors(x, y + 1);
                            match n1.contains(&(x, y + 1)) && n2.contains(&(x, y)) {
                                true => '|',
                                false => '*',
                            }
                        })
                        .collect_vec();
                    vec![row.clone(), new_row]
                }
            })
            .collect_vec();
    }

    /// push apart each pipe column
    pub fn push_pipe_columns(&mut self) {
        self.tiles = (0..self.tiles.len())
            .map(|y| {
                (0..self.tiles[y].len())
                    .flat_map(|x| {
                        let c = self.tiles[y][x];
                        match x == self.tiles[y].len() - 1 {
                            true => vec![c],
                            false => {
                                let n1 = self.get_pipe_neighbors(x, y);
                                let n2 = self.get_pipe_neighbors(x + 1, y);
                                let n = match n1.contains(&(x + 1, y)) && n2.contains(&(x, y)) {
                                    true => '-',
                                    false => '*',
                                };
                                vec![c, n]
                            }
                        }
                    })
                    .collect_vec()
            })
            .collect();
    }

    pub fn pretty_print(&self) -> Result<String, Report> {
        let mut table = Table::new();
        table.headers = (0..self.tiles[0].len()).map(|n| n.to_string()).collect_vec();
        table.headers.insert(0, "y/x".to_string());
        table.rows = self
            .tiles
            .iter()
            .enumerate()
            .map(|(y, row)| {
                let mut row = row.iter().map(|c| c.to_string()).collect_vec();
                row.insert(0, y.to_string());
                row
            })
            .collect_vec();
        let mut md = table.to_markdown()?;
        md = md.replace("| |", "| X");
        md = md.replace('|', "");
        md = md.replace('X', "|");
        Ok(md)
    }
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

// ----------------------------------------------------------------------------
// Tiles

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Character,
    Trap,
    Enemy,
    Base,
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

#[derive(Clone, Copy)]
pub enum Fill {
    Flood,
    Loop,
}
