use regex::Regex;
use std::collections::HashMap;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"(\d+) (red|blue|green)").unwrap();
    let max_available: HashMap<&str, u32> =
        HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let default_game_result_map: HashMap<&str, u32> =
        HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
    let mut games_max_seen: Vec<HashMap<&str, u32>> =
        vec![default_game_result_map.clone(); input.lines().count()];

    for (line, game_max_seen) in input.lines().zip(games_max_seen.iter_mut()) {
        for group in line.split(';') {
            for captures in re.captures_iter(group) {
                if let (Some(number), Some(colour)) = (
                    captures.get(1).and_then(|m| m.as_str().parse::<u32>().ok()),
                    captures.get(2).map(|m| m.as_str()),
                ) {
                    if let Some(current_max) = game_max_seen.get_mut(colour) {
                        if *current_max < number {
                            *current_max = number;
                        }
                    }
                }
            }
        }
    }

    let mut result: u32 = 0;
    for (game_num, game_result) in games_max_seen.iter().enumerate() {
        if game_result.iter().all(|(c, v)| v <= &max_available[c]) {
            result += game_num.try_into().unwrap_or(0) + 1;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(\d+) (red|blue|green)").unwrap();
    let default_game_result_map: HashMap<&str, u32> =
        HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
    let mut games_max_seen: Vec<HashMap<&str, u32>> =
        vec![default_game_result_map.clone(); input.lines().count()];

    for (line, game_max_seen) in input.lines().zip(games_max_seen.iter_mut()) {
        for group in line.split(';') {
            for captures in re.captures_iter(group) {
                if let (Some(number), Some(colour)) = (
                    captures.get(1).and_then(|m| m.as_str().parse::<u32>().ok()),
                    captures.get(2).map(|m| m.as_str()),
                ) {
                    if let Some(current_max) = game_max_seen.get_mut(colour) {
                        if *current_max < number {
                            *current_max = number;
                        }
                    }
                }
            }
        }
    }

    Some(
        games_max_seen
            .iter()
            .map(|game| game.values().product::<u32>())
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
