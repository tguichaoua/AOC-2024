use itertools::Itertools;

advent_of_code::solution!();

pub fn parse(input: &str) -> impl Iterator<Item: Iterator<Item = u32>> + '_ {
    input
        .lines()
        .map(|line| line.split_whitespace().map(|n| n.parse().unwrap()))
}

fn check_value(cmp: core::cmp::Ordering) -> impl Fn((u32, u32)) -> bool {
    move |(a, b)| a.cmp(&b) == cmp && a.abs_diff(b) <= 3
}

fn is_report_safe(report: impl IntoIterator<Item = u32>) -> bool {
    let mut iter = report.into_iter().tuple_windows();

    let Some((a, b)) = iter.next() else {
        // Empty report is safe, I guess
        return true;
    };

    let cmp = a.cmp(&b);

    if cmp == core::cmp::Ordering::Equal {
        // No change is not safe
        return false;
    }

    let check_value = check_value(cmp);

    check_value((a, b)) && iter.all(check_value)
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse(input);

    let mut are_safe = 0;
    for report in reports {
        if is_report_safe(report) {
            are_safe += 1;
        }
    }

    Some(are_safe)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse(input);

    let mut are_safe = 0;

    for report in reports {
        let report = report.collect_vec();

        // Note: brute force
        for i in 0..report.len() {
            let head = &report[..i];
            let tail = &report[(i + 1)..];

            let report = head.iter().chain(tail).copied();

            if is_report_safe(report) {
                are_safe += 1;
                break;
            }
        }
    }

    Some(are_safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
