advent_of_code::solution!(4);

// Solution to part 1 courtesy of Marvin Jones

use nom::{
    bytes::complete::tag,
    character::complete::{char, space0, space1, u32},
    error::Error,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
    Finish, IResult,
};
use std::str::FromStr;
use std::{
    cmp,
    collections::{HashSet, VecDeque},
};

#[derive(Debug, PartialEq)]
struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn winning_number_count(&self) -> usize {
        self.numbers
            .iter()
            .filter(|&number| self.winning_numbers.contains(number))
            .count()
    }

    fn score(&self) -> u32 {
        let base: u32 = 2;
        match self.winning_number_count() {
            0 => 0,
            count => base.pow(count as u32 - 1),
        }
    }
}

fn parse_id(input: &str) -> IResult<&str, u32> {
    let (i, id) = preceded(tag("Card"), preceded(space1, u32))(input)?;
    Ok((i, id))
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    let (i, numbers) = delimited(space0, separated_list1(space1, u32), space0)(input)?;
    Ok((i, numbers))
}

fn parse_fields_of_card(input: &str) -> IResult<&str, (u32, Vec<u32>, Vec<u32>)> {
    let (i, (id, (winning_numbers, numbers))) = separated_pair(
        parse_id,
        char(':'),
        separated_pair(parse_numbers, char('|'), parse_numbers),
    )(input)?;
    Ok((i, (id, winning_numbers, numbers)))
}

impl FromStr for Card {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_fields_of_card(s).finish() {
            Ok((_remaining, (id, winning_numbers, numbers))) => Ok(Card {
                id,
                winning_numbers: HashSet::from_iter(winning_numbers),
                numbers,
            }),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let total = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| Card::from_str(line).unwrap())
        .map(|card| card.score())
        .sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (total_cards, _): (usize, VecDeque<usize>) = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| Card::from_str(line).unwrap())
        .map(|card| card.winning_number_count())
        .fold(
            (0, VecDeque::new()),
            |(mut total, mut acc): (usize, VecDeque<usize>), num_wins| {
                let extra_cards = acc.pop_front().unwrap_or(0);
                total += extra_cards + 1;
                acc.resize(cmp::max(acc.len(), num_wins), 0);
                let _ = acc.iter_mut().take(num_wins).map(|v| *v += extra_cards + 1);
                (total, acc)
            },
        );

    Some(total_cards as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
