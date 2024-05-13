use std::collections::{BTreeSet, VecDeque};

use pathfinding::matrix::{
    directions::{DIRECTIONS_4, E, N, S, W},
    Matrix,
};

advent_of_code::solution!(23);

type Coord = (usize, usize);

#[derive(Debug, Default, Clone)]
struct Search {
    head: Coord,
    seen: BTreeSet<Coord>,
}

fn parse_input(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.lines().map(|line| line.chars()).collect::<Vec<_>>()).unwrap()
}

fn find_start_end(map: &Matrix<char>) -> ((usize, usize), (usize, usize)) {
    let (start, _) = map.items().find(|(.., &kind)| kind == '.').unwrap();
    let (end, _) = map
        .items()
        .filter(|(.., &kind)| kind == '.')
        .last()
        .unwrap();
    (start, end)
}

type ValidMovesFn = fn((usize, usize), &Matrix<char>) -> Vec<(usize, usize)>;

fn valid_moves_with_slopes(coord: Coord, map: &Matrix<char>) -> Vec<Coord> {
    const DIRECTION_OPPOSITE: [char; 4] = ['<', '^', '>', 'v'];
    let mut next_moves = Vec::<Coord>::default();
    match map.get(coord) {
        Some(&'^') => next_moves.push(map.move_in_direction(coord, N).unwrap()),
        Some(&'>') => next_moves.push(map.move_in_direction(coord, E).unwrap()),
        Some(&'v') => next_moves.push(map.move_in_direction(coord, S).unwrap()),
        Some(&'<') => next_moves.push(map.move_in_direction(coord, W).unwrap()),
        Some(&'.') => {
            for (ix, &dir) in DIRECTIONS_4.iter().enumerate() {
                if let Some(move_) = map.move_in_direction(coord, dir).filter(|&x| {
                    let kind = map.get(x).unwrap_or(&'#');
                    kind != &DIRECTION_OPPOSITE[ix] && kind != &'#'
                }) {
                    next_moves.push(move_);
                }
            }
        }
        Some(&c) => panic!("Unexpected value for coord: {:?}={}", coord, c),
        None => panic!("No value at coord {:?}", coord),
    };
    next_moves
}


fn find_lengths(
    start: (usize, usize),
    valid_moves_fn: ValidMovesFn,
    map: Matrix<char>,
    end: (usize, usize),
) -> Vec<usize> {
    let mut searches = vec![Search {
        head: start,
        seen: BTreeSet::from_iter([start]),
    }];
    let mut active_searches = VecDeque::<usize>::new();
    active_searches.push_back(0);

    while let Some(active_search_index) = active_searches.pop_front() {
        let active_search = searches.get_mut(active_search_index).unwrap();
        let next_moves = valid_moves_fn(active_search.head, &map)
            .iter()
            .filter(|coord| !active_search.seen.contains(coord))
            .copied()
            .collect::<Vec<_>>();

        if let Some(&next_move) = next_moves.first() {
            active_search.head = next_move;
            active_search.seen.insert(next_move);
            active_searches.push_back(active_search_index);

            // if more than 1 move found, then fork search
            for &next_move in next_moves.iter().skip(1) {
                let mut new_active_search = searches[active_search_index].clone();
                new_active_search.head = next_move;
                new_active_search.seen.remove(next_moves.first().unwrap());
                new_active_search.seen.insert(next_move);
                searches.push(new_active_search);
                active_searches.push_back(searches.len() - 1);
            }
        }
    }

    searches
        .iter()
        .filter(|search| search.head == end)
        .map(|search| search.seen.len() - 1)
        .collect()
}

fn valid_moves_without_slopes(coord: Coord, map: &Matrix<char>) -> Vec<Coord> {
    map.neighbours(coord, false)
        .filter(|&c| map.get(c).unwrap() != &'#')
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse_input(input);
    let (start, end) = find_start_end(&map);

    let valid_moves_fn = valid_moves_with_slopes;
    let lengths = find_lengths(start, valid_moves_fn, map, end);

    lengths.iter().max().copied()
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = parse_input(input);
    let (start, end) = find_start_end(&map);

    let valid_moves_fn = valid_moves_without_slopes;
    let lengths = find_lengths(start, valid_moves_fn, map, end);

    lengths.iter().max().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
