use pathfinding::{directed::astar::astar, matrix::Matrix};

advent_of_code::solution!(17);

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
struct Node {
    position: (usize, usize),
    direction: (isize, isize),
    steps_in_direction: usize,
}

impl Node {
    fn distance_to_position(self, position: &(usize, usize)) -> u32 {
        (self.position.0.abs_diff(position.0) + self.position.1.abs_diff(position.1)) as u32
    }
}

fn parse_input(input: &str) -> Matrix<u32> {
    Matrix::from_rows(
        input
            .lines()
            .map(|l| l.chars().filter_map(|c| c.to_digit(10))),
    )
    .unwrap()
}

fn find_shortest_path_cost(
    map: &Matrix<u32>,
    min_steps_in_direction: usize,
    max_steps_in_direction: usize,
) -> u32 {
    let start_node = Node {
        position: (0, 0),
        direction: (0, 0),
        steps_in_direction: 0,
    };

    let goal_position = (map.rows - 1, map.columns - 1);

    let (_, cost) = astar(
        &start_node,
        |node| {
            let mut successors = Vec::with_capacity(3);

            let mut create_successor = |direction, steps_in_direction| {
                successors.extend(map.move_in_direction(node.position, direction).map(
                    |position| {
                        (
                            Node {
                                position,
                                direction,
                                steps_in_direction,
                            },
                            map[position],
                        )
                    },
                ));
            };

            if node.steps_in_direction < max_steps_in_direction {
                create_successor(node.direction, node.steps_in_direction + 1);
            }

            if node.steps_in_direction >= min_steps_in_direction {
                create_successor((-node.direction.1, -node.direction.0), 1);
                create_successor((node.direction.1, node.direction.0), 1);
            } else if node.steps_in_direction == 0 {
                create_successor((1, 0), 1);
                create_successor((0, 1), 1);
            }
            successors
        },
        |node| node.distance_to_position(&goal_position),
        |node| node.position == goal_position && node.steps_in_direction >= min_steps_in_direction,
    )
    .unwrap();

    cost
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    Some(find_shortest_path_cost(&grid, 1, 3))
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    Some(find_shortest_path_cost(&grid, 4, 10))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
