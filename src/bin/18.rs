use std::collections::BTreeSet;

use itertools::Itertools;
use nom::character::complete::{hex_digit1, newline, space1};
use nom::combinator::{map, opt};
use nom::multi::many1;
use nom::{
    character::complete::{char, one_of, u32},
    sequence::{delimited, preceded, terminated, tuple},
    Finish, IResult,
};
use num::Complex;
use pathfinding::grid::Grid;

advent_of_code::solution!(18);

type Direction = Complex<isize>;
const UP: Direction = Complex::<isize>::new(0, -1);
const RIGHT: Direction = Complex::<isize>::new(1, 0);
const DOWN: Direction = Complex::<isize>::new(0, 1);
const LEFT: Direction = Complex::<isize>::new(-1, 0);

struct DigMove {
    direction: Direction,
    delta: u32,
    colour: u32,
}

#[derive(Ord, Eq, PartialEq, PartialOrd)]
struct Trench<'a> {
    x: isize,
    y: isize,
    colour: &'a u32,
}

fn dig_move(input: &str) -> IResult<&str, DigMove> {
    let dir = map(one_of("URDL"), |d| match d {
        'U' => UP,
        'R' => RIGHT,
        'D' => DOWN,
        'L' => LEFT,
        _ => panic!("Unexpected move: {d}"),
    });

    let rgb_hex = map(
        delimited(char('('), preceded(char('#'), hex_digit1), char(')')),
        |h| u32::from_str_radix(h, 16).unwrap(),
    );

    let (i, (direction, delta, colour)) =
        tuple((dir, preceded(space1, u32), preceded(space1, rgb_hex)))(input)?;

    Ok((
        i,
        DigMove {
            direction,
            delta,
            colour,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<DigMove>> {
    let (i, dig_moves) = many1(terminated(dig_move, opt(newline)))(input)?;
    Ok((i, dig_moves))
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, dig_moves) = parse_input(input).finish().unwrap();

    let trenches = &mut BTreeSet::new();

    dig_moves
        .iter()
        .fold((0isize, 0isize), |(mut x, mut y), dig_move| {
            for _ in 0..dig_move.delta {
                x += dig_move.direction.re;
                y += dig_move.direction.im;
                trenches.insert(Trench {
                    x,
                    y,
                    colour: &dig_move.colour,
                });
            }

            (x, y)
        });

    let (_, _, inner_area) = trenches.iter().tuple_windows().fold(
        (true, false, 0),
        |(is_inside, is_edge, area), (a, b)| match (is_inside, is_edge, a, b) {
            // on column change
            (_, _, a, b) if a.x != b.x => (true, false, area),
            // on edge start
            (_, false, a, b) if a.y + 1 == b.y => (is_inside, true, area),
            // on edge end when transition to inside
            (false, true, a, b) if a.y + 1 != b.y => (!is_inside, false, b.y - a.y - 1 + area),
            // on edge end when transition to outside
            (true, true, a, b) if a.y + 1 != b.y => (!is_inside, false, area),
            // inside
            (true, false, a, b) => (false, false, b.y - a.y - 1 + area),
            // outside
            (false, false, _, _) => (true, is_edge, area),
            // on edge
            (_, true, _, _) => (is_inside, is_edge, area),
        },
    );

    trenches.len().checked_add_signed(inner_area)
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
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
