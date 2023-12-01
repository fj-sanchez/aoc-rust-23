use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let only_digits = input.lines().map(|line: &str| {
        line.chars()
            .filter_map(|c: char| c.to_digit(10))
            .collect::<Vec<u32>>()
    });

    let cal_values = only_digits.into_iter().map(|line| {
        let first = line.first().unwrap_or(&0);
        let last = line.last().unwrap_or(&0);
        first * 10 + last
    });

    Some(cal_values.sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let all_digits: HashMap<&str, u32> = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let cal_values = input.lines().map(|line| {
        all_digits
            .iter()
            .flat_map(|(&k, &v)| line.match_indices(k).map(move |(i, _)| (i, v)))
            .sorted_by_key(|(index, _)| *index)
            .with_position()
            .filter_map(|(position, (_, value))| match position {
                itertools::Position::First => Some(value * 10),
                itertools::Position::Last => Some(value),
                _ => None,
            })
            .sum::<u32>()
    });

    Some(cal_values.sum::<u32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
