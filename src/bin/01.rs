use advent_of_code::parse_tuple;
use itertools::Itertools;

advent_of_code::solution!(1);

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(parse_tuple::<(u32, u32)>)
        .try_collect()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut a, mut b) = parse(input);

    a.sort_unstable();
    b.sort_unstable();

    let total_dist = a.into_iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum();

    Some(total_dist)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (a, b) = parse(input);

    struct Count {
        in_a: u32,
        in_b: u32,
    }

    let mut counts = std::collections::HashMap::new();

    for n in a {
        match counts.entry(n) {
            std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                let entry: &mut Count = occupied_entry.get_mut();
                entry.in_a += 1;
            }
            std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                let in_b: u32 = b.iter().filter(|&&b| b == n).count().try_into().unwrap();
                vacant_entry.insert(Count { in_a: 1, in_b });
            }
        }
    }

    let score = counts
        .into_iter()
        .map(|(n, Count { in_a, in_b })| n * in_a * in_b)
        .sum();

    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
