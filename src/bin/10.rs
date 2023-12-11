use std::collections::{vec_deque, HashSet, VecDeque};

use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    Finish, IResult,
};
use num::integer::div_rem;

advent_of_code::solution!(10);

#[derive(Debug, Eq, PartialEq, Hash)]
// X increases left to right
struct Coord {
    x: usize,
    y: usize,
}

type Pipe = char;
type Map = Vec<Vec<Pipe>>;

fn parse_input(input: &str) -> IResult<&str, Map> {
    let pipe = one_of("S-LJF7|.");
    let line = many1(pipe);
    let (i, map) = separated_list1(line_ending, line)(input)?;
    Ok((i, map))
}

fn pretty_input(input: &str) -> String {
    input
        .to_string()
        .replace("-", "═")
        .replace("L", "╚")
        .replace("J", "╝")
        .replace("F", "╔")
        .replace("7", "╗")
        .replace("|", "║")
}

type CoordDelta = (i32, i32);

fn next_pipes_delta(pipe: Pipe) -> &'static [(i32, i32)] {
    match pipe {
        '-' => &[(-1, 0), (1, 0)],
        '|' => &[(0, -1), (0, 1)],
        'F' => &[(0, 1), (1, 0)],
        'L' => &[(0, -1), (1, 0)],
        'J' => &[(0, -1), (-1, 0)],
        '7' => &[(0, 1), (-1, 0)],
        _ => panic!("Unrecognised pipe character"),
    }
}

fn get_start() -> char {
    // TODO implement so it can generalise
    // '╝'
    'F' // for test
        // 'J'  // for input data
}

pub fn part_one(input: &str) -> Option<u32> {
    // print!("{}", pretty_input(input));

    let width = input.find('\n').unwrap();
    let height = input.lines().count();
    let start_coord = input
        .find("S")
        .map(|index| div_rem(index, width + 1))
        .map(|(y, x)| Coord { x, y })
        .unwrap();

    let (_, mut map) = parse_input(input).finish().unwrap();
    map[start_coord.y][start_coord.x] = get_start();

    let mut stack: VecDeque<&Coord> = VecDeque::new();
    let mut visited: HashSet<Coord> = HashSet::new();

    stack.push_back(&start_coord);
    while let Some(coords) = stack.pop_front() {
        if visited.insert(coords) {
            let _ = next_pipes_delta(map[coords.y][coords.x])
                .iter()
                .map(| delta| Coord {
                    x: ((coords.x as i32) + delta.0) as usize,
                    y: ((coords.y as i32) + delta.1) as usize,
                });
        }
    }

    Some((visited.len() / 2) as u32)
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
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
