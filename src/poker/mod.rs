use crate::day::Part;

use color_eyre::eyre::{eyre, Report, Result};
use core::cmp::Ordering;
use itertools::Itertools;
use log::debug;

#[derive(Debug, Eq, PartialEq)]
pub struct Hand {
    pub hand_type: HandType,
    pub cards: Vec<char>,
    pub cards_rank: Vec<usize>,
    pub bid: usize,
}

impl Hand {
    pub fn from_cards(cards: &[char], bid: usize, part: &Part) -> Result<Self, Report> {
        let cards_rank = cards
            .iter()
            .map(|c| {
                let r = match c.to_digit(10) {
                    // simple number card
                    Some(d) => d,
                    // face card points
                    None => match c {
                        'T' => 10,
                        'J' => match *part {
                            Part::Part1 => 11,
                            Part::Part2 => 1,
                        },
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
            hand_type: HandType::from_cards(cards, part)?,
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

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
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
    pub fn from_cards(cards: &[char], part: &Part) -> Result<Self, Report> {
        // count up each card in the hand
        let cards_unique = cards.iter().unique().cloned().collect_vec();
        let mut counts =
            cards_unique.iter().map(|u| cards.iter().filter(|c| *c == u).count()).collect_vec();
        counts.sort();
        counts.reverse();

        // parse card count to hand type
        let mut hand_type = match *counts {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => return Err(eyre!("Unknown hand type: {cards:?}")),
        };

        // handle "J" as jokers in Part 2
        if *part == Part::Part2 && cards.contains(&'J') {
            let num_jokers = cards.iter().filter(|c| **c == 'J').count();

            hand_type = match hand_type {
                HandType::FiveOfAKind | HandType::FourOfAKind => HandType::FiveOfAKind,
                HandType::FullHouse => match num_jokers {
                    3 | 2 => HandType::FiveOfAKind,
                    _ => HandType::FourOfAKind,
                },
                HandType::ThreeOfAKind => match num_jokers {
                    3 => HandType::FourOfAKind,
                    2 => HandType::FiveOfAKind,
                    _ => HandType::FourOfAKind,
                },
                HandType::TwoPair => match num_jokers {
                    2 => HandType::FourOfAKind,
                    _ => HandType::FullHouse,
                },
                HandType::OnePair => HandType::ThreeOfAKind,
                HandType::HighCard => HandType::OnePair,
            };
        }

        debug!("cards: {cards:?}, hand_type: {hand_type:?}");
        Ok(hand_type)
    }
}
