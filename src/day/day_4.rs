use crate::day::Part;

use color_eyre::eyre::{Report, Result};
use itertools::Itertools;
use log::{debug, info};
use std::str::FromStr;

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Card {
    id: usize,
    numbers: Vec<usize>,
    winning_numbers: Vec<usize>,
    matches: Vec<usize>,
    points: usize,
}

impl Card {
    pub fn new() -> Self {
        Card {
            id: 0,
            numbers: Vec::new(),
            winning_numbers: Vec::new(),
            matches: Vec::new(),
            points: 0,
        }
    }
}

impl FromStr for Card {
    type Err = Report;

    fn from_str(card: &str) -> Result<Self, Report> {
        let pieces = card.split(' ').filter(|p| !p.is_empty()).collect_vec();

        let id = pieces[1].replace(':', "").parse()?;
        let split = pieces.iter().position(|p| *p == "|").unwrap();
        let numbers =
            pieces[2..split].iter().copied().map(|p| p.parse().unwrap()).collect_vec();
        let winning_numbers =
            pieces[split + 1..].iter().copied().map(|p| p.parse().unwrap()).collect_vec();
        let matches =
            numbers.iter().copied().filter(|n| winning_numbers.contains(n)).collect_vec();

        // points = 2^(x-1), where x is number of matches
        let points = match !matches.is_empty() {
            true => 2_usize.pow((matches.len() - 1) as u32),
            false => 0,
        };

        let card = Card {
            id,
            numbers,
            winning_numbers,
            matches,
            points,
        };

        debug!("card: {card:?}");
        Ok(card)
    }
}

/// Day 4
pub fn run(_part: &Part) -> Result<usize, Report> {
    let mut input = std::fs::read_to_string("data/day_4.txt")?;
    if input.ends_with('\n') || input.ends_with('\r') {
        input.pop();
    }

    let cards: Vec<_> =
        input.split('\n').map(|c| Card::from_str(c).unwrap_or_default()).collect();

    let result = cards.iter().map(|c| c.points).sum();

    info!("Answer: {result}");
    Ok(result)
}
#[test]
fn part_1() -> Result<(), Report> {
    let expected = 20407;
    let observed = run(&Part::Part1)?;
    assert_eq!(observed, expected);
    Ok(())
}

#[test]
fn part_2() -> Result<(), Report> {
    let expected = 1;
    let observed = run(&Part::Part2)?;
    assert_eq!(observed, expected);
    Ok(())
}
