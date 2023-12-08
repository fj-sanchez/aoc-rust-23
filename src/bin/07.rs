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

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '2' => Ok(Card::_2),
            '3' => Ok(Card::_3),
            '4' => Ok(Card::_4),
            '5' => Ok(Card::_5),
            '6' => Ok(Card::_6),
            '7' => Ok(Card::_7),
            '8' => Ok(Card::_8),
            '9' => Ok(Card::_9),
            'T' => Ok(Card::T),
            'J' => Ok(Card::J),
            'Q' => Ok(Card::Q),
            'K' => Ok(Card::K),
            'A' => Ok(Card::A),
            _ => Err(()),
        }
    }
}

struct Hand {
    cards: [Card; 5],
    bid: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        fn get_cards_freqs(hand: &Hand) -> Vec<usize> {
            hand.cards
                .iter()
                .counts()
                .into_values()
                .sorted()
                .rev()
                .collect()
        }
        let self_freqs = get_cards_freqs(self);
        let other_freqs = get_cards_freqs(other);

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

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (i, card) = one_of("AKQJT98765432")(input)?;
    Ok((i, Card::try_from(card).unwrap()))
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
    fn cmp_with_joker(this: &Hand, other: &Hand) -> Ordering {
        fn get_hand_with_adjusted_freqs(hand: &Hand) -> Vec<usize> {
            let mut counts = hand.cards.iter().counts();
            let joker_count = counts.remove(&Card::J).unwrap_or(0);
            let mut freqs: Vec<usize> = counts.into_values().sorted().rev().collect();
            match freqs.first_mut() {
                Some(count) => {
                    *count += joker_count;
                    freqs
                }
                None => {
                    freqs.push(joker_count);
                    freqs
                }
            }
        }
        let this_freqs = get_hand_with_adjusted_freqs(this);
        let other_freqs = get_hand_with_adjusted_freqs(other);

        match Vec::cmp(&this_freqs, &other_freqs) {
            // Ordering::Equal => other.cards.cmp(&other.cards),
            Ordering::Equal => this
                .cards
                .iter()
                .zip(other.cards.iter())
                .skip_while(|(&ref a, &ref b)| a == b)
                .next()
                .map(|(&ref a, &ref b)| match (a, b) {
                    (Card::J, _) => Ordering::Less,
                    (_, Card::J) => Ordering::Greater,
                    (a, b) => Card::cmp(&a, &b),
                })
                .unwrap_or(Ordering::Equal),
            ord => ord,
        }
    }

    Some(
        input
            .lines()
            .map(|line| Hand::from_str(line).unwrap())
            .sorted_by(cmp_with_joker)
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
