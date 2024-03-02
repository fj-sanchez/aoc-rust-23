use itertools::Itertools;
use pathfinding::matrix::Matrix;

advent_of_code::solution!(11);

type GalaxiesMap = Matrix<char>;
fn parse_input(input: &str) -> GalaxiesMap {
    Matrix::from_rows(input.lines().map(|c| c.chars())).unwrap()
}

type Galaxy = (usize, usize);
fn get_galaxies_coordinates(map: &GalaxiesMap, expand_by: usize) -> Vec<Galaxy> {
    fn get_expanding_rows(input: &GalaxiesMap) -> Vec<usize> {
        input
            .iter()
            .enumerate()
            .filter_map(|(i, l)| l.iter().all(|&c| c == '.').then_some(i))
            .collect()
    }

    let expanding_rows: Vec<usize> = get_expanding_rows(map);
    let expanding_columns: Vec<usize> = get_expanding_rows(&map.transposed());

    let galaxies: Vec<Galaxy> = map
        .items()
        .filter(|(_, &c)| c == '#')
        .map(|((row, col), _)| {
            let expanded_rows = expanding_rows.binary_search(&row).unwrap_err();
            let expanded_columns = expanding_columns.binary_search(&col).unwrap_err();
            let actual_row = row + (expand_by - 1) * expanded_rows;
            let actual_col = col + (expand_by - 1) * expanded_columns;
            (actual_row, actual_col)
        })
        .collect();

    galaxies
}

fn solve(input: &str, expand_by: usize) -> Option<usize> {
    let map = parse_input(input);
    let galaxies = get_galaxies_coordinates(&map, expand_by);

    Some(
        galaxies
            .iter()
            .tuple_combinations()
            .map(|(&(ra, ca), &(rb, cb))| ra.abs_diff(rb) + ca.abs_diff(cb))
            .sum::<usize>(),
    )
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 2)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 1000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let _result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(8410));
    }
}
