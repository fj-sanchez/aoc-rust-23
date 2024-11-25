use std::cmp;

use nalgebra::iter::RowIter;

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

fn find_reflection(values: &[u32]) -> Option<usize> {
    (1..values.len()).find(|&i| {
        let reflection_size = cmp::min(i, values.len() - i);
        let (left, right) = values.split_at(i);
        left.iter()
            .rev()
            .take(reflection_size)
            .eq(right.iter().take(reflection_size))
    })
}

fn summarize_pattern(pattern: &Pattern) -> u32 {
    // Check for vertical reflection (columns)
    if let Some(col) = find_reflection(&pattern.columns) {
        return col as u32;
    }

    // Check for horizontal reflection (rows)
    if let Some(row) = find_reflection(&pattern.rows) {
        return row as u32 * 100;
    }

    0
}

fn find_smudged_reflection(pattern: &Pattern) -> u32 {
    let original_result = summarize_pattern(pattern);
    let row_num_bits = pattern.columns.len();
    // let col_num_bits = pattern.rows.len();

    let mut fixed_pattern = Pattern {
        rows: pattern.rows.clone(),
        columns: pattern.columns.clone(),
    };

    // Try each position for a smudge
    for row_ix in 0..fixed_pattern.rows.len() {
        for i in 0..row_num_bits {
            fixed_pattern.rows[row_ix] ^= 1 << i;
            fixed_pattern.columns[i] ^= 1 << row_ix;

            if let Some(col) = find_reflection(&fixed_pattern.columns) {
                if col as u32 != original_result {
                    return col as u32;
                }
            }

            if let Some(row) = find_reflection(&fixed_pattern.rows) {
                if (row as u32) * 100 != original_result {
                    return (row as u32) * 100;
                }
            }

            fixed_pattern.rows[row_ix] ^= 1 << i;
            fixed_pattern.columns[i] ^= 1 << row_ix;
        }
    }

    panic!("No smudged reflection found");
}

pub fn part_one(input: &str) -> Option<u32> {
    let patterns = parse_input(input);
    Some(patterns.iter().map(summarize_pattern).sum())
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
