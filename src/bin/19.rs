use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!();

fn parse(input: &str) -> (impl Iterator<Item = &[u8]>, impl Iterator<Item = &[u8]>) {
    debug_assert!(input.is_ascii());

    let mut lines = input.lines();

    let towels = lines.next().unwrap().split(", ").map(str::as_bytes);
    let _ = lines.next();
    let designs = lines.map(str::as_bytes);

    (towels, designs)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (towels, designs) = parse(input);

    let towels = towels.collect_vec();

    let mut starts = Vec::new();
    let mut possible_design = 0;

    'next_design: for design in designs {
        starts.clear();
        starts.push(0);

        while let Some(start) = starts.pop() {
            let d = &design[start..];

            for towel in &towels {
                if d.starts_with(towel) {
                    let new_start = start + towel.len();
                    if new_start == design.len() {
                        possible_design += 1;
                        continue 'next_design;
                    } else {
                        starts.push(new_start);
                    }
                }
            }
        }
    }

    Some(possible_design)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (towels, designs) = parse(input);

    let towels = towels.collect_vec();
    let mut memory = HashMap::new();

    let possible_design = designs
        .map(|design| number_of_combination(&mut memory, design, &towels))
        .sum();

    return Some(possible_design);

    fn number_of_combination<'a>(
        memory: &mut HashMap<&'a [u8], u64>,
        design: &'a [u8],
        towels: &[&[u8]],
    ) -> u64 {
        if let Some(count) = memory.get(design).copied() {
            return count;
        }

        let mut count = 0;

        for towel in towels {
            if design.starts_with(towel) {
                if towel.len() == design.len() {
                    count += 1;
                } else {
                    count += number_of_combination(memory, &design[towel.len()..], towels);
                }
            }
        }

        memory.insert(design, count);

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
