use crate::day::Part;
use crate::utils;

use color_eyre::eyre::{eyre, Report, Result};
use core::cmp::Ordering;
use itertools::Itertools;
//use log::debug;
use log::info;

#[derive(Debug, Eq, PartialEq)]
pub struct Hand {
    hand_type: HandType,
    cards: Vec<char>,
    cards_rank: Vec<usize>,
    bid: usize,
}

impl Hand {
    pub fn from_cards(cards: &[char], bid: usize) -> Result<Self, Report> {
        let cards_rank = cards
            .iter()
            .map(|c| {
                let r = match c.to_digit(10) {
                    // simple number card
                    Some(d) => d,
                    // face card points
                    None => match c {
                        'T' => 10,
                        'J' => 11,
                        'Q' => 12,
                        'K' => 13,
                        'A' => 14,
                        _ => return Err(eyre!("Unknown card: {c}")),
                    },
                } as usize;
                Ok(r)
            })
            .collect::<Result<Vec<_>, Report>>()?;

        let hand = Hand {
            hand_type: HandType::from_cards(cards)?,
            cards: cards.to_vec(),
            cards_rank,
            bid,
        };

        Ok(hand)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type == other.hand_type {
            return self.cards_rank.cmp(&other.cards_rank).reverse();
        }
        self.hand_type.cmp(&other.hand_type)
    }
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    pub fn from_cards(cards: &[char]) -> Result<Self, Report> {
        let mut counts =
            cards.iter().unique().map(|u| cards.iter().filter(|c| *c == u).count()).collect_vec();
        counts.sort();
        counts.reverse();

        let t = match *counts {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => return Err(eyre!("Unknown hand type: {cards:?}")),
        };

        Ok(t)
    }
}

/// Day 7 - Camel Cards
/// A modified version of poker. We need to:
/// 1. Order a 'Hand' based on the Type (ex. Full House)
///     - Order types: Five of a Kind > Four of a Kind
///     - Split String into chars: "T55J5" => ['T', '5', '5', 'J', '5']
///     - Count char occurences: [('T', 1), ('5' : 3), ('J': 1)]
///     - Sort the counts and match Type patterns:
///         [3, 1, 1] => Three Of A Kind, [3, 2] => Full House
/// 2. Order a 'Hand' based on the Power (reading left to )
pub fn run(part: &Part) -> Result<usize, Report> {
    // read in puzzle input
    let input = utils::read_to_string("data/day_7.txt")?;
    let lines = input.split('\n').map(|l| l.split(' ').collect_tuple().unwrap()).collect_vec();

    // parse cards and bids into hands
    let mut hands = lines
        .into_iter()
        .map(|(hand, bid)| {
            let cards = hand.chars().collect_vec();
            let bid = bid.parse::<usize>().unwrap();
            let hand = Hand::from_cards(&cards, bid)?;
            Ok(hand)
        })
        .collect::<Result<Vec<_>, Report>>()?;

    // Part 1: rank hands
    hands.sort();

    let result = match *part {
        Part::Part1 => hands.iter().enumerate().map(|(i, h)| h.bid * (hands.len() - i)).sum(),
        Part::Part2 => 2,
    };

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
    let expected = 2;
    let observed = run(&Part::Part2)?;
    assert_eq!(observed, expected);
    Ok(())
}
