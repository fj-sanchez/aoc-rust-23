use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    vec,
};

use pathfinding::matrix::{
    directions::{DIRECTIONS_4, E, N, S, W},
    Matrix,
};

advent_of_code::solution!(23);

type Coord = (usize, usize);
type ValidMovesFn = fn(Coord, &Matrix<char>) -> Vec<Coord>;
type Edge = Vec<(Coord, usize)>;
type Graph = BTreeMap<Coord, Edge>;

#[derive(Debug, Default, Clone, Copy)]
struct Search {
    head: Coord,
    seen: u64,
    cost: usize,
}

impl Search {
    fn seen_contains(self, node_index: usize) -> bool {
        (self.seen & (1 << node_index)) > 0
    }

    fn seen_insert(&mut self, node_index: usize) {
        self.seen |= 1 << node_index;
    }

    fn seen_remove(&mut self, node_index: usize) {
        self.seen &= !(1 << node_index);
    }
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

fn valid_moves_without_slopes(coord: Coord, map: &Matrix<char>) -> Vec<Coord> {
    map.neighbours(coord, false)
        .filter(|&c| map.get(c).unwrap() != &'#')
        .collect()
}

fn is_junction(coord: Coord, map: &Matrix<char>) -> bool {
    valid_moves_without_slopes(coord, map).len() > 2
}

fn create_graph_from_map(
    start: Coord,
    end: Coord,
    map: &Matrix<char>,
    valid_moves_fn: ValidMovesFn,
) -> Graph {
    // create a node per junction in the graph and initialise their edges
    let mut graph: Graph = map
        .items()
        .filter(|&(.., kind)| kind != &'#')
        .filter(|&(coord, ..)| is_junction(coord, map))
        .map(|(c, ..)| (c, vec![]))
        .collect();
    graph.insert(start, vec![]);
    graph.insert(end, vec![]);

    let mut seen = BTreeSet::<Coord>::new();
    let mut searches = VecDeque::<(Coord, Coord)>::new();
    searches.push_back((start, map.move_in_direction(start, S).unwrap()));
    seen.insert(start);

    while let Some((junction, junction_exit)) = searches.pop_front() {
        let mut section_length = 1;
        let mut next_in_search = junction_exit;
        loop {
            let next_moves = valid_moves_fn(next_in_search, map)
                .iter()
                .filter(|&coord| coord != &junction)
                .filter(|&coord| !seen.contains(coord) || graph.contains_key(coord))
                .copied()
                .collect::<Vec<_>>();

            // if next_in_search is junction, create edge and enqueue searches from exits
            if graph.contains_key(&next_in_search) {
                // println!(
                //     "From {:?} to {:?} with cost {}",
                //     &junction, &next_in_search, section_length
                // );
                graph
                    .get_mut(&junction)
                    .unwrap()
                    .push((next_in_search, section_length));
                for next_move in next_moves {
                    if !seen.contains(&next_move) {
                        searches.push_front((next_in_search, next_move));
                    }
                }
                break;
            } else if !next_moves.is_empty() {
                assert!(
                    next_moves.len() == 1,
                    "next_in_search: {:?}, next_moves: {:?}",
                    next_in_search,
                    next_moves
                );

                section_length += 1;
                seen.insert(next_in_search);
                next_in_search = *next_moves.first().unwrap();
            } else {
                break;
            }
        }
    }
    graph
}

fn find_lengths_in_graph(start: (usize, usize), end: (usize, usize), graph: Graph) -> Vec<usize> {
    let node_index_lookup = graph
        .keys()
        .enumerate()
        .map(|(idx, coord)| (coord, idx))
        .collect::<BTreeMap<&Coord, usize>>();
    let mut searches = vec![Search {
        head: start,
        seen: 1,
        cost: 0,
    }];
    let mut active_searches = VecDeque::<usize>::new();
    active_searches.push_back(0);

    // FIXME: cache attempt doesn't work, wrong key
    // let mut cache = BTreeMap::<(usize, u64), usize>::new();
    // cache.insert((node_index_lookup[&start], 1), 0);

    while let Some(active_search_index) = active_searches.pop_front() {
        let active_search = searches.get_mut(active_search_index).unwrap();

        // if let Some(cached_cost) =
        //     cache.get(&(node_index_lookup[&active_search.head], active_search.seen))
        // {
        //     if cached_cost > &active_search.cost {
        //         break;
        //     }
        // }

        let next_nodes = graph
            .get(&active_search.head)
            .unwrap()
            .iter()
            .filter(|(coord, ..)| !active_search.seen_contains(node_index_lookup[coord]))
            .copied()
            .collect::<Vec<_>>();

        if let Some(&(next_node, next_node_cost)) = next_nodes.first() {
            active_search.head = next_node;
            active_search.seen_insert(node_index_lookup[&next_node]);
            active_search.cost += next_node_cost;
            // cache
            //     .entry((node_index_lookup[&active_search.head], active_search.seen))
            //     .and_modify(|cost| {
            //         if *cost < active_search.cost {
            //             *cost = active_search.cost;
            //         }
            //     })
            //     .or_insert(active_search.cost);
            active_searches.push_front(active_search_index);

            // if more than 1 move found, then fork search
            for &(other_node, other_cost) in next_nodes.iter().skip(1) {
                let mut new_active_search = searches[active_search_index];
                new_active_search.head = other_node;
                new_active_search.seen_remove(node_index_lookup[&next_node]);
                new_active_search.seen_insert(node_index_lookup[&other_node]);
                new_active_search.cost -= next_node_cost;
                new_active_search.cost += other_cost;
                // cache
                //     .entry((
                //         node_index_lookup[&new_active_search.head],
                //         new_active_search.seen,
                //     ))
                //     .and_modify(|cost| {
                //         if *cost < new_active_search.cost {
                //             *cost = new_active_search.cost;
                //         }
                //     })
                //     .or_insert(new_active_search.cost);
                searches.push(new_active_search);
                active_searches.push_front(searches.len() - 1);
            }
        }
    }

    searches
        .iter()
        .filter(|search| search.head == end)
        .map(|search| search.cost)
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse_input(input);
    let (start, end) = find_start_end(&map);

    let valid_moves_fn = valid_moves_with_slopes;
    let graph = create_graph_from_map(start, end, &map, valid_moves_fn);
    let lengths = find_lengths_in_graph(start, end, graph);

    lengths.iter().max().copied()
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = parse_input(input);
    let (start, end) = find_start_end(&map);

    let valid_moves_fn = valid_moves_without_slopes;
    let directed_graph = create_graph_from_map(start, end, &map, valid_moves_fn);
    let mut undirected_graph = directed_graph.clone();
    for (node, edges) in directed_graph {
        for (edge, cost) in edges {
            undirected_graph.get_mut(&edge).unwrap().push((node, cost));
        }
    }

    let pre_exit_edges = undirected_graph.get(&end).cloned().unwrap();
    for (pre_exit_edge, cost) in pre_exit_edges {
        undirected_graph
            .entry(pre_exit_edge)
            .and_modify(|edges| *edges = vec![(end, cost)]);
    }
    let lengths = find_lengths_in_graph(start, end, undirected_graph);

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
