use nom::character::complete::{hex_digit1, newline, space1};
use nom::combinator::{map, opt};
use nom::multi::many1;
use nom::{
    character::complete::{char, one_of, u32},
    sequence::{delimited, preceded, terminated, tuple},
    Finish, IResult,
};
use num::Complex;

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

fn get_inner_area(dig_moves: &[DigMove]) -> isize {
    let (area, _) =
        dig_moves
            .iter()
            .rev()
            .fold((0, Complex::new(0, 0)), |(inner, prev), dig_move| {
                let end_coords = prev - dig_move.direction.scale(dig_move.delta as isize);
                let tmp = (prev.re * end_coords.im) - (end_coords.re * prev.im);
                (inner + tmp, end_coords)
            });
    area.abs() / 2
}

pub fn part_one(input: &str) -> Option<isize> {
    let (_, dig_moves) = parse_input(input).finish().unwrap();

    let inner_area = get_inner_area(&dig_moves);
    let perimeter: isize = dig_moves.iter().map(|t| t.delta).sum::<u32>() as isize;
    let total_area = inner_area + perimeter / 2 + 1;

    Some(total_area)
}

pub fn part_two(input: &str) -> Option<isize> {
    let (_, dig_moves) = &mut parse_input(input).finish().unwrap();

    dig_moves.iter_mut().for_each(|trench| {
        trench.delta = trench.colour >> 4;
        trench.direction = match trench.colour & 0b1111 {
            0 => RIGHT,
            1 => DOWN,
            2 => LEFT,
            3 => UP,
            _ => panic!(
                "Invalid direction decoded from colour: colour={:#08X}",
                trench.colour
            ),
        }
    });

    let inner_area = get_inner_area(dig_moves);
    let perimeter: isize = dig_moves.iter().map(|t| t.delta).sum::<u32>() as isize;
    let total_area = inner_area + perimeter / 2 + 1;

    Some(total_area)
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
        assert_eq!(result, Some(952408144115));
    }
}
