use std::{collections::HashSet, ops::ControlFlow};

use advent_of_code::Dir;
use glam::IVec2 as Pos;

advent_of_code::solution!(6);

struct Input {
    obstacles: HashSet<Pos>,
    start_pos: Pos,
    height: i32,
    width: i32,
}

fn parse(input: &str) -> Input {
    debug_assert!(input.is_ascii());

    let mut obstacles = HashSet::new();
    let mut start_pos = None;

    input.lines().enumerate().for_each(|(y, line)| {
        let y = y.try_into().unwrap();
        line.bytes().enumerate().for_each(|(x, c)| {
            let x = x.try_into().unwrap();
            match c {
                b'#' => {
                    obstacles.insert(Pos::new(x, y));
                }
                b'^' => {
                    debug_assert!(start_pos.is_none());
                    start_pos = Some(Pos::new(x, y));
                }
                _ => {}
            }
        });
    });

    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    debug_assert!(input.lines().all(|l| l.len() == width));

    let start_pos = start_pos.expect("no guard pos");

    Input {
        obstacles,
        start_pos,
        height: height.try_into().unwrap(),
        width: width.try_into().unwrap(),
    }
}

/// Simulates the guard's movement.
///
/// Returns `true` is `visit` returns [`ControlFlow::Break`] and
/// `false` if the guard leaves the map.
fn explore(
    start_pos: Pos,
    width: i32,
    height: i32,
    is_obstacle: impl Fn(Pos) -> bool,
    mut visit: impl FnMut(Pos, Dir) -> ControlFlow<()>,
) -> bool {
    let mut guard_pos = start_pos;
    let mut facing_direction = Dir::Up;

    if visit(guard_pos, facing_direction).is_break() {
        return true;
    }

    loop {
        let dir = facing_direction.as_vec_down_right();

        'walk_foward: loop {
            let next_pos = guard_pos + dir;

            let is_outside =
                !(0..width).contains(&next_pos.x) || !(0..height).contains(&next_pos.y);

            if is_outside {
                return false;
            }

            if is_obstacle(next_pos) {
                facing_direction = facing_direction.rotated_clockwise();
                break 'walk_foward;
            } else {
                guard_pos = next_pos;
                if visit(guard_pos, facing_direction).is_break() {
                    return true;
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let Input {
        obstacles,
        start_pos,
        height,
        width,
    } = parse(input);

    let mut visited = HashSet::new();

    explore(
        start_pos,
        width,
        height,
        |pos| obstacles.contains(&pos),
        |pos, _| {
            visited.insert(pos);
            ControlFlow::Continue(())
        },
    );

    Some(visited.len().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let Input {
        obstacles,
        start_pos,
        height,
        width,
    } = parse(input);

    let mut visited = HashSet::new();
    explore(
        start_pos,
        width,
        height,
        |pos| obstacles.contains(&pos),
        |pos, _| {
            visited.insert(pos);
            ControlFlow::Continue(())
        },
    );

    // We are not allowed to put an obstacle where the guard is.
    visited.remove(&start_pos);

    let mut possibility_to_make_a_loop = 0;

    for new_obstacle in visited {
        let mut states = HashSet::new();

        let is_loop = explore(
            start_pos,
            width,
            height,
            |pos| pos == new_obstacle || obstacles.contains(&pos),
            |pos, facing_direction| match states.insert((pos, facing_direction)) {
                true => ControlFlow::Continue(()),
                false => ControlFlow::Break(()),
            },
        );

        if is_loop {
            possibility_to_make_a_loop += 1;
        }
    }

    Some(possibility_to_make_a_loop)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
