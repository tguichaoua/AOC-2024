use std::collections::{HashSet, VecDeque};

use advent_of_code::four_directions_bounded;
use glam::{uvec2, UVec2};

advent_of_code::solution!();

fn parse(input: &str) -> impl Iterator<Item = UVec2> + '_ {
    input.lines().map(|line| {
        let (x, y) = line.split_once(',').unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        uvec2(x, y)
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve_one(input, uvec2(71, 71), 1024))
}

fn solve_one(input: &str, bounds: UVec2, fallen_bytes: usize) -> u32 {
    struct Explorer {
        pos: UVec2,
        distance: u32,
    }

    let mut visited = HashSet::new();
    let mut to_explore = VecDeque::new();

    to_explore.push_back(Explorer {
        pos: uvec2(0, 0),
        distance: 0,
    });
    visited.insert(uvec2(0, 0));

    let goal = bounds - uvec2(1, 1);
    let walls: HashSet<_> = parse(input).take(fallen_bytes).collect();

    while let Some(explorer) = to_explore.pop_front() {
        for pos in four_directions_bounded(explorer.pos, bounds) {
            if pos == goal {
                return explorer.distance + 1;
            }

            if !walls.contains(&pos) && visited.insert(pos) {
                to_explore.push_back(Explorer {
                    pos,
                    distance: explorer.distance + 1,
                });
            }
        }
    }

    unreachable!()
}

/* -------------------------------------------------------------------------- */

fn solve_two(input: &str, bounds: UVec2) -> String {
    let falling_bytes = parse(input);

    let mut walls = HashSet::new();

    let mut visited = HashSet::new();
    let mut to_explore = Vec::new();

    let goal = bounds - uvec2(1, 1);

    'falling_bytes: for byte in falling_bytes {
        walls.insert(byte);

        visited.clear();
        to_explore.clear();

        to_explore.push(uvec2(0, 0));
        visited.insert(uvec2(0, 0));

        while let Some(pos) = to_explore.pop() {
            for pos in four_directions_bounded(pos, bounds) {
                if pos == goal {
                    continue 'falling_bytes;
                }

                if !walls.contains(&pos) && visited.insert(pos) {
                    to_explore.push(pos);
                }
            }
        }

        return format!("{},{}", byte.x, byte.y);
    }

    unreachable!();
}

pub fn part_two(input: &str) -> Option<String> {
    Some(solve_two(input, uvec2(71, 71)))
}

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_one(
            &advent_of_code::template::read_file("examples", DAY),
            uvec2(7, 7),
            12,
        );
        assert_eq!(result, 22);
    }

    #[test]
    fn test_part_two() {
        let result = solve_two(
            &advent_of_code::template::read_file("examples", DAY),
            uvec2(7, 7),
        );
        assert_eq!(result, "6,1");
    }
}
