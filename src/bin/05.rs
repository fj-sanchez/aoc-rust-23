use std::{ops::Range, str::FromStr};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space0, u64},
    combinator::opt,
    error::Error,
    multi::many0,
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

    fn destination_ranges(&self, range: &Range<u64>) -> (Vec<Range<u64>>, Vec<Range<u64>>) {
        let mut ranges_to_map: Vec<Range<u64>> = Vec::new();
        let mut remaining: Vec<Range<u64>> = Vec::new();

        // 1. no-overlap or complete overlap, just map the input range
        // 2. partial overlap on the left, return 2 ranges non-overlapping and overlapping
        // 3. partial overlap on the right, return 2 ranges overlapping and non-overlapping
        // 4. this fully contained in the input range, return 3 ranges, 2 non-overlapping and the overlapping
        if range.end <= self.source.start || range.start >= self.source.end {
            remaining.push(range.clone());
        } else if self.source.contains(&range.start) && self.source.contains(&(range.end - 1)) {
            ranges_to_map.push(range.start..range.end);
        } else if self.source.contains(&(range.end)) {
            remaining.push(range.start..self.source.start);
            ranges_to_map.push(self.source.start..range.end);
        } else if self.source.contains(&range.start) {
            ranges_to_map.push(range.start..self.source.end);
            remaining.push(self.source.end..range.end);
        } else {
            remaining.extend([range.start..self.source.start, self.source.end..range.end]);
            ranges_to_map.push(self.source.clone());
        }

        let mapped_ranges = ranges_to_map
            .into_iter()
            .map(|r| {
                self.destination_value(r.start).unwrap_or(r.start)
                    ..self.destination_value(r.end - 1).unwrap_or(r.end - 1) + 1
            })
            .collect();

        (remaining, mapped_ranges)
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

fn parse_inputs(input: &str) -> (Vec<u64>, Vec<Vec<RangeMapping>>) {
    let (_, seeds) = parse_seeds(input).unwrap();
    let mapping_blocks: Vec<Vec<RangeMapping>> = input
        .split_terminator("\n\n")
        .skip(1)
        .map(|mapping_block| {
            mapping_block
                .lines()
                .skip(1)
                .map(|l| RangeMapping::from_str(l).ok().unwrap())
                .collect_vec()
        })
        .collect_vec();
    (seeds, mapping_blocks)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (seeds, mapping_blocks) = parse_inputs(input);

    let results: Vec<u64> = seeds
        .iter()
        .map(|&seed| {
            let result = mapping_blocks.iter().fold(seed, |acc, mapping_block| {
                mapping_block
                    .iter()
                    .find_map(|r| r.destination_value(acc))
                    .unwrap_or(acc)
            });
            result
        })
        .collect_vec();

    Some(results.into_iter().min().unwrap() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (seeds, mapping_blocks) = parse_inputs(input);
    let seed_ranges: Vec<Range<u64>> = seeds
        .iter()
        .tuples::<(_, _)>()
        .map(|(&start, &length)| start..start + length)
        .collect();

    let results: Vec<Range<u64>> =
        mapping_blocks
            .iter()
            .fold(seed_ranges.clone(), |acc, mapping_block| {
                let mut mapped: Vec<Range<u64>> = Vec::new();
                let mut pending_mapping = acc.clone();
                for rm in mapping_block {
                    let mut unmapped: Vec<Range<u64>> = Vec::new();
                    for r in &pending_mapping {
                        let (remaining, destination_ranges) = rm.destination_ranges(r);
                        mapped.extend(destination_ranges);
                        unmapped.extend(remaining);
                    }
                    pending_mapping.clone_from(&unmapped);
                }
                pending_mapping.append(&mut mapped);
                pending_mapping
            });

    let x: Vec<u64> = results.iter().map(|r| r.start).sorted().collect();
    Some(*x.iter().min().unwrap() as u32)
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

    #[test]
    fn test_range_decomposition() {
        let rm = RangeMapping {
            destination: 105..115,
            source: 5..15,
        };
        let v = rm.destination_ranges(&(0..5));
        assert_eq!(v.0[0], (0..5));
        assert!(v.1.is_empty());

        let v = rm.destination_ranges(&(0..7));
        assert_eq!(v.0[0], (0..5));
        assert_eq!(v.1[0], (105..107));

        let v = rm.destination_ranges(&(5..15));
        assert!(v.0.is_empty());
        assert_eq!(v.1[0], (105..115));

        let v = rm.destination_ranges(&(5..16));
        assert_eq!(v.0[0], (15..16));
        assert_eq!(v.1[0], (105..115));

        let v = rm.destination_ranges(&(15..16));
        assert_eq!(v.0[0], (15..16));
        assert!(v.1.is_empty());
    }
}
