use indexmap::IndexMap;
use nom::{
    character::complete::{alpha1, char, one_of, u8},
    combinator::{opt},
    multi::{separated_list1},
    sequence::{tuple},
    Finish, IResult,
};

advent_of_code::solution!(15);

enum Operation {
    Insert,
    Remove,
}
struct Step {
    label: String,
    operation: Operation,
    lens_fl: Option<u8>,
}

#[inline]
fn calculate_hash(current_value: u32, character: char) -> u32 {
    (((current_value + character as u32) * 17) % 256)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Step>> {
    let step = tuple((alpha1, one_of("=-"), opt(u8)));
    // let as_step = map(step, |(seq, op, lens_fl)| Step { seq, op, lens_fl });
    let (i, steps_data) = separated_list1(char(','), step)(input)?;
    let steps = steps_data
        .into_iter()
        .map(|(label, op, lens_fl)| {
            let operation = match op {
                '=' => Ok(Operation::Insert),
                '-' => Ok(Operation::Remove),
                _ => Err(()),
            }
            .unwrap();
            Step {
                label: label.to_string(),
                operation,
                lens_fl,
            }
        })
        .collect();

    Ok((i, steps))
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .replace('\n', "")
            .split(',')
            .map(|s| s.chars().fold(0, calculate_hash))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, steps) = parse_input(input.replace('\n', "").as_str())
        .finish()
        .unwrap();

    let mut boxes: [IndexMap<String, u8>; 256] = std::array::from_fn(|_| IndexMap::new());

    for step in &steps {
        let box_number = step.label.chars().fold(0, calculate_hash) as usize;
        match step.operation {
            Operation::Insert => {
                *boxes[box_number].entry(step.label.clone()).or_insert(0) = step.lens_fl.unwrap();
            }
            Operation::Remove => {
                boxes[box_number].shift_remove(step.label.as_str());
            }
        }
    }

    Some(
        boxes
            .iter()
            .enumerate()
            .flat_map(|(box_number, box_)| {
                box_.iter()
                    .enumerate()
                    .map(move |(slot_number, (_, &lens_fl))| {
                        ((box_number + 1) * (slot_number + 1) * (lens_fl as usize)) as u32
                    })
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
