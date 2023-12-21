use pathfinding::{
    directed::{
        astar::astar_bag,
        bfs::bfs_reach,
        dijkstra::{dijkstra_partial, dijkstra_reach},
    },
    grid::Grid,
    matrix::directions::DIRECTIONS_4,
};

advent_of_code::solution!(21);

type Map = Grid;
type Coord = (usize, usize);
fn parse_input(input: &str) -> (Map, Coord) {
    let mut start: Coord = (0, 0);
    let map = Grid::from_iter(
        input
            .lines()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .filter_map(|(x, c)| match c {
                        '.' => Some((x, y)),
                        'S' => {
                            start = (x, y);
                            Some((x, y))
                        }
                        _ => None,
                    })
                    .collect::<Vec<Coord>>()
            })
            .collect::<Vec<Coord>>(),
    );
    (map, start)
}

pub fn part_one(input: &str) -> Option<usize> {
    const NUM_STEPS: usize = 64;
    let (grid, start) = parse_input(input);

    Some(
        bfs_reach((start, 0usize), |&(n, c)| {
            grid.neighbours(n)
                .iter()
                .map(|&n| (n, c + 1))
                .collect::<Vec<(Coord, usize)>>()
        })
        .skip_while(|&(_, c)| c < NUM_STEPS)
        .take_while(|&(_, c)| c == NUM_STEPS)
        .count(),
    )
}

type SignedCoords = (i64, i64);
fn neighbours_tiling(grid: &Map, (nx, ny): SignedCoords) -> Vec<SignedCoords> {
    DIRECTIONS_4
        .iter()
        .filter_map(|&(dx, dy)| {
            let node = (nx + dx as i64, ny + dy as i64);
            let bounded_node = (
                (node.0.rem_euclid(grid.width as i64)) as usize,
                (node.1.rem_euclid(grid.height as i64)) as usize,
            );
            // let bounded_node = (nx as usize, ny as usize);

            match bounded_node {
                n if grid.has_vertex(n) => Some(node),
                _ => None,
            }
        })
        .collect()
}

pub fn part_two(input: &str) -> Option<usize> {
    // const NUM_STEPS: usize = 26501365;
    const NUM_STEPS: usize = 4;
    let (grid, start) = parse_input(input);
    let signed_start = (start.0 as i64, start.1 as i64);

    let n: Vec<SignedCoords> = bfs_reach((signed_start, 0usize), |&(n, c)| {
        neighbours_tiling(&grid, n)
            .iter()
            .map(|&n| (n, c + 1))
            .collect::<Vec<(SignedCoords, usize)>>()
    })
    .skip_while(|&(_, c)| c < NUM_STEPS)
    .take_while(|&(_, c)| c == NUM_STEPS)
    .map(|(n, _)| n)
    .collect();

    let mx = n.iter().min_by_key(|n| n.0).unwrap().0;
    let my = n.iter().min_by_key(|n| n.1).unwrap().1;
    let g = Grid::from_iter(n.iter().map(|&(x, y)| ((x-mx) as usize, (y-my) as usize)));

    println!("{g:#?}");
    Some(n.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(16733044));
    }
}
