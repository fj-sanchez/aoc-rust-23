use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    ops::RangeInclusive,
};

use itertools::Itertools;
use nom::{
    character::complete::{char, u32},
    combinator::map,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};
use pathfinding::directed::dfs::dfs_reach;

advent_of_code::solution!(22);

#[derive(Debug)]
struct Coord3d {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct XYProjection {
    dx: RangeInclusive<usize>,
    dy: RangeInclusive<usize>,
}

fn is_intersect(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
    a.start() <= b.end() && b.start() <= a.end()
}

impl XYProjection {
    fn intersects(&self, other: &XYProjection) -> bool {
        is_intersect(&self.dx, &other.dx) && is_intersect(&self.dy, &other.dy)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Block {
    id: usize,
    z_bottom: usize,
    z_top: usize,
    xy_projection: XYProjection,
}

fn coord(input: &str) -> IResult<&str, Coord3d> {
    map(
        tuple((u32, preceded(char(','), u32), preceded(char(','), u32))),
        |(x, y, z)| Coord3d {
            x: x as usize,
            y: y as usize,
            z: z as usize,
        },
    )(input)
}

fn parse_blocks(input: &str) -> (Coord3d, Coord3d) {
    let (_, block) = separated_pair(coord, char('~'), coord)(input).unwrap();
    block
}

fn parse_input(input: &str) -> Vec<Block> {
    input
        .lines()
        .map(parse_blocks)
        .enumerate()
        .map(|(id, (c1, c2))| {
            assert!(c1.x <= c2.x);
            assert!(c1.y <= c2.y);
            assert!(c1.z <= c2.z);
            Block {
                id,
                z_bottom: c1.z,
                z_top: c2.z,
                xy_projection: XYProjection {
                    dx: c1.x..=c2.x,
                    dy: c1.y..=c2.y,
                },
            }
        })
        .collect::<Vec<_>>()
}

fn drop_and_get_supporting_blocks(mut blocks: Vec<Block>) -> HashMap<usize, Vec<usize>> {
    let mut supported_by_blocks = HashMap::<usize, Vec<usize>>::new();
    let mut top_faces = BTreeMap::<usize, Vec<Block>>::default();
    for block in blocks.iter().cloned() {
        top_faces.entry(block.z_top).or_default().push(block);
    }

    for falling_block in blocks.iter_mut().sorted_by_key(|b| b.z_bottom) {
        supported_by_blocks.entry(falling_block.id).or_default();

        let mut z_min_support = 0usize;
        // println!(
        //     "Falling block {}: {:?}",
        //     (falling_block.id + 65) as u8 as char,
        //     falling_block
        // );

        for (lower_block_top_z, lower_block) in top_faces
            .range(0..falling_block.z_bottom)
            .rev()
            .flat_map(|(z_, projections)| projections.iter().map(|p| (*z_, p)))
        {
            // println!("  Lower block: {}", (lower_block.id + 65) as u8 as char);
            if lower_block_top_z < z_min_support {
                // println!(
                //     "    Lower block {} top z={} below falling block {} supported bottom z={}. Stop searching.",
                //     (lower_block.id + 65) as u8 as char,
                //     lower_block_top_z,
                //     (falling_block.id + 65) as u8 as char,
                //     z_min_support,
                // );
                break;
            }
            if falling_block
                .xy_projection
                .intersects(&lower_block.xy_projection)
            {
                supported_by_blocks
                    .entry(falling_block.id)
                    .or_default()
                    .push(lower_block.id);
                z_min_support = lower_block_top_z;
                // println!(
                //     "    Found: {} top z={} and supports {} bottom z={}.",
                //     (lower_block.id + 65) as u8 as char,
                //     lower_block_top_z,
                //     (falling_block.id + 65) as u8 as char,
                //     falling_block.z_bottom,
                // );
            }
        }
        if z_min_support != falling_block.z_bottom {
            update_top_faces(&mut top_faces, falling_block, z_min_support);
        } else {
            // println!(
            //     "    Not found: ground z=0 supports {} bottom z={}.",
            //     (falling_block.id + 65) as u8 as char,
            //     falling_block.z_bottom
            // );
        }
        falling_block.z_top -= falling_block.z_bottom - (z_min_support + 1);
        falling_block.z_bottom = z_min_support + 1;
        // println!("  Landed: {:?}", falling_block);
    }
    supported_by_blocks
}

fn update_top_faces(
    top_faces: &mut BTreeMap<usize, Vec<Block>>,
    falling_block: &mut Block,
    z_min_support: usize,
) {
    let blocks_at_same_z_top = top_faces.get_mut(&falling_block.z_top).unwrap();
    let mut falling_block_ref = blocks_at_same_z_top.remove(
        blocks_at_same_z_top
            .iter()
            .position(|b| b.id == falling_block.id)
            .expect("Block not found"),
    );
    falling_block_ref.z_top -= falling_block_ref.z_bottom - (z_min_support + 1);
    falling_block_ref.z_bottom = z_min_support + 1;
    top_faces
        .entry(falling_block_ref.z_top)
        .or_default()
        .push(falling_block_ref);
}

pub fn part_one(input: &str) -> Option<usize> {
    let blocks = parse_input(input);
    let supported_by_blocks = drop_and_get_supporting_blocks(blocks);

    // for (k, v) in &supported_by_blocks {
    //     println!("key={} value={:?}", (k + 65) as u8 as char, v);
    // }

    let all_supporting_blocks = supported_by_blocks
        .values()
        .flatten()
        .collect::<BTreeSet<_>>();

    let non_supporting_blocks = supported_by_blocks
        .keys()
        .collect::<BTreeSet<_>>()
        .difference(&all_supporting_blocks)
        .cloned()
        .collect::<BTreeSet<_>>();

    let required_supporting_blocks = supported_by_blocks
        .values()
        .filter(|&supporting_blocks| supporting_blocks.len() == 1)
        .flatten()
        .collect::<BTreeSet<_>>();

    Some(
        all_supporting_blocks
            .difference(&required_supporting_blocks)
            .count()
            + non_supporting_blocks.len(),
    )
}

fn get_supports_from_supported_by(
    supported_by_blocks: &HashMap<usize, Vec<usize>>,
) -> BTreeMap<usize, BTreeSet<usize>> {
    let mut supports_block = BTreeMap::<usize, BTreeSet<usize>>::new();
    // initialise with all blocks as keys
    supported_by_blocks.keys().for_each(|&block| {
        supports_block.entry(block).or_default();
    });

    supported_by_blocks
        .iter()
        .map(|(supported_block, supported_by)| {
            supported_by
                .iter()
                .map(move |supporting_block| (supporting_block, supported_block))
                .collect::<Vec<_>>()
        })
        .fold(&mut supports_block, |acc, supporting_supported| {
            supporting_supported
                .iter()
                .for_each(|(&supporting, &supported)| {
                    acc.entry(supporting).or_default().insert(supported);
                });
            acc
        });
    supports_block
}

pub fn part_two(input: &str) -> Option<usize> {
    let blocks = parse_input(input);
    let supported_by_blocks = drop_and_get_supporting_blocks(blocks);
    let supports_block = get_supports_from_supported_by(&supported_by_blocks);

    let mut total_dropped = 0usize;
    for block_id in supports_block.keys() {
        let mut dropped = BTreeSet::<usize>::default();
        dropped.insert(*block_id);

        let mut successors = |block: &usize| {
            let supported_by_block = supports_block.get(block).unwrap();
            let will_drop = supported_by_block
                .iter()
                .filter(|&supported_block| {
                    supported_by_blocks
                        .get(supported_block)
                        .unwrap()
                        .iter()
                        .filter(|&block| !dropped.contains(block))
                        .count()
                        == 0
                })
                .collect::<Vec<_>>();
            dropped.extend(will_drop.clone());
            will_drop
        };

        let blocks_will_drop = dfs_reach(block_id, |&n| successors(n))
            .skip(1)
            .unique()
            .collect::<Vec<_>>();
        total_dropped += blocks_will_drop.len();
    }

    Some(total_dropped)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(3));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(2));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(2));
    }
}
