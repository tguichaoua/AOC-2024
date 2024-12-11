use std::collections::HashSet;

use advent_of_code::ascii_array_2d_with;

advent_of_code::solution!();

pub fn part_one(input: &str) -> Option<u32> {
    let map = ascii_array_2d_with(input, |n| {
        debug_assert!(n.is_ascii_digit());
        n - b'0'
    });

    let trailheads = map
        .iter_with_index()
        .filter(|(_, &value)| value == 0)
        .map(|(pos, _)| pos);

    let mut trailheads_score = 0;

    let mut goals = HashSet::new();
    let mut to_explore = Vec::new();

    for trailhead in trailheads {
        goals.clear();
        to_explore.clear();

        to_explore.push((trailhead, 0));

        while let Some(((r, c), value)) = to_explore.pop() {
            if value == 9 {
                goals.insert((r, c));
                continue;
            }

            let target = value + 1;

            // Up
            if let Some(r) = r.checked_sub(1) {
                if map.get(r, c).copied().unwrap() == target {
                    to_explore.push(((r, c), target));
                }
            }

            // Down
            {
                let r = r + 1;
                if map.get(r, c).copied().is_some_and(|x| x == target) {
                    to_explore.push(((r, c), target));
                }
            }

            // Left
            if let Some(c) = c.checked_sub(1) {
                if map.get(r, c).copied().unwrap() == target {
                    to_explore.push(((r, c), target));
                }
            }

            // Right
            {
                let c = c + 1;
                if map.get(r, c).copied().is_some_and(|x| x == target) {
                    to_explore.push(((r, c), target));
                }
            }
        }

        trailheads_score += goals.len();
    }

    Some(trailheads_score.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = ascii_array_2d_with(input, |n| {
        debug_assert!(n.is_ascii_digit());
        n - b'0'
    });

    let mut to_explore = Vec::new();
    for (pos, &value) in map.iter_with_index() {
        if value == 0 {
            to_explore.push((pos, 0));
        }
    }

    let mut trailheads_score = 0;

    while let Some(((r, c), value)) = to_explore.pop() {
        if value == 9 {
            trailheads_score += 1;
            continue;
        }

        let target = value + 1;

        // Up
        if let Some(r) = r.checked_sub(1) {
            if map.get(r, c).copied().unwrap() == target {
                to_explore.push(((r, c), target));
            }
        }

        // Down
        {
            let r = r + 1;
            if map.get(r, c).copied().is_some_and(|x| x == target) {
                to_explore.push(((r, c), target));
            }
        }

        // Left
        if let Some(c) = c.checked_sub(1) {
            if map.get(r, c).copied().unwrap() == target {
                to_explore.push(((r, c), target));
            }
        }

        // Right
        {
            let c = c + 1;
            if map.get(r, c).copied().is_some_and(|x| x == target) {
                to_explore.push(((r, c), target));
            }
        }
    }

    Some(trailheads_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
