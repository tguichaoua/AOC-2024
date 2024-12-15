use std::collections::HashSet;

use advent_of_code::{parse_ascii_map, Dir};
use glam::{uvec2, UVec2};

advent_of_code::solution!();

/* -------------------------------------------------------------------------- */

fn parse_one(
    input: &str,
) -> (
    UVec2,
    HashSet<UVec2>,
    HashSet<UVec2>,
    impl Iterator<Item = Dir> + '_,
) {
    debug_assert!(input.is_ascii());

    let (map, moves) = input.split_once("\n\n").unwrap();

    let mut robot_position = None;
    let mut box_positions = HashSet::new();
    let mut walls = HashSet::new();

    parse_ascii_map(map).for_each(|(pos, c)| {
        let pos = pos.try_into().unwrap();
        match c {
            b'#' => {
                walls.insert(pos);
            }
            b'O' => {
                box_positions.insert(pos);
            }
            b'.' => { /* nothing */ }
            b'@' => {
                debug_assert!(robot_position.is_none());
                robot_position = Some(pos);
            }
            _ => unreachable!(),
        }
    });

    let moves = moves
        .lines()
        .flat_map(|line| line.bytes())
        .map(|c| match c {
            b'^' => Dir::Up,
            b'>' => Dir::Right,
            b'v' => Dir::Down,
            b'<' => Dir::Left,
            _ => unreachable!(),
        });

    let robot_position = robot_position.unwrap();

    (robot_position, walls, box_positions, moves)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut robot_position, walls, mut box_positions, moves) = parse_one(input);

    let mut box_to_move = Vec::new();
    for dir in moves {
        let dir = dir.as_vec_down_right();

        let mut pos = robot_position;

        let can_move = loop {
            // `pos`` cannot be at left-most or top-most thanks to walls so it shouldn't actually wrap.
            pos = pos.wrapping_add_signed(dir);

            if walls.contains(&pos) {
                // We reach a wall, we cannot move
                break false;
            } else if let Some(box_pos) = box_positions.get(&pos).copied() {
                // We found a box, wait to see if we can move it
                box_to_move.push(box_pos);
            } else {
                // We found a hole, we can move
                break true;
            }
        };

        if can_move {
            robot_position = robot_position.wrapping_add_signed(dir);
            for box_pos in box_to_move.drain(..).rev() {
                box_positions.remove(&box_pos);
                box_positions.insert(box_pos.wrapping_add_signed(dir));
            }
        } else {
            box_to_move.clear();
        }
    }

    Some(
        box_positions
            .into_iter()
            .map(|pos| 100 * pos.y + pos.x)
            .sum(),
    )
}

/* -------------------------------------------------------------------------- */

fn parse_two(
    input: &str,
) -> (
    UVec2,
    HashSet<UVec2>,
    HashSet<UVec2>,
    impl Iterator<Item = Dir> + '_,
) {
    debug_assert!(input.is_ascii());

    let (map, moves) = input.split_once("\n\n").unwrap();

    let mut robot_position = None;
    let mut box_positions = HashSet::new();
    let mut walls = HashSet::new();

    parse_ascii_map(map).for_each(|(pos, c)| {
        let mut pos: UVec2 = pos.try_into().unwrap();
        pos.x *= 2;

        match c {
            b'#' => {
                walls.insert(pos);
                walls.insert(pos + uvec2(1, 0));
            }
            b'O' => {
                box_positions.insert(pos);
            }
            b'.' => { /* nothing */ }
            b'@' => {
                debug_assert!(robot_position.is_none());
                robot_position = Some(pos);
            }
            _ => unreachable!(),
        }
    });

    let moves = moves
        .lines()
        .flat_map(|line| line.bytes())
        .map(|c| match c {
            b'^' => Dir::Up,
            b'>' => Dir::Right,
            b'v' => Dir::Down,
            b'<' => Dir::Left,
            _ => unreachable!(),
        });

    let robot_position = robot_position.unwrap();

    (robot_position, walls, box_positions, moves)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut robot_position, walls, mut box_positions, moves) = parse_two(input);

    let mut box_to_move = Vec::new();
    let mut pos_to_check = Vec::new();
    for dir in moves {
        let is_vertical = dir.is_vertical();
        let movement = dir.as_vec_down_right();

        let mut can_move = true;
        pos_to_check.push(robot_position);

        while let Some(pos) = pos_to_check.pop() {
            // `pos` cannot be at left-most or top-most thanks to walls so it shouldn't actually wrap.
            let pos = pos.wrapping_add_signed(movement);

            if walls.contains(&pos) {
                // We reach a wall, we cannot move
                can_move = false;
                break;
            } else if let Some(box_pos) = box_positions.get(&pos).copied() {
                // We found a box, wait to see if we can move it
                box_to_move.push(box_pos);

                if is_vertical {
                    pos_to_check.push(pos);
                    pos_to_check.push(pos + uvec2(1, 0));
                } else {
                    pos_to_check.push(pos + uvec2(1, 0));
                }
            } else if let Some(box_pos) = box_positions.get(&(pos - uvec2(1, 0))).copied() {
                // We found a box, wait to see if we can move it
                box_to_move.push(box_pos);

                if is_vertical {
                    pos_to_check.push(pos);
                    pos_to_check.push(pos - uvec2(1, 0));
                } else {
                    pos_to_check.push(pos - uvec2(1, 0));
                }
            } else {
                // We found a hole, we can move
            }
        }

        if can_move {
            debug_assert!(pos_to_check.is_empty());
            robot_position = robot_position.wrapping_add_signed(movement);
            for box_pos in box_to_move.drain(..).rev() {
                box_positions.remove(&box_pos);
                box_positions.insert(box_pos.wrapping_add_signed(movement));
            }
        } else {
            box_to_move.clear();
            pos_to_check.clear();
        }
    }

    Some(
        box_positions
            .into_iter()
            .map(|pos| 100 * pos.y + pos.x)
            .sum(),
    )
}

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_small() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one_large() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two_large() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9021));
    }
}
