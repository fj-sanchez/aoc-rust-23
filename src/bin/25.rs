use itertools::Itertools;
use pathfinding::prelude::{bfs, bfs_reach};

use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use std::collections::HashMap;

use nom::{
    character::complete::{alpha1, char, newline, space1},
    combinator::opt,
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

advent_of_code::solution!(25);

type Component<'a> = (&'a str, Vec<&'a str>);

fn component_connections(input: &str) -> IResult<&str, Component> {
    let (i, component) = separated_pair(
        alpha1,
        char(':'),
        delimited(space1, separated_list1(space1, alpha1), opt(newline)),
    )(input)?;
    Ok((i, component))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Component>> {
    many1(component_connections)(input)
}

fn get_shortest_path_edges(
    start: &&str,
    end: &&str,
    graph: &HashMap<&str, Vec<&str>>,
) -> Vec<(String, String)> {
    bfs(&start, |&n| graph.get(n).unwrap(), |&n| n == end)
        .unwrap()
        .into_iter()
        .map(|n| n.to_string())
        .tuple_windows::<(String, String)>()
        .map(|(a, b)| if a > b { (a, b) } else { (b, a) })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, components_and_connections) = parse_input(input).unwrap();

    let mut components_graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for (component_name, connections) in components_and_connections.iter() {
        components_graph
            .entry(component_name)
            .or_default()
            .extend(connections);
        for c in connections {
            components_graph.entry(c).or_default().push(component_name);
        }
    }

    const SAMPLE_SIZE: usize = 100;

    let component_names: Vec<_> = components_graph.keys().cloned().collect();
    let mut rng = StdRng::seed_from_u64(0);

    let mut components_sample = Vec::new();
    for _ in 0..SAMPLE_SIZE {
        let t = component_names.choose_multiple(&mut rng, 2).collect_vec();
        components_sample.extend(t);
    }
    // component_names.partial_shuffle(, components_graph.len().min(SAMPLE_SIZE));

    let visited_edges_frequency: HashMap<(String, String), usize> = components_sample
        .iter()
        .tuples()
        .fold(HashMap::new(), |mut frequencies, (c1, c2)| {
            for edge in get_shortest_path_edges(c1, c2, &components_graph) {
                *frequencies.entry(edge).or_default() += 1;
            }
            frequencies
        });

    let edges_frequencies = visited_edges_frequency.iter().collect_vec();
    let top_three = edges_frequencies
        .iter()
        .sorted_by_key(|(_, count)| count)
        .rev()
        .take(3)
        .collect_vec();

    for ((component_1, component_2), _frequency) in &top_three {
        // println!("{} to {}: {}", component_1, component_2, _frequency);
        for (c1, c2) in [(component_1, component_2), (component_2, component_1)] {
            if let Some(connections) = components_graph.get_mut(c1.as_str()) {
                if let Some(index) = connections.iter().position(|component| component == c2) {
                    connections.remove(index);
                }
            }
        }
    }

    let component_count_group_1 =
        bfs_reach(&component_names[0], |&n| components_graph.get(n).unwrap()).count();

    Some(component_count_group_1 * (components_graph.len() - component_count_group_1))
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }
}
