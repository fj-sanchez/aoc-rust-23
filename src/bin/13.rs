use std::cmp;

advent_of_code::solution!(13);

#[derive(Debug, Default)]
struct Pattern {
    rows: Vec<u32>,
    columns: Vec<u32>,
}

fn parse_input(input: &str) -> Vec<Pattern> {
    input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|pattern_str| {
            let lines: Vec<&str> = pattern_str.lines().collect();
            let height = lines.len();
            let width = lines[0].len();

            let mut pattern = Pattern {
                rows: vec![0; height],
                columns: vec![0; width],
            };

            for (row_ix, line) in lines.iter().enumerate() {
                for (col_ix, c) in line.chars().enumerate() {
                    let value = (c == '#') as u32;
                    pattern.rows[row_ix] |= value << col_ix;
                    pattern.columns[col_ix] |= value << row_ix;
                }
            }

            pattern
        })
        .collect()
}

fn find_reflection_different_than(values: &[u32], ignore: u32) -> Option<u32> {
    let length = values.len() as u32;
    (1..length).filter(|&v| v != ignore).find(|&i| {
        let reflection_size = cmp::min(i, length - i) as usize;
        let (left, right) = values.split_at(i as usize);
        left.iter()
            .rev()
            .take(reflection_size)
            .eq(right.iter().take(reflection_size))
    })
}

fn summarize_pattern(pattern: &Pattern) -> Option<u32> {
    summarize_pattern_different_than(pattern, 0)
}

fn summarize_pattern_different_than(pattern: &Pattern, ignore: u32) -> Option<u32> {
    // Check for vertical reflection (columns)
    if let Some(col) = find_reflection_different_than(&pattern.columns, ignore % 100) {
        return Some(col);
    }

    // Check for horizontal reflection (rows)
    if let Some(row) = find_reflection_different_than(&pattern.rows, ignore / 100) {
        return Some(row * 100);
    }

    None
}

fn find_smudged_reflection(pattern: &Pattern) -> u32 {
    let original_result = summarize_pattern(pattern).unwrap();
    let row_num_bits = pattern.columns.len();
    // let col_num_bits = pattern.rows.len();

    let mut fixed_pattern = Pattern {
        rows: pattern.rows.clone(),
        columns: pattern.columns.clone(),
    };

    // Try each position for a smudge
    for row_ix in 0..fixed_pattern.rows.len() {
        for bit_ix in 0..row_num_bits {
            fixed_pattern.rows[row_ix] ^= 1 << bit_ix;
            fixed_pattern.columns[bit_ix] ^= 1 << row_ix;

            if let Some(result) = summarize_pattern_different_than(&fixed_pattern, original_result)
            {
                return result;
            }

            fixed_pattern.rows[row_ix] ^= 1 << bit_ix;
            fixed_pattern.columns[bit_ix] ^= 1 << row_ix;
        }
    }

    panic!("No smudged reflection found");
}

pub fn part_one(input: &str) -> Option<u32> {
    let patterns = parse_input(input);
    Some(
        patterns
            .iter()
            .map(summarize_pattern)
            .map(|v| v.unwrap())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let patterns = parse_input(input);
    Some(patterns.iter().map(find_smudged_reflection).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
