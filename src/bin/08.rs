use std::collections::HashSet;

use advent_of_code::contains_point;
use glam::IVec2 as Pos;
use itertools::Itertools;

advent_of_code::solution!(8);

fn parse(input: &str) -> (glam::IVec2, impl Iterator<Item = (Pos, u8)> + Clone + '_) {
    debug_assert!(input.is_ascii());

    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    debug_assert!(input.lines().all(|line| line.len() == width));

    let antennas = input.lines().enumerate().flat_map(|(y, line)| {
        line.bytes()
            .enumerate()
            .filter(|&(_, freq)| (freq != b'.'))
            .map(move |(x, freq)| (Pos::new(x.try_into().unwrap(), y.try_into().unwrap()), freq))
    });

    (
        glam::ivec2(width.try_into().unwrap(), height.try_into().unwrap()),
        antennas,
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map_size, antennas) = parse(input);

    let antennas_couples = antennas
        .tuple_combinations()
        .filter(|((_, freq_a), (_, freq_b))| freq_a == freq_b)
        .map(|((a, _), (b, _))| (a, b));

    let anti_nodes = antennas_couples
        .flat_map(|(a, b)| [2 * b - a, 2 * a - b])
        .filter(|&pos| contains_point(map_size, pos))
        .collect::<HashSet<_>>();

    Some(anti_nodes.len().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map_size, antennas) = parse(input);

    let antennas_couples = antennas
        .tuple_combinations()
        .filter(|((_, freq_a), (_, freq_b))| freq_a == freq_b)
        .map(|((a, _), (b, _))| (a, b));

    let mut anti_nodes = HashSet::new();

    for (a, b) in antennas_couples {
        let v = a - b;

        anti_nodes.insert(a);
        anti_nodes.insert(b);

        for n in 0.. {
            let u = a + v * n;

            if contains_point(map_size, u) {
                anti_nodes.insert(u);
            } else {
                break;
            }
        }

        for n in 0.. {
            let u = b - v * n;

            if contains_point(map_size, u) {
                anti_nodes.insert(u);
            } else {
                break;
            }
        }
    }

    Some(anti_nodes.len().try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
