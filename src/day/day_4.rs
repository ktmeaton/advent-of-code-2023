use crate::day::Part;

use color_eyre::eyre::{Report, Result};
use itertools::Itertools;
use log::{debug, info};
use std::str::FromStr;

#[derive(Clone, Debug, Default)]
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

    pub fn expand_cards(&self, deck: &Vec<Card>) -> Result<Vec<usize>, Report> {
        let mut ids = vec![self.id];
        let num_copies = self.matches.len();
        let ids_won = ((self.id + 1)..(self.id + 1 + num_copies)).collect_vec();

        ids_won.iter().for_each(|id| {
            let card = &deck[id - 1];
            let ids_rec = card.expand_cards(deck).unwrap_or_default();
            ids.extend(ids_rec);
        });

        Ok(ids)
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

        Ok(card)
    }
}

/// Day 4
pub fn run(part: &Part) -> Result<usize, Report> {
    let mut input = std::fs::read_to_string("data/day_4.txt")?;

    if input.ends_with('\n') || input.ends_with('\r') {
        input.pop();
    }

    let deck: Vec<_> =
        input.split('\n').map(|c| Card::from_str(c).unwrap_or_default()).collect();

    let result = match *part {
        Part::Part1 => deck.iter().map(|c| c.points).sum(),
        Part::Part2 => {
            deck.iter().filter_map(|c| c.expand_cards(&deck).ok()).flatten().count()
        }
    };

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
    let expected = 23806951;
    let observed = run(&Part::Part2)?;
    assert_eq!(observed, expected);
    Ok(())
}
