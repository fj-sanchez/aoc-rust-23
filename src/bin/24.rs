use itertools::Itertools;
use nalgebra::Vector3;
use nom::{
    character::complete::{char, i64, multispace0},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};
use num::Zero;

advent_of_code::solution!(24);

type Coord3D = (i64, i64, i64);
type CoordFloat3D = (f64, f64, f64);
type Hailstone = (Coord3D, Coord3D);
type Region = (CoordFloat3D, CoordFloat3D);

fn coord3d(input: &str) -> IResult<&str, Vec<i64>> {
    let number = delimited(multispace0, i64, multispace0);
    separated_list1(char(','), number)(input)
}

fn position_velocity(input: &str) -> IResult<&str, (Vec<i64>, Vec<i64>)> {
    separated_pair(coord3d, char('@'), coord3d)(input)
}

fn parse_input(input: &str) -> Vec<Hailstone> {
    let hailstones = input
        .lines()
        .map(|l| {
            let (_, (pos, vel)) = position_velocity(l).unwrap();
            (
                pos.iter().copied().collect_tuple().unwrap(),
                vel.iter().copied().collect_tuple().unwrap(),
            )
        })
        .collect();
    hailstones
}

fn intersection_point_2d(a: Hailstone, b: Hailstone) -> Option<CoordFloat3D> {
    let ((a_px, a_py, _a_pz), (a_vx, a_vy, _a_vz)) = a;
    let ((b_px, b_py, _b_pz), (b_vx, b_vy, _b_vz)) = b;

    let (p0_x, p0_y, _p1_x, _p1_y) = (a_px, a_py, a_px + a_vx, a_py + a_vy);
    let (p2_x, p2_y, _p3_x, _p3_y) = (b_px, b_py, b_px + b_vx, b_py + b_vy);
    let (s1_x, s1_y) = (a_vx, a_vy);
    let (s2_x, s2_y) = (b_vx, b_vy);

    let denominator = (-s2_x * s1_y + s1_x * s2_y) as f64;

    let (s, t) = match denominator {
        _ if denominator.is_zero() => (None, None),
        denominator => {
            let s = ((-s1_y * (p0_x - p2_x) + s1_x * (p0_y - p2_y)) as f64) / denominator;
            let t = ((s2_x * (p0_y - p2_y) - s2_y * (p0_x - p2_x)) as f64) / denominator;
            (Some(s), Some(t))
        }
    };

    match (s, t) {
        // Collision detected
        (Some(s), Some(t)) if s >= 0.0 && t >= 0.0 => Some((
            p0_x as f64 + (t * s1_x as f64),
            p0_y as f64 + (t * s1_y as f64),
            0.0,
        )),
        _ => None, // No collision
    }
}

fn point_in_region_2d(point: CoordFloat3D, region: Region) -> bool {
    let (px, py, _pz) = point;
    let (top_left_corner, bottom_right_corner) = region;
    let (tl_x, tl_y, _tl_z) = top_left_corner;
    let (br_x, br_y, _br_z) = bottom_right_corner;
    px >= tl_x && px <= br_x && py >= br_y && py <= tl_y
}

fn count_2d_intersections_in_region(input: &str, region: Region) -> Option<usize> {
    let hailstones = parse_input(input);
    Some(
        hailstones
            .iter()
            .tuple_combinations()
            .filter_map(|(a, b)| intersection_point_2d(*a, *b))
            .filter(|&a| point_in_region_2d(a, region))
            .count(),
    )
}

const REGION: Region = (
    (200000000000000.0, 400000000000000.0, 0.0),
    (400000000000000.0, 200000000000000.0, 0.0),
);

pub fn part_one_test(input: &str) -> Option<usize> {
    {
        const TEST_REGION: Region = ((7.0, 27.0, 0.0), (27.0, 7.0, 0.0));
        count_2d_intersections_in_region(input, TEST_REGION)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    count_2d_intersections_in_region(input, REGION)
}

pub fn part_two(input: &str) -> Option<usize> {
    let hailstones = parse_input(input);

    // take any 3 hailstones and make the last 2 relative to the first one
    let (&h0, &h1, &h2) = hailstones.iter().take(3).collect_tuple().unwrap();
    let p1: Vector3<i128> = nalgebra::convert(
        Vector3::new(h1.0 .0, h1.0 .1, h1.0 .2) - Vector3::new(h0.0 .0, h0.0 .1, h0.0 .2),
    );
    let v1: Vector3<i128> = nalgebra::convert(
        Vector3::new(h1.1 .0, h1.1 .1, h1.1 .2) - Vector3::new(h0.1 .0, h0.1 .1, h0.1 .2),
    );

    let p2: Vector3<i128> = nalgebra::convert(
        Vector3::new(h2.0 .0, h2.0 .1, h2.0 .2) - Vector3::new(h0.0 .0, h0.0 .1, h0.0 .2),
    );
    let v2: Vector3<i128> = nalgebra::convert(
        Vector3::new(h2.1 .0, h2.1 .1, h2.1 .2) - Vector3::new(h0.1 .0, h0.1 .1, h0.1 .2),
    );

    // the vectors where the collitions happen, are p1 + t1 * v1 and p2 + t2 * v2, and because the desired trajectory collides with the 3 hailstones,
    // from hailstone 0 these 2 vectors are collineal, so their cross product must be 0. Using this and the fact that (axb)*a = (axb)*b = 0, we can
    // reduce to the next to equations for the collision times.
    let t1_collision = (-(p1.cross(&p2).dot(&v2)) / v1.cross(&p2).dot(&v2)) as i64;
    let t2_collision = (-(p1.cross(&p2).dot(&v1)) / p1.cross(&v2).dot(&v1)) as i64;

    // we can now calculate the collistion position for hailstones 1 and 2, calculate the velocity of our searched hailstone
    // and then find where its starting position was
    let collision_1 = Vector3::new(h1.0 .0, h1.0 .1, h1.0 .2)
        + t1_collision * Vector3::new(h1.1 .0, h1.1 .1, h1.1 .2);
    let collision_2 = Vector3::new(h2.0 .0, h2.0 .1, h2.0 .2)
        + t2_collision * Vector3::new(h2.1 .0, h2.1 .1, h2.1 .2);
    let velocity = (collision_2 - collision_1) / (t2_collision - t1_collision);
    let position = collision_1 - t1_collision * velocity;

    Some(position.sum() as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_test(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(47));
    }
}
