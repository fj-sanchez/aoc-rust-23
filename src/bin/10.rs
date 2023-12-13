use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    Finish, IResult,
};
use num::integer::div_rem;

advent_of_code::solution!(10);

fn _pretty_input(input: &str) -> String {
    input
        .to_string()
        .replace('-', "═")
        .replace('L', "╚")
        .replace('J', "╝")
        .replace('F', "╔")
        .replace('7', "╗")
        .replace('|', "║")
}

#[derive(Debug, Eq, PartialEq, Hash)]
// X increases left to right
struct Coord {
    x: i32,
    y: i32,
}

type Pipe = char;
type Map = Vec<Vec<Pipe>>;

fn parse_input(input: &str) -> IResult<&str, Map> {
    let pipe = one_of("S-LJF7|.");
    let line = many1(pipe);
    let (i, map) = separated_list1(line_ending, line)(input)?;
    Ok((i, map))
}

fn get_next_pipes_delta(pipe: Pipe) -> &'static [(i32, i32); 2] {
    match pipe {
        // clockwise order starting at the top
        '-' => &[(1, 0), (-1, 0)],
        '|' => &[(0, -1), (0, 1)],
        'F' => &[(1, 0), (0, 1)],
        'L' => &[(0, -1), (1, 0)],
        'J' => &[(0, -1), (-1, 0)],
        '7' => &[(0, 1), (-1, 0)],
        _ => panic!("Unrecognised pipe character"),
    }
}

fn get_start_pipe_type(map: &Map, start_coords: &Coord) -> Pipe {
    let pipe_shapes = ['-', '|', 'F', 'L', 'J', '7'];
    let neighbours_deltas: Vec<(i32, i32)> = get_neighbours_pipes_coords(start_coords, map)
        .iter()
        .filter(|&c| {
            get_next_pipes_delta(map[c.y as usize][c.x as usize])
                .iter()
                .any(|&(dx, dy)| c.x + dx == start_coords.x && c.y + dy == start_coords.y)
        })
        .map(|n| (n.x - start_coords.x, n.y - start_coords.y))
        .collect();
    *pipe_shapes
        .iter()
        .find(|&&pipe| {
            get_next_pipes_delta(pipe)
                .iter()
                .all(|delta| neighbours_deltas.iter().contains(delta))
        })
        .unwrap()
}

fn get_neighbours_pipes_coords(coords: &Coord, map: &Vec<Vec<char>>) -> Vec<Coord> {
    let check_deltas: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    check_deltas
        .iter()
        .filter_map(|&(dx, dy)| match (coords.x + dx, coords.y + dy) {
            (x, y) if (x as usize) < map[0].len() && (y as usize) < map.len() => {
                Some(Coord { x, y })
            }
            _ => None,
        })
        .filter(|c| map[c.y as usize][c.x as usize] != '.')
        .collect()
}

fn get_map(input: &str, start_coords: &Coord) -> Map {
    let (_, mut map) = parse_input(input).finish().unwrap();
    map[start_coords.y as usize][start_coords.x as usize] = get_start_pipe_type(&map, start_coords);
    map
}

fn get_start_coordinate(input: &str) -> Coord {
    let width = input.find('\n').unwrap();

    input
        .find('S')
        .map(|index| div_rem(index as i32, (width + 1) as i32))
        .map(|(y, x)| Coord { x, y })
        .unwrap()
}

fn get_pipe_loop_coordinates(start_coord: Coord, map: &[Vec<char>]) -> HashSet<Coord> {
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut stack: VecDeque<Coord> = VecDeque::new();
    stack.push_back(start_coord);
    while let Some(coords) = stack.pop_front() {
        if !visited.contains(&coords) {
            let deltas = get_next_pipes_delta(map[coords.y as usize][coords.x as usize]);
            deltas.iter().for_each(|(dx, dy)| {
                stack.push_back(Coord {
                    x: coords.x + dx,
                    y: coords.y + dy,
                })
            });
            visited.insert(coords);
        }
    }
    visited
}

pub fn part_one(input: &str) -> Option<u32> {
    let start_coord = get_start_coordinate(input);
    let map = get_map(input, &start_coord);
    let visited = get_pipe_loop_coordinates(start_coord, &map);

    Some((visited.len() / 2) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let start_coord = get_start_coordinate(input);
    let map = get_map(input, &start_coord);
    let visited = get_pipe_loop_coordinates(start_coord, &map);

    let height = map.len() as i32;
    let width = map[0].len() as i32;

    let count: usize = (0..height)
        .flat_map(|y| (0..width).map(move |x| Coord { x, y }))
        .filter(|coord| !visited.contains(coord))
        .map(|coord| {
            let mut prev = '.';
            (coord.x + 1..width)
                .filter(|&next_right| {
                    visited.contains(&Coord {
                        x: next_right,
                        y: coord.y,
                    })
                })
                .filter(|&next_right| {
                    let pipe = map[coord.y as usize][next_right as usize];
                    match (prev, pipe) {
                        (_, '-') => false,
                        ('F', '7') | ('L', 'J') => {
                            prev = pipe;
                            true
                        }
                        ('F', 'J') | ('L', '7') => {
                            prev = pipe;
                            false
                        }
                        (_, _) => {
                            prev = pipe;
                            true
                        }
                    }
                })
                .count()
                % 2
        })
        .sum();

    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(4));
    }
}
