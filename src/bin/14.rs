use std::{
    collections::VecDeque,
    hash::{DefaultHasher, Hash, Hasher},
};

use indexmap::IndexMap;

use pathfinding::matrix::{directions, Matrix};

advent_of_code::solution!(14);

type Direction = (isize, isize);
type Position = (usize, usize);

fn tilt(platform: &mut Matrix<char>, dir: Direction) {
    if dir != directions::N {
        panic!("Only north expected.")
    }

    let mut deque = vec![VecDeque::<Position>::new(); platform.columns];
    for (coords, item) in platform.clone().items() {
        let (_, col) = &coords;

        match item {
            '.' => deque[*col].push_back(coords),
            'O' => {
                if let Some(first_empty) = deque[*col].pop_front() {
                    platform.swap(first_empty, coords);
                    deque[*col].push_back(coords);
                }
            }
            '#' => deque[*col].clear(),
            _ => panic!(
                "Unknown element found in the platform: coords={:?} value={}",
                coords, item
            ),
        }
    }
}

fn total_load(platform: &Matrix<char>) -> usize {
    platform
        .items()
        .filter_map(|((row, _), item)| item.eq(&'O').then_some(platform.rows - row))
        .sum()
}

fn _print_platform(platform: &Matrix<char>) {
    for row in platform.iter() {
        println!("{:?}", row.iter().collect::<String>());
    }
    println!();
}

pub fn part_one(input: &str) -> Option<usize> {
    let platform = &mut Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap();
    tilt(platform, directions::N);

    Some(total_load(platform))
}

pub fn part_two(input: &str) -> Option<usize> {
    let repeat = 1000000000;
    let platform = &mut Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap();
    let mut seen = IndexMap::<u64, usize>::new();

    for i in 0..repeat {
        for _ in 0..4 {
            tilt(platform, directions::N);
            platform.rotate_cw(1);
        }

        let mut hasher = DefaultHasher::new();
        platform.hash(&mut hasher);
        let key = hasher.finish();
        let val = total_load(platform);

        let (index, value) = seen.insert_full(key, val);
        if value.is_some() {
            let solution_index = index + (repeat - 1 - i) % (i - index);
            let (_, load) = seen.get_index(solution_index).unwrap();
            return Some(*load);
        }
    }

    Some(total_load(platform))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
