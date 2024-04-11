use std::collections::BTreeSet;

use pathfinding::{directed::bfs::bfs_reach, grid::Grid, matrix::directions::DIRECTIONS_4};

advent_of_code::solution!(21);

type Map = Grid;
type Coord = (usize, usize);
type SignedCoord = (isize, isize);

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

#[inline(always)]
fn same_parity(a: usize, b: usize) -> bool {
    (a & 1) == (b & 1)
}

fn reachable_plots(start: (usize, usize), num_steps: usize, grid: &Grid) -> Vec<(usize, usize)> {
    let mut seen: BTreeSet<Coord> = BTreeSet::new();
    seen.insert(start);
    bfs_reach(
        (start, 0usize, same_parity(num_steps, 0)),
        |&(coords, steps, _)| {
            grid.neighbours(coords)
                .iter()
                .map(|&n| {
                    (
                        n,
                        steps + 1,
                        steps < num_steps && same_parity(num_steps, steps + 1),
                    )
                })
                .filter(|&(_, c, _)| c <= num_steps)
                .filter(|&(n, ..)| seen.insert(n))
                .collect::<Vec<_>>()
        },
    )
    .filter(|&(.., is_final)| is_final)
    .map(|(coords, ..)| coords)
    .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    // const NUM_STEPS: usize = 6;
    const NUM_STEPS: usize = 64;
    let (grid, start) = parse_input(input);

    let reachable = reachable_plots(start, NUM_STEPS, &grid);
    Some(reachable.len())
}

fn neighbours_tiling(grid: &Map, (nx, ny): SignedCoord) -> Vec<SignedCoord> {
    DIRECTIONS_4
        .iter()
        .filter_map(|&(dx, dy)| {
            let node = (nx + dx, ny + dy);
            let bounded_node = (
                (node.0.rem_euclid(grid.width as isize)) as usize,
                (node.1.rem_euclid(grid.height as isize)) as usize,
            );

            match bounded_node {
                n if grid.has_vertex(n) => Some(node),
                _ => None,
            }
        })
        .collect()
}

pub fn part_two(input: &str) -> Option<usize> {
    // const NUM_STEPS: usize = 17;
    const NUM_STEPS: usize = 26501365;

    let (grid, start) = parse_input(input);
    let signed_start = (start.0 as isize, start.1 as isize);

    // The input is 131x131 with S in the centre.
    // There are only empty plots from S on each straight direction which means
    // 65 steps to get to any repeated tile. If we consider a single direction we
    // could travel (26501365-65)/131 = 202,300 tiles in that direction
    let dist_to_edges = (grid.width - 1) / 2;
    let tiles_per_dir: usize = (NUM_STEPS - dist_to_edges) / grid.width;

    // let's calculate for a simplified case of 2 extra tiles per direction which
    // would form a 5x5 grid of tiles
    let extra_tiles: usize = 2;
    let reduced_num_steps = dist_to_edges + grid.width * extra_tiles;

    // With that number of steps we can reach every middle plot at the end of the
    // tiles on each cardinal direction. Equally, every other plot falling under
    // the diamond formed by these 4 points would be reachable. There would be some
    // tiles fully reachable and some only partially that are repated over and over:
    //
    //      __ s1 N s2 __
    //      s1 b1 e b2 s2
    //      W  e  o  e  E
    //      s3 b3 e b4 s4
    //      __ s3 S s4 __
    //
    // Fully reachable:
    // - tile at the centre [o]: only same parity plots would be reachable.
    // - adjacent tiles [e]: because tiles have an odd number of plots, only
    //      opposite parity plots would be reachable.
    // - adjacent to these: repeats the same pattern as the centre one, same parity.
    //      These don't show up in this example but they would if we increase the size.
    // Partially reachable:
    // - diamon corners [NESW]: at the end of each cardinal direction, a triangle pointing in
    //      cardinal direction would be reachable, each of them would be different.
    // - small corners [s1-4]: next to the diamon corners along the line of the diamound countour
    //      some small corners would be reachable. There would be 8 of these in this example,
    //      of which only 4 would be different.
    // - big corners [b1-4]: between small corners some bigger ones would be reachable. This is
    //      similar to the previous ones, but in this example only 4, all different, appear.
    // Because these repeat, if we call N to the number of tiles that would be repeated in one
    //  direction, then we can calculate how many times each type would be repeated:
    // o = (N - 1)^2
    // e = N^2
    // NESW = 1
    // s1-4 = N (for each type)
    // b1-4 = N - 1 (for each type)
    let reachable = reachable_plots_with_tiling(signed_start, reduced_num_steps, &grid);
    let o_tiles = (tiles_per_dir - 1).pow(2);
    let e_tiles = tiles_per_dir.pow(2);
    let s_tiles = tiles_per_dir;
    let b_tiles = tiles_per_dir - 1;

    // create a 2D array to represent the 5x5 tile arrangement described previously
    // then count how many reachable plots there are on each of these tiles
    let mut tiles = [[0usize; 5]; 5];
    let width = grid.width as isize;
    let height = grid.height as isize;
    for &(x, y) in &reachable {
        let tile_x = ((x + 2 * width) / width) as usize;
        let tile_y = ((y + 2 * height) / height) as usize;
        tiles[tile_y][tile_x] += 1;
    }

    let o_plots = o_tiles * tiles[2][2];
    let e_plots = e_tiles * tiles[1][2];
    let n_plots = tiles[0][2];
    let w_plots = tiles[2][0];
    let s_plots = tiles[4][2];
    let ee_plots = tiles[2][4];
    let s1_plots = s_tiles * tiles[0][1];
    let s2_plots = s_tiles * tiles[0][3];
    let s3_plots = s_tiles * tiles[4][1];
    let s4_plots = s_tiles * tiles[4][3];
    let b1_plots = b_tiles * tiles[1][1];
    let b2_plots = b_tiles * tiles[1][3];
    let b3_plots = b_tiles * tiles[3][1];
    let b4_plots = b_tiles * tiles[3][3];

    let total_plots = o_plots
        + e_plots
        + n_plots
        + ee_plots
        + s_plots
        + w_plots
        + s1_plots
        + s2_plots
        + s3_plots
        + s4_plots
        + b1_plots
        + b2_plots
        + b3_plots
        + b4_plots;

    Some(total_plots)
}

fn reachable_plots_with_tiling(
    signed_start: SignedCoord,
    num_steps: usize,
    grid: &Grid,
) -> Vec<SignedCoord> {
    let mut seen: BTreeSet<SignedCoord> = BTreeSet::new();
    seen.insert(signed_start);
    bfs_reach(
        (signed_start, 0usize, same_parity(num_steps, 0)),
        |&(coords, steps, _)| {
            neighbours_tiling(grid, coords)
                .iter()
                .map(|&n| {
                    (
                        n,
                        steps + 1,
                        steps < num_steps && same_parity(num_steps, steps + 1),
                    )
                })
                .filter(|&(_, c, _)| c <= num_steps)
                .filter(|&(n, ..)| seen.insert(n))
                .collect::<Vec<_>>()
        },
    )
    .filter(|&(.., is_final)| is_final)
    .map(|(coord, ..)| coord)
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let _result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let _result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        // assert_eq!(result, Some(324));
    }
}
