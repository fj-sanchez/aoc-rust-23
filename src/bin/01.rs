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
    let all_digits = HashMap::from([
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

    let lines_digits_indices: Vec<Vec<(usize, &str)>> = input
        .lines()
        .map(|line| {
            all_digits
                .keys()
                .flat_map(|k| line.match_indices(k))
                .sorted()
                .collect()
        })
        .collect();

    let mut result: u32 = 0;
    for line in lines_digits_indices {
        let (_, k): &(usize, &str) = line.first()?;
        result += all_digits[k] * 10;

        // starting from the left, remove any match that overlaps with any previous match
        let mut last_digit = "";
        let mut next_valid_position: usize = 0;
        for (start, digit) in line {
            if start < next_valid_position {
                continue;
            }
            last_digit = digit;
            next_valid_position = start + digit.len();
        }
        result += all_digits.get(last_digit)?;
    }
    Some(result)
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
