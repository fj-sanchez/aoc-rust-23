use itertools::Itertools;

use nom::{
    character::complete::{i64, line_ending, space1},
    combinator::opt,
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};
advent_of_code::solution!(9);

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    let single_sequence = terminated(separated_list1(space1, i64), opt(line_ending));
    let (i, sequences) = many1(single_sequence)(input)?;
    Ok((i, sequences))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, sequences) = parse_input(input).unwrap();

    Some(
        sequences
            .iter()
            .map(|seq| {
                let mut stack: Vec<i64> = Vec::new();
                let mut acc: Vec<i64> = seq.clone();

                while acc.iter().any(|&v| v != 0) {
                    stack.push(*acc.last().unwrap());
                    acc = acc.iter().tuple_windows().map(|(a, b)| b - a).collect();
                }
                stack.iter().sum::<i64>()
            })
            .sum::<i64>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, sequences) = parse_input(input).unwrap();

    Some(
        sequences
            .iter()
            .map(|seq| {
                let mut stack: Vec<i64> = Vec::new();
                let mut acc: Vec<i64> = seq.clone();

                while acc.iter().any(|&v| v != 0) {
                    stack.push(*acc.first().unwrap());
                    acc = acc.iter().tuple_windows().map(|(a, b)| b - a).collect();
                }
                stack.iter().rev().fold(0, |acc, v| v - acc)
            })
            .sum::<i64>() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
