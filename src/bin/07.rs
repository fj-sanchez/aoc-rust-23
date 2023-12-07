use std::{cmp::Ordering, str::FromStr};

use itertools::Itertools;
use nom::{
    character::complete::u32,
    character::complete::{line_ending, one_of, space1},
    combinator::opt,
    error::Error,
    multi::count,
    sequence::{separated_pair, terminated},
    Finish, IResult,
};

advent_of_code::solution!(7);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    _2 = 0,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CardWithJoker {
    J = 0,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    Q,
    K,
    A,
}

impl TryFrom<Card> for CardWithJoker {
    type Error = ();

    fn try_from(value: Card) -> Result<Self, Self::Error> {
        match value {
            Card::_2 => Ok(CardWithJoker::_2),
            Card::_3 => Ok(CardWithJoker::_3),
            Card::_4 => Ok(CardWithJoker::_4),
            Card::_5 => Ok(CardWithJoker::_5),
            Card::_6 => Ok(CardWithJoker::_6),
            Card::_7 => Ok(CardWithJoker::_7),
            Card::_8 => Ok(CardWithJoker::_8),
            Card::_9 => Ok(CardWithJoker::_9),
            Card::T => Ok(CardWithJoker::T),
            Card::J => Ok(CardWithJoker::J),
            Card::Q => Ok(CardWithJoker::Q),
            Card::K => Ok(CardWithJoker::K),
            Card::A => Ok(CardWithJoker::A),
        }
    }
}

macro_rules! impl_TryFrom {
    (for $($t:ty),+) => {
        $(impl TryFrom<char> for $t {
            type Error = ();

            fn try_from(c: char) -> Result<Self, Self::Error> {
                match c {
                    '2' => Ok(<$t>::_2),
                    '3' => Ok(<$t>::_3),
                    '4' => Ok(<$t>::_4),
                    '5' => Ok(<$t>::_5),
                    '6' => Ok(<$t>::_6),
                    '7' => Ok(<$t>::_7),
                    '8' => Ok(<$t>::_8),
                    '9' => Ok(<$t>::_9),
                    'T' => Ok(<$t>::T),
                    'J' => Ok(<$t>::J),
                    'Q' => Ok(<$t>::Q),
                    'K' => Ok(<$t>::K),
                    'A' => Ok(<$t>::A),
                    _ => Err(()),
                }
            }
        })*
    }
}

impl_TryFrom!(for Card, CardWithJoker);

struct Hand {
    cards: [Card; 5],
    bid: u32,
}

struct HandWithJoker {
    cards: [CardWithJoker; 5],
    bid: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_freqs = self
            .cards
            .iter()
            .counts()
            .into_values()
            .sorted()
            .rev()
            .collect();
        let other_freqs = other
            .cards
            .iter()
            .counts()
            .into_values()
            .sorted()
            .rev()
            .collect();

        match Vec::cmp(&self_freqs, &other_freqs) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            ord => ord,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Hand {}

impl Ord for HandWithJoker {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut self_counts = self.cards.iter().counts();
        let self_joker_count = self_counts.remove(&CardWithJoker::J).unwrap_or(0);
        let mut self_freqs: Vec<usize> = self_counts.into_values().sorted().rev().collect();
        match self_freqs.first_mut() {
            Some(count) => *count += self_joker_count,
            None => self_freqs.push(self_joker_count),
        }

        let mut other_counts = other.cards.iter().counts();
        let other_joker_count = other_counts.remove(&CardWithJoker::J).unwrap_or(0);
        let mut other_freqs: Vec<usize> = other_counts.into_values().sorted().rev().collect();
        match other_freqs.first_mut() {
            Some(count) => *count += other_joker_count,
            None => other_freqs.push(other_joker_count),
        }

        match Vec::cmp(&self_freqs, &other_freqs) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            ord => ord,
        }
    }
}

impl PartialOrd for HandWithJoker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HandWithJoker {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for HandWithJoker {}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (i, card) = one_of("AKQJT98765432")(input)?;
    Ok((i, Card::try_from(card).unwrap()))
}

fn parse_card_with_joker(input: &str) -> IResult<&str, CardWithJoker> {
    let (i, card) = one_of("AKQJT98765432")(input)?;
    Ok((i, CardWithJoker::try_from(card).unwrap()))
}

fn parse_bid(input: &str) -> IResult<&str, u32> {
    let (i, bid) = terminated(u32, opt(line_ending))(input)?;
    Ok((i, bid))
}

fn parse_hand(input: &str) -> IResult<&str, (Vec<Card>, u32)> {
    let hand = count(parse_card, 5);
    let (i, (cards, bid)) = separated_pair(hand, space1, parse_bid)(input)?;

    Ok((i, (cards, bid)))
}

fn parse_hand_with_joker(input: &str) -> IResult<&str, (Vec<CardWithJoker>, u32)> {
    let hand = count(parse_card_with_joker, 5);
    let (i, (cards, bid)) = separated_pair(hand, space1, parse_bid)(input)?;

    Ok((i, (cards, bid)))
}

impl FromStr for Hand {
    type Err = Error<String>;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        match parse_hand(line).finish() {
            Ok((_, (cards, bid))) => Ok(Hand {
                cards: cards.try_into().unwrap(),
                bid,
            }),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

impl FromStr for HandWithJoker {
    type Err = Error<String>;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        match parse_hand_with_joker(line).finish() {
            Ok((_, (cards, bid))) => Ok(HandWithJoker {
                cards: cards.try_into().unwrap(),
                bid,
            }),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| Hand::from_str(line).unwrap())
            .sorted()
            .enumerate()
            .map(|(rank, hand)| (rank + 1) as u32 * hand.bid)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| HandWithJoker::from_str(line).unwrap())
            .sorted()
            .enumerate()
            .map(|(rank, hand)| (rank + 1) as u32 * hand.bid)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
