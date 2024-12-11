use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use num::Integer;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    struct Stone {
        value: u64,
        step: u64,
    }

    let mut stones = input
        .split_ascii_whitespace()
        .map(|stone| {
            let value = stone.parse().unwrap();
            Stone { value, step: 0 }
        })
        .collect_vec();

    let mut stone_count = 0;

    while let Some(Stone {
        mut value,
        mut step,
    }) = stones.pop()
    {
        loop {
            if step == 25 {
                stone_count += 1;
                break;
            }

            if value == 0 {
                value = 1;
            } else {
                let n = value.to_string();
                if n.len().is_even() {
                    let (a, b) = n.split_at_checked(n.len() / 2).unwrap();
                    value = a.parse().unwrap();
                    stones.push(Stone {
                        value: b.parse().unwrap(),
                        step: step + 1,
                    });
                } else {
                    value *= 2024;
                }
            }

            step += 1;
        }
    }

    Some(stone_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    const STEP: usize = 75;

    let stones = input
        .split_ascii_whitespace()
        .map(|stone| stone.parse::<u64>().unwrap())
        .collect_vec();

    let chains = {
        let mut jobs = Vec::new();
        let mut chains = HashMap::new();

        jobs.extend(stones.iter().copied());

        while let Some(mut value) = jobs.pop() {
            if chains.contains_key(&value) {
                continue;
            }

            let mut values = Vec::new();

            loop {
                values.push(value);
                let n = value.to_string();
                if n.len().is_even() {
                    let (a, b) = n.split_at_checked(n.len() / 2).unwrap();
                    let a = a.parse().unwrap();
                    let b = b.parse().unwrap();

                    for (i, value) in values.into_iter().rev().enumerate() {
                        chains.insert(
                            value,
                            Chain {
                                length: i + 1,
                                values: [a, b],
                            },
                        );
                    }

                    jobs.push(a);
                    jobs.push(b);

                    break;
                } else {
                    value = if value == 0 { 1 } else { value * 2024 };
                }
            }
        }

        chains
    };

    let values = {
        struct Job {
            value: u64,
            chain: Chain,
            current_step: usize,
        }

        let mut values = HashMap::new();
        let mut jobs = VecDeque::with_capacity(chains.len());

        for (value, chain) in chains {
            for i in 0..chain.length {
                values.insert((value, i), 1);
            }

            jobs.push_back(Job {
                value,
                chain,
                current_step: chain.length,
            });
        }

        while let Some(mut job) = jobs.pop_front() {
            let [a, b] = job.chain.values;

            loop {
                if job.current_step > STEP {
                    break;
                }

                let a = values
                    .get(&(a, job.current_step - job.chain.length))
                    .copied();
                let b = values
                    .get(&(b, job.current_step - job.chain.length))
                    .copied();

                if let (Some(a), Some(b)) = (a, b) {
                    values.insert((job.value, job.current_step), a + b);
                    job.current_step += 1;
                } else {
                    jobs.push_back(job);
                    break;
                }
            }
        }

        values
    };

    let count = stones
        .into_iter()
        .map(|value| values.get(&(value, STEP)).unwrap())
        .sum();

    return Some(count);

    #[derive(Debug, Clone, Copy)]
    struct Chain {
        /// The number of steps after which the chain split in two
        length: usize,
        /// The values that's result from the splitting.
        values: [u64; 2],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}
