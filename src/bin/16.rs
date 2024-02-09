// use cached::proc_macro::cached;
use num::complex::Complex;
use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(16);

type Direction = Complex<i32>;
const UP: Direction = Complex::<i32>::new(0, -1);
const RIGHT: Direction = Complex::<i32>::new(1, 0);
const DOWN: Direction = Complex::<i32>::new(0, 1);
const LEFT: Direction = Complex::<i32>::new(-1, 0);

type Position = Complex<i32>;
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Node {
    position: Position,
    direction: Direction,
}

// #[cached] this actually slows it down
fn exits(direction: Direction, cell: char) -> Vec<Direction> {
    match cell {
        '-' if direction == UP || direction == DOWN => vec![LEFT, RIGHT],
        '|' if direction == LEFT || direction == RIGHT => vec![UP, DOWN],
        '/' => vec![-Complex::new(direction.im, direction.re)],
        '\\' => vec![Complex::new(direction.im, direction.re)],
        _ => vec![direction],
    }
}

type MapData = HashMap<Complex<i32>, char>;
fn parse_input(input: &str) -> MapData {
    let mut map_data: HashMap<Complex<i32>, char> = HashMap::new();

    input.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, c)| {
            map_data.insert(Complex::new(col as i32, row as i32), c);
        })
    });

    map_data
}

pub fn part_one(input: &str) -> Option<u32> {
    let map_data: MapData = parse_input(input);

    let current_pos = Node {
        position: Complex::new(0, 0),
        direction: RIGHT,
    };
    count_energized_cells(current_pos, &map_data)
}

fn count_energized_cells(current_pos: Node, map_data: &HashMap<Complex<i32>, char>) -> Option<u32> {
    let mut visited: HashSet<Node> = HashSet::new();
    let mut boundary: VecDeque<Node> = VecDeque::new();
    boundary.push_back(current_pos);

    while let Some(node) = boundary.pop_front() {
        visited.insert(node);
        exits(node.direction, map_data[&node.position])
            .iter()
            .for_each(|&exit| {
                let next_node = Node {
                    position: node.position + exit,
                    direction: exit,
                };
                if map_data.contains_key(&next_node.position) && !visited.contains(&next_node) {
                    boundary.push_back(next_node);
                }
            })
    }

    Some(visited.iter().unique_by(|n| n.position).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map_data: MapData = parse_input(input);

    let &bottom_right = map_data.keys().max_by_key(|pos| pos.re + pos.im).unwrap();
    let top_nodes = (0..bottom_right.re + 1).map(|x| Node {
        position: Complex::new(x, 0),
        direction: DOWN,
    });
    let bottom_nodes = (0..bottom_right.re + 1).map(|x| Node {
        position: Complex::new(x, bottom_right.im),
        direction: UP,
    });
    let left_nodes = (0..bottom_right.im).map(|y| Node {
        position: Complex::new(0, y),
        direction: RIGHT,
    });
    let right_nodes = (0..bottom_right.im).map(|y| Node {
        position: Complex::new(bottom_right.im, y),
        direction: LEFT,
    });

    top_nodes
        .chain(right_nodes)
        .chain(bottom_nodes)
        .chain(left_nodes)
        .map(|node| count_energized_cells(node, &map_data).unwrap())
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
