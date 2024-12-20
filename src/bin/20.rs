use std::{collections::HashSet, mem};

use advent_of_code::{four_directions, parse_ascii_map, VecExt};
use glam::UVec2;
use itertools::Itertools;

advent_of_code::solution!();

/* -------------------------------------------------------------------------- */

struct Input {
    start: UVec2,
    end: UVec2,
    route: HashSet<UVec2>,
}

fn parse(input: &str) -> Input {
    let mut start = None;
    let mut end = None;
    let mut route = HashSet::new();

    parse_ascii_map(input).for_each(|(pos, c)| {
        match c {
            b'.' => {
                route.insert(pos);
            }
            b'S' => {
                debug_assert!(start.is_none());
                start = Some(pos);
            }
            b'E' => {
                debug_assert!(end.is_none());
                end = Some(pos);
            }
            b'#' => { /* nothing */ }
            _ => unreachable!(),
        }
    });

    let start = start.unwrap();
    let end = end.unwrap();

    route.insert(start);
    route.insert(end);

    Input { start, end, route }
}

/* -------------------------------------------------------------------------- */

fn solve_one(input: &str, mut f: impl FnMut(usize)) {
    let Input { start, end, route } = parse(input);

    let path = {
        let mut path = Vec::with_capacity(route.len());

        let mut previous: UVec2 = start;
        let mut current = start;
        path.push(start);

        while current != end {
            let mut positions = {
                let route = &route;
                four_directions(current).filter(move |p| route.contains(p) && *p != previous)
            };

            previous = mem::replace(&mut current, positions.next().unwrap());

            // We assumes there is only one path
            debug_assert!(positions.next().is_none());

            path.push(current);
        }

        path
    };

    for (time, pos) in path[..path.len() - 1].iter().copied().enumerate() {
        // up
        {
            if let Some(wall_pos) = pos.up() {
                if let Some(route_pos) = wall_pos.up() {
                    if !route.contains(&wall_pos) {
                        if let Some((target_time, _)) = path[time + 1..]
                            .iter()
                            .copied()
                            .find_position(|&x| x == route_pos)
                        {
                            let saved_time = target_time - 1;
                            f(saved_time);
                        }
                    }
                }
            }
        }

        // left
        {
            if let Some(wall_pos) = pos.left() {
                if let Some(route_pos) = wall_pos.left() {
                    if !route.contains(&wall_pos) {
                        if let Some((target_time, _)) = path[time + 1..]
                            .iter()
                            .copied()
                            .find_position(|&x| x == route_pos)
                        {
                            let saved_time = target_time - 1;

                            f(saved_time);
                        }
                    }
                }
            }
        }

        // down
        {
            let wall_pos = pos.down();
            let route_pos = wall_pos.down();

            if !route.contains(&wall_pos) {
                if let Some((target_time, _)) = path[time + 1..]
                    .iter()
                    .copied()
                    .find_position(|&x| x == route_pos)
                {
                    let saved_time = target_time - 1;

                    f(saved_time);
                }
            }
        }

        // right
        {
            let wall_pos = pos.right();
            let route_pos = wall_pos.right();

            if !route.contains(&wall_pos) {
                if let Some((target_time, _)) = path[time + 1..]
                    .iter()
                    .copied()
                    .find_position(|&x| x == route_pos)
                {
                    let saved_time = target_time - 1;

                    f(saved_time);
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut cheats = 0;

    solve_one(input, |saved_time| {
        if saved_time >= 100 {
            cheats += 1
        }
    });

    Some(cheats)
}

/* -------------------------------------------------------------------------- */

fn solve_two(input: &str, mut f: impl FnMut(usize)) {
    let Input { start, end, route } = parse(input);

    let path = {
        let mut path = Vec::with_capacity(route.len());

        let mut previous: UVec2 = start;
        let mut current = start;
        path.push(start);

        while current != end {
            let mut positions = {
                let route = &route;
                four_directions(current).filter(move |p| route.contains(p) && *p != previous)
            };

            previous = mem::replace(&mut current, positions.next().unwrap());

            // We assumes there is only one path
            debug_assert!(positions.next().is_none());

            path.push(current);
        }

        path
    };

    for (time, a) in path.iter().copied().enumerate() {
        for (time2, b) in path[time + 1..].iter().copied().enumerate() {
            let dist = a.x.abs_diff(b.x) + a.y.abs_diff(b.y);
            let dist: usize = dist.try_into().unwrap();

            if dist <= 20 {
                let time_saved = time2 + 1 - dist;
                f(time_saved);
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cheats = 0;

    solve_two(input, |saved_time| {
        if saved_time >= 100 {
            cheats += 1
        }
    });

    Some(cheats)
}

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_part_one() {
        let mut cheats: HashMap<_, usize> = HashMap::new();

        solve_one(
            &advent_of_code::template::read_file("examples", DAY),
            |saved_time| {
                *cheats.entry(saved_time).or_default() += 1;
            },
        );

        let mut cheats = cheats.into_iter().collect_vec();
        cheats.sort_unstable_by_key(|(saved_time, _)| *saved_time);

        assert_eq!(
            cheats,
            [
                (2, 14),
                (4, 14),
                (6, 2),
                (8, 4),
                (10, 2),
                (12, 3),
                (20, 1),
                (36, 1),
                (38, 1),
                (40, 1),
                (64, 1)
            ]
        )
    }

    #[test]
    fn test_part_two() {
        let mut cheats: HashMap<_, usize> = HashMap::new();

        solve_two(
            &advent_of_code::template::read_file("examples", DAY),
            |saved_time| {
                if saved_time >= 50 {
                    *cheats.entry(saved_time).or_default() += 1;
                }
            },
        );

        let mut cheats = cheats.into_iter().collect_vec();
        cheats.sort_unstable_by_key(|(saved_time, _)| *saved_time);

        assert_eq!(
            cheats,
            [
                (50, 32),
                (52, 31),
                (54, 29),
                (56, 39),
                (58, 25),
                (60, 23),
                (62, 20),
                (64, 19),
                (66, 12),
                (68, 14),
                (70, 12),
                (72, 22),
                (74, 4),
                (76, 3),
            ]
        )
    }
}
