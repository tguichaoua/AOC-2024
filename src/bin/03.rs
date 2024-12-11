advent_of_code::solution!();

pub fn part_one(input: &str) -> Option<u32> {
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let total = re
        .captures_iter(input)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            let a: u32 = a.parse().unwrap();
            let b: u32 = b.parse().unwrap();

            a * b
        })
        .sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = regex::Regex::new(r"(?:mul\((\d{1,3}),(\d{1,3})\))|(?:do\(\))|(?:don't\(\))").unwrap();

    let mut enabled = true;

    let total = re
        .captures_iter(input)
        .map(|c| {
            let s = c.get(0).unwrap().as_str();

            if s.starts_with("don't") {
                enabled = false;
                0
            } else if s.starts_with("do") {
                enabled = true;
                0
            } else if enabled {
                let a: u32 = c.get(1).unwrap().as_str().parse().unwrap();
                let b: u32 = c.get(2).unwrap().as_str().parse().unwrap();

                a * b
            } else {
                0
            }
        })
        .sum();

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
