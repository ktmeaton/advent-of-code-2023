use crate::day::Part;
use crate::poker;
use crate::utils;

use color_eyre::eyre::{Report, Result};
use itertools::Itertools;
use log::info;

/// Day 7 - Camel Cards (aka poker)
pub fn run(part: &Part) -> Result<usize, Report> {
    // read in puzzle input
    let input = utils::read_to_string("data/day_7.txt")?;
    let lines = input.split('\n').map(|l| l.split(' ').collect_tuple().unwrap()).collect_vec();

    // parse cards and bids into poker hands
    let mut hands = lines
        .into_iter()
        .map(|(hand, bid)| {
            let cards = hand.chars().collect_vec();
            let bid = bid.parse::<usize>().unwrap();
            let hand = poker::Hand::from_cards(&cards, bid, part)?;
            Ok(hand)
        })
        .collect::<Result<Vec<_>, Report>>()?;

    // rank hands, strongest to weakest
    hands.sort();

    // result is the produce of rank and the bid
    let result = hands.iter().enumerate().map(|(i, h)| h.bid * (hands.len() - i)).sum();
    info!("Answer: {result}");
    Ok(result)
}

#[test]
fn part_1() -> Result<(), Report> {
    let expected = 248569531;
    let observed = run(&Part::Part1)?;
    assert_eq!(observed, expected);
    Ok(())
}

#[test]
fn part_2() -> Result<(), Report> {
    let expected = 250382098;
    let observed = run(&Part::Part2)?;
    assert_eq!(observed, expected);
    Ok(())
}
