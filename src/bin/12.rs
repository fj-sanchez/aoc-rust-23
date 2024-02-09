// Solution based on https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2023/day12.rs

use std::str::FromStr;

use nom::{
    character::complete::{char, line_ending, one_of, space1, u32},
    combinator::opt,
    error::Error,
    multi::{many0, separated_list1},
    sequence::{separated_pair, terminated},
    Finish, IResult,
};

advent_of_code::solution!(12);

#[derive(Debug, Clone)]
struct SpringConditions {
    row: Vec<char>,
    groups: Vec<u32>,
}

fn parse_input(input: &str) -> IResult<&str, (Vec<char>, Vec<u32>)> {
    let (i, (row, groups)) = terminated(
        separated_pair(
            many0(one_of("#.?")),
            space1,
            separated_list1(char(','), u32),
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

fn count_solutions(springs: &[SpringConditions], repeat: usize) -> u64 {
    let mut count = 0;
    let mut pattern: Vec<char> = Vec::new();
    let mut groups: Vec<u32> = Vec::new();
    // Exact size is not too important as long as there's enough space.
    let mut broken = vec![0; 500];
    let mut table = vec![0; 500 * 50];

    for spring in springs.iter() {
        pattern.clear();
        groups.clear();

        for _ in 1..repeat {
            pattern.extend(&spring.row);
            pattern.push('?');
            groups.extend(&spring.groups);
        }

        pattern.extend(&spring.row);
        pattern.push('.');
        groups.extend(&spring.groups);

        let mut sum = 0;
        broken.push(0);

        for (i, &b) in pattern.iter().enumerate() {
            if b != '.' {
                sum += 1;
            }
            broken[i + 1] = sum;
        }

        let wiggle = pattern.len() - groups.iter().sum::<u32>() as usize - groups.len() + 1;

        // Count combinations, handling the first row as a special case.
        let size = groups[0] as usize;
        let mut sum: u64 = 0;
        let mut valid = true;

        for i in 0..wiggle {
            if pattern[i + size] == '#' {
                sum = 0;
            } else if valid && broken[i + size] - broken[i] == size {
                sum += 1;
            }

            table[i + size] = sum;
            valid &= pattern[i] != '#';
        }

        let mut start = size + 1;

        for (row, size) in groups.iter().map(|&n| n as usize).enumerate().skip(1) {
            let prev = (row - 1) * pattern.len();
            let cur = row * pattern.len();

            sum = 0;

            for i in start..start + wiggle {
                if pattern[i + size] == '#' {
                    sum = 0;
                } else if table[prev + i - 1] > 0
                    && pattern[i - 1] != '#'
                    && broken[i + size] - broken[i] == size
                {
                    sum += table[prev + i - 1];
                }

                table[cur + i + size] = sum;
            }
            start += size + 1;
        }
        count += sum;
    }

    count
}

pub fn part_one(input: &str) -> Option<u64> {
    let spring_conditions: Vec<SpringConditions> = input
        .lines()
        .map(|line| SpringConditions::from_str(line).unwrap())
        .collect();

    Some(count_solutions(&spring_conditions, 1))
}

pub fn part_two(input: &str) -> Option<u64> {
    let spring_conditions: Vec<SpringConditions> = input
        .lines()
        .map(|line| SpringConditions::from_str(line).unwrap())
        .collect();

    Some(count_solutions(&spring_conditions, 5))
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
        assert_eq!(result, Some(525152));
    }
}
