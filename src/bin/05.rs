use std::{
    ops::Range,
    str::FromStr,
};

use itertools::Itertools;
use nom::{
    bytes::complete::{tag},
    character::complete::{line_ending, space0, u64},
    combinator::opt,
    error::Error,
    multi::{many0 },
    sequence::{preceded, terminated, tuple},
    Finish, IResult,
};

advent_of_code::solution!(5);

// Custom type to represent a range mapping
#[derive(Debug)]
struct RangeMapping {
    destination: Range<u64>,
    source: Range<u64>,
}

impl RangeMapping {
    fn destination_value(&self, value: u64) -> Option<u64> {
        if self.source.contains(&value) {
            // Map the value if it falls within the source range
            let relative_position = value - self.source.start;
            Some(self.destination.start + relative_position)
        } else {
            None
        }
    }
}

fn number(input: &str) -> IResult<&str, u64> {
    let (i, number) = preceded(space0, u64)(input)?;
    Ok((i, number))
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    let seeds_prefix = tag("seeds:");
    let numbers = many0(number);

    let (i, seeds) = preceded(seeds_prefix, terminated(numbers, line_ending))(input)?;
    Ok((i, seeds))
}

fn parse_mapping(input: &str) -> IResult<&str, (u64, u64, u64)> {
    let (i, mapping) = terminated(tuple((number, number, number)), opt(line_ending))(input)?;
    Ok((i, mapping))
}

impl FromStr for RangeMapping {
    type Err = Error<String>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match parse_mapping(input).finish() {
            Ok((_, (destination, source, count))) => Ok(RangeMapping {
                destination: destination..destination + count,
                source: source..source + count,
            }),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, seeds) = parse_seeds(input).ok()?;

    let mapping_blocks: Vec<Vec<RangeMapping>> = input
        .split_terminator("\n\n")
        .skip(1)
        .map(|mapping_block| {
            mapping_block
                .lines()
                .skip(1)
                .map(|l| RangeMapping::from_str(&l).ok().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let results: Vec<u64> = seeds
        .iter()
        .map(|&seed| {
            let result = mapping_blocks.iter().fold(seed, |acc, mapping_block| {
                mapping_block
                    .iter()
                    .find_map(|r| r.destination_value(acc))
                    .unwrap_or_else(|| acc)
            });
            result
        })
        .collect_vec();

    Some(results.into_iter().min().unwrap() as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
