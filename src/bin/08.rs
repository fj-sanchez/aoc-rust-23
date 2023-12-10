use std::collections::HashMap;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, line_ending, multispace1, one_of},
    combinator::opt,
    multi::{fold_many1, many1},
    sequence::{delimited, separated_pair, terminated},
    Finish, IResult,
};
use num::integer::lcm;

advent_of_code::solution!(8);

#[derive(Debug, PartialEq)]
enum Direction {
    Left = 0,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (i, direction) = one_of("LR")(input)?;
    Ok((i, Direction::try_from(direction).unwrap()))
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Direction>> {
    let (i, moves) = terminated(many1(parse_direction), line_ending)(input)?;
    Ok((i, moves))
}

fn parse_identifier(input: &str) -> IResult<&str, &str> {
    let (i, id) = alpha1(input)?;
    Ok((i, id))
}

fn parse_tuple(input: &str) -> IResult<&str, (&str, &str)> {
    let (i, (left, right)) = delimited(
        char('('),
        separated_pair(parse_identifier, tag(", "), parse_identifier),
        char(')'),
    )(input)?;
    Ok((i, (left, right)))
}

type NodeMap<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn parse_node(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    let node_data = separated_pair(parse_identifier, tag(" = "), parse_tuple);
    let (i, (node_id, (left, right))) = terminated(node_data, opt(line_ending))(input)?;
    Ok((i, (node_id, (left, right))))
}

fn parse_nodes(input: &str) -> IResult<&str, NodeMap> {
    let (i, nodes) = fold_many1(parse_node, HashMap::new, |mut acc: NodeMap, (n, lr)| {
        acc.insert(n, lr);
        acc
    })(input)?;
    Ok((i, nodes))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Direction>, NodeMap)> {
    let (i, (moves, nodes)) = separated_pair(parse_moves, multispace1, parse_nodes)(input)?;
    Ok((i, (moves, nodes)))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, (moves, nodes)) = parse_input(input).finish().unwrap();

    let (steps, _) = moves
        .iter()
        .cycle()
        .fold_while(
            (0u64, "AAA".to_string()),
            |(count, mut next_node): (u64, String), dir: &Direction| {
                if next_node == "ZZZ" {
                    Done((count, next_node))
                } else {
                    match nodes[&next_node as &str] {
                        (left, _) if *dir == Direction::Left => next_node = left.to_string(),
                        (_, right) if *dir == Direction::Right => next_node = right.to_string(),
                        _ => {}
                    }
                    Continue((count + 1, next_node))
                }
            },
        )
        .into_inner();

    Some(steps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, (moves, nodes)) = parse_input(input).finish().unwrap();

    let starting_nodes: Vec<String> = nodes
        .keys()
        .cloned()
        .filter(|n| n.ends_with('A'))
        .map_into()
        .collect_vec();

    let start_end_distances = starting_nodes.iter().map(|starting_node| {
        let (steps, _) = moves
            .iter()
            .cycle()
            .fold_while(
                (0u64, starting_node.clone()),
                |(count, mut next_node): (u64, String), dir: &Direction| {
                    if next_node.ends_with('Z') {
                        Done((count, next_node))
                    } else {
                        match nodes[&next_node as &str] {
                            (left, _) if *dir == Direction::Left => next_node = left.to_string(),
                            (_, right) if *dir == Direction::Right => next_node = right.to_string(),
                            _ => {}
                        }
                        Continue((count + 1, next_node))
                    }
                },
            )
            .into_inner();
        steps
    });

    // This doesn't generalise but works for this problem input
    start_end_distances.into_iter().reduce(lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
