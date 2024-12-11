advent_of_code::solution!();

fn parse(input: &str) -> impl Iterator<Item = (u32, u32)> + '_ {
    input.lines().map(|line| {
        let (l, r) = line.split_once("   ").unwrap();
        let l: u32 = l.parse().unwrap();
        let r: u32 = r.parse().unwrap();

        (l, r)
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut a, mut b) = parse(input).collect::<(Vec<_>, Vec<_>)>();

    a.sort_unstable();
    b.sort_unstable();

    let total_dist = a.into_iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum();

    Some(total_dist)
}

pub fn part_two(input: &str) -> Option<u32> {
    let values = parse(input);

    let mut left = Vec::new();
    let mut right = std::collections::HashMap::new();

    for (l, r) in values {
        left.push(l);
        *right.entry(r).or_insert(0) += 1;
    }

    let score = left
        .iter()
        .map(|l| l * right.get(l).copied().unwrap_or(0))
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
