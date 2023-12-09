use regex::Regex;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let width: usize = input.lines().next()?.len();
    let symbols_re: Regex = Regex::new(r"[^a-zA-Z0-9.]").unwrap();
    let numbers_re: Regex = Regex::new(r"\d+").unwrap();

    let linear_input = input.replace('\n', "");
    let valid_positions: HashSet<i32> = symbols_re
        .find_iter(&linear_input)
        .flat_map(|symbol| {
            let symbol_pos = symbol.start() as i32;
            let stride = width as i32;
            [-stride, 0, stride]
                .iter()
                .flat_map(move |stride_| (symbol_pos + stride_ - 1)..=(symbol_pos + stride_ + 1))
                .filter(|&pos| pos >= 0)
                .collect::<HashSet<i32>>()
        })
        .collect();

    Some(
        numbers_re
            .find_iter(&linear_input)
            .filter(|num| {
                num.range()
                    .any(|pos| valid_positions.contains(&(pos as i32)))
            })
            .filter_map(|part_number| part_number.as_str().parse::<u32>().ok())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let width: usize = input.lines().next()?.len();
    let symbols_re: Regex = Regex::new(r"\*").unwrap();
    let numbers_re: Regex = Regex::new(r"\d+").unwrap();

    let linear_input = input.replace('\n', "");
    let gears_valid_positions: HashMap<i32, HashSet<i32>> = symbols_re
        .find_iter(&linear_input)
        .map(|symbol| {
            let symbol_pos = symbol.start() as i32;
            let stride = width as i32;
            let valid_positions = [-stride, 0, stride]
                .iter()
                .flat_map(move |stride_| (symbol_pos + stride_ - 1)..=(symbol_pos + stride_ + 1))
                .filter(|&pos| pos >= 0)
                .collect::<HashSet<i32>>();
            (symbol_pos, valid_positions)
        })
        .collect();

    let result = numbers_re
        .find_iter(&linear_input)
        .fold(HashMap::new(), |mut acc: HashMap<i32, Vec<u32>>, num| {
            for (&gear, valid_positions) in &gears_valid_positions {
                if num
                    .range()
                    .any(|pos| valid_positions.contains(&(pos as i32)))
                {
                    acc.entry(gear)
                        .or_default()
                        .push(num.as_str().parse::<u32>().unwrap());
                }
            }
            acc
        })
        .values()
        .filter(|&v| (v.len() == 2))
        .map(|v| v.iter().product::<u32>())
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
