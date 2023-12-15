use std::str::FromStr;

use nom::{
    character::complete::{char, i32, line_ending, one_of, space1},
    error::Error,
    multi::{many0, separated_list1},
    sequence::{separated_pair, terminated},
    Finish, IResult, combinator::opt,
};

advent_of_code::solution!(12);

#[derive(Debug)]
struct SpringConditions {
    row: Vec<char>,
    groups: Vec<i32>,
}

fn parse_input(input: &str) -> IResult<&str, (Vec<char>, Vec<i32>)> {
    let (i, (row, groups)) = terminated(
        separated_pair(
            many0(one_of("#.?")),
            space1,
            separated_list1(char(','), i32),
        ),
        opt(line_ending),
    )(input)?;
    Ok((i, (row, groups)))
}

impl FromStr for SpringConditions {
    type Err = Error<String>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match parse_input(input).finish() {
            Ok((_, (row, groups))) => Ok(SpringConditions { row, groups }),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let spring_conditions: Vec<SpringConditions> = input
        .lines()
        .map(|line| SpringConditions::from_str(line).unwrap())
        .collect();

    Some(0)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
