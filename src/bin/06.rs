use std::{cmp::Ordering, ops::Range};

use nom::{
    bytes::complete::tag,
    character::complete::space0,
    character::complete::{line_ending, u64},
    combinator::opt,
    error::Error,
    multi::many0,
    sequence::{pair, preceded, terminated},
    Finish, IResult,
};

advent_of_code::solution!(6);

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn pushtimes(&self) -> Range<i64> {
        let a: f64 = 1.;
        let b: f64 = -(self.time as f64);
        let c: f64 = self.distance as f64;

        // discriminant
        let d = b.powi(2) - 4. * a * c;

        match (d as i64).cmp(&0) {
            Ordering::Less => 0..0,
            Ordering::Equal => {
                let x: i64 = (-b / (2. * a)) as i64;
                x..x + 1
            }
            Ordering::Greater => {
                let sqrt = f64::sqrt(d as f64);

                let x1: i64 = ((-b - sqrt) / (2. * a) + 10. * f64::EPSILON).ceil() as i64;
                let x2: i64 = ((-b + sqrt) / (2. * a) - 10. * f64::EPSILON).floor() as i64;
                x1..x2 + 1
            }
        }
    }
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u64>> {
    let number = preceded(space0, u64);
    let (i, numbers) = terminated(many0(number), opt(line_ending))(input)?;
    Ok((i, numbers))
}

fn parse_times(input: &str) -> IResult<&str, Vec<u64>> {
    let header = tag("Time:");
    let (i, times) = preceded(header, parse_numbers)(input)?;
    Ok((i, times))
}

fn parse_distances(input: &str) -> IResult<&str, Vec<u64>> {
    let header = tag("Distance:");
    let (i, distances) = preceded(header, parse_numbers)(input)?;
    Ok((i, distances))
}

fn parse_document(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    let (i, (times, distances)) = pair(parse_times, parse_distances)(input)?;
    Ok((i, (times, distances)))
}
type Err = Error<String>;

fn parse_input(input: &str) -> Result<Vec<Race>, Err> {
    match parse_document(input).finish() {
        Ok((_, (times, distances))) => Ok(times
            .into_iter()
            .zip(distances.into_iter())
            .map(|(t, d)| Race {
                time: t,
                distance: d,
            })
            .collect()),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let races = parse_input(input).ok()?;

    Some(
        races
            .iter()
            .map(|race| {
                let r: Range<i64> = race.pushtimes();
                let p: u32 = r.count() as u32;
                p
            })
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let sanitized_input = input.replace(" ", "");
    part_one(&sanitized_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
