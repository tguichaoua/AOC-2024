use itertools::Itertools;

advent_of_code::solution!();

pub fn parse(
    input: &str,
) -> (
    impl Iterator<Item = (u32, u32)> + '_,
    impl Iterator<Item: Iterator<Item = u32>> + '_,
) {
    let (ordering_rules, updates) = input.split_once("\n\n").unwrap();

    let ordering_rules = ordering_rules.lines().map(|line| {
        let (before, after) = line.split_once("|").unwrap();
        (
            before.parse::<u32>().unwrap(),
            after.parse::<u32>().unwrap(),
        )
    });

    let updates = updates
        .lines()
        .map(|line| line.split(",").map(|x| x.parse::<u32>().unwrap()));

    (ordering_rules, updates)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (ordering_rules, updates) = parse(input);

    let ordering_rules = ordering_rules.into_group_map();

    let mut total = 0;

    'update: for update in updates {
        let values = update.collect_vec();

        for i in 0..values.len() {
            if let Some(should_be_after) = ordering_rules.get(&values[i]) {
                for before in &values[..i] {
                    if should_be_after.contains(before) {
                        // the update is not ordered correctly.
                        continue 'update;
                    }
                }
            }
        }

        // the update is correctly ordered.
        let middle_value = values[values.len() / 2];
        total += middle_value;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (ordering_rules, updates) = parse(input);

    let ordering_rules = ordering_rules.into_group_map();

    let mut total = 0;

    for update in updates {
        let mut values = update.collect_vec();

        let mut sorted = true;

        'check_order: for i in 0..values.len() {
            if let Some(should_be_after) = ordering_rules.get(&values[i]) {
                for before in &values[..i] {
                    if should_be_after.contains(before) {
                        // the update is not ordered correctly.
                        sorted = false;
                        break 'check_order;
                    }
                }
            }
        }

        if sorted {
            // the update is correctly ordered.
            continue;
        }

        values.sort_unstable_by(|a, b| {
            if let Some(should_be_after) = ordering_rules.get(a) {
                if should_be_after.contains(b) {
                    return core::cmp::Ordering::Less;
                }
            }
            if let Some(should_be_after) = ordering_rules.get(b) {
                if should_be_after.contains(a) {
                    return core::cmp::Ordering::Greater;
                }
            }
            core::cmp::Ordering::Equal
        });

        let middle_value = values[values.len() / 2];
        total += middle_value;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
