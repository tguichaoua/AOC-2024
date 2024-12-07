advent_of_code::solution!(7);

fn parse(input: &str) -> impl Iterator<Item = (u64, impl Iterator<Item = u64> + '_)> {
    input.lines().map(|line| {
        let (result, values) = line.split_once(':').unwrap();

        let result = result.parse().unwrap();
        let values = values.split_whitespace().map(|n| n.parse().unwrap());

        (result, values)
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let tests = parse(input);

    let total = tests.fold(0, |acc, (result, mut values)| {
        let mut totals = Vec::new();
        totals.push(values.next().unwrap());

        let mut current_totals = Vec::new();

        for value in values {
            core::mem::swap(&mut totals, &mut current_totals);
            for x in current_totals.drain(..) {
                let plus = x + value;
                let mul = x * value;

                if plus <= result {
                    totals.push(plus);
                }

                if mul <= result {
                    totals.push(mul);
                }
            }

            if totals.is_empty() {
                break;
            }
        }

        if totals.iter().any(|&x| x == result) {
            acc + result
        } else {
            acc
        }
    });

    Some(total)
}

fn concat(lhs: u64, rhs: u64) -> u64 {
    let pow_10 = rhs.ilog10();

    lhs * 10_u64.pow(pow_10 + 1) + rhs
}

pub fn part_two(input: &str) -> Option<u64> {
    let tests = parse(input);

    let total = tests.fold(0, |acc, (result, mut values)| {
        let mut totals = Vec::new();
        totals.push(values.next().unwrap());

        let mut current_totals = Vec::new();

        for value in values {
            core::mem::swap(&mut totals, &mut current_totals);
            for x in current_totals.drain(..) {
                let plus = x + value;
                let mul = x * value;
                let concat = concat(x, value);

                if plus <= result {
                    totals.push(plus);
                }

                if mul <= result {
                    totals.push(mul);
                }

                if concat <= result {
                    totals.push(concat);
                }
            }

            if totals.is_empty() {
                break;
            }
        }

        if totals.iter().any(|&x| x == result) {
            acc + result
        } else {
            acc
        }
    });

    Some(total)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }

    #[test]
    fn test_concat() {
        #![allow(clippy::inconsistent_digit_grouping)]

        assert_eq!(concat(64, 132), 64_132);
        assert_eq!(concat(72, 9), 72_9);
        assert_eq!(concat(10001, 9757575), 10001_9757575);
    }
}
