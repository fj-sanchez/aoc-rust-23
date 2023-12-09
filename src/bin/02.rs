use regex::Regex;
use std::collections::HashMap;

advent_of_code::solution!(2);

fn update_game_max_seen<'a>(line: &'a str, game_max_seen: &mut HashMap<&'a str, u32>) {
    let re = Regex::new(r"(\d+) (red|blue|green)").unwrap();

    line.split(';')
        .flat_map(|group| re.captures_iter(group))
        .filter_map(|captures| {
            let number = captures.get(1).and_then(|m| m.as_str().parse::<u32>().ok());
            let colour = captures.get(2).map(|m| m.as_str());

            number.and_then(|number| colour.map(|colour| (number, colour)))
        })
        .for_each(|(number, colour)| {
            if let Some(current_max) = game_max_seen.get_mut(colour) {
                if *current_max < number {
                    *current_max = number;
                }
            }
        });
}

pub fn part_one(input: &str) -> Option<u32> {
    let max_available: HashMap<&str, u32> =
        HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let default_game_result_map: HashMap<&str, u32> =
        HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
    let mut games_max_seen: Vec<HashMap<&str, u32>> =
        vec![default_game_result_map.clone(); input.lines().count()];

    for (line, game_max_seen) in input.lines().zip(games_max_seen.iter_mut()) {
        update_game_max_seen(line, game_max_seen);
    }

    Some(
        games_max_seen
            .iter()
            .enumerate()
            .filter_map(|(game_index, game_result)| {
                (game_result.iter().all(|(c, v)| v <= &max_available[c]))
                    .then_some(game_index as u32 + 1)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let default_game_result_map: HashMap<&str, u32> =
        HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
    let mut games_max_seen: Vec<HashMap<&str, u32>> =
        vec![default_game_result_map.clone(); input.lines().count()];

    for (line, game_max_seen) in input.lines().zip(games_max_seen.iter_mut()) {
        update_game_max_seen(line, game_max_seen);
    }

    Some(
        games_max_seen
            .iter()
            .map(|game| game.values().product::<u32>())
            .sum(),
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
