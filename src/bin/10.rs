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
        .replace("-", "═")
        .replace("L", "╚")
        .replace("J", "╝")
        .replace("F", "╔")
        .replace("7", "╗")
        .replace("|", "║")
}

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
            get_next_pipes_delta(map[c.y][c.x]).iter().any(|&(dx, dy)| {
                (c.x as i32) + dx == start_coords.x as i32
                    && (c.y as i32) + dy == start_coords.y as i32
            })
        })
        .map(|n| {
            (
                (n.x as i32) - (start_coords.x as i32),
                (n.y as i32) - (start_coords.y as i32),
            )
        })
        .collect();
    *pipe_shapes
        .iter()
        .skip_while(|&&pipe| {
            !get_next_pipes_delta(pipe)
                .iter()
                .all(|delta| neighbours_deltas.iter().contains(delta))
        })
        .next()
        .unwrap()
}

fn get_neighbours_pipes_coords(coords: &Coord, map: &Vec<Vec<char>>) -> Vec<Coord> {
    let check_deltas: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    check_deltas
        .iter()
        .filter_map(|&(dx, dy)| {
            match (
                (coords.x as i32).checked_add(dx),
                (coords.y as i32).checked_add(dy),
            ) {
                (Some(x), Some(y)) if (x as usize) < map[0].len() && (y as usize) < map.len() => {
                    Some(Coord {
                        x: (x as usize),
                        y: (y as usize),
                    })
                }
                _ => return None,
            }
        })
        .filter(|c| map[c.y][c.x] != '.')
        .collect()
}

fn get_map(input: &str, start_coords: &Coord) -> Map {
    let (_, mut map) = parse_input(input).finish().unwrap();
    map[start_coords.y][start_coords.x] = get_start_pipe_type(&map, start_coords);
    map
}

fn get_start_coordinate(input: &str) -> Coord {
    let width = input.find('\n').unwrap();
    let start_coord = input
        .find("S")
        .map(|index| div_rem(index, width + 1))
        .map(|(y, x)| Coord { x, y })
        .unwrap();
    start_coord
}

fn get_pipe_loop_coordinates(start_coord: Coord, map: Vec<Vec<char>>) -> HashSet<Coord> {
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut stack: VecDeque<Coord> = VecDeque::new();
    stack.push_back(start_coord);
    while let Some(coords) = stack.pop_front() {
        if !visited.contains(&coords) {
            let deltas = get_next_pipes_delta(map[coords.y][coords.x]);
            deltas.iter().for_each(|delta| {
                stack.push_back(Coord {
                    x: ((coords.x as i32) + delta.0) as usize,
                    y: ((coords.y as i32) + delta.1) as usize,
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
    let visited = get_pipe_loop_coordinates(start_coord, map);

    Some((visited.len() / 2) as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    // let pipe_loop_coordinates = fun_name(input);
    Some(0)
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
