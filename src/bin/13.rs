use advent_of_code::int_linear_solve2;
use glam::{i64vec2, ivec2, I64Vec2, IVec2};
use itertools::Itertools;

advent_of_code::solution!();

#[derive(Debug)]
struct Machine {
    button_a: IVec2,
    button_b: IVec2,
    prize: IVec2,
}

fn parse(input: &str) -> impl Iterator<Item = Machine> + '_ {
    input.lines().batching(|lines| {
        let a = lines.next()?;
        let b = lines.next().unwrap();
        let prize = lines.next().unwrap();
        // The empty line between inputs
        let _ = lines.next();

        const BUTTON_A_PREFIX: &str = "Button A: X+";
        const BUTTON_B_PREFIX: &str = "Button B: X+";
        const BUTTON_IN_BETWEEN: &str = ", Y+";
        const PRIZE_PREFIX: &str = "Prize: X=";
        const PRIZE_IN_BETWEEN: &str = ", Y=";

        let (ax, ay) = a
            .strip_prefix(BUTTON_A_PREFIX)
            .unwrap()
            .split_once(BUTTON_IN_BETWEEN)
            .unwrap();
        let (bx, by) = b
            .strip_prefix(BUTTON_B_PREFIX)
            .unwrap()
            .split_once(BUTTON_IN_BETWEEN)
            .unwrap();
        let (px, py) = prize
            .strip_prefix(PRIZE_PREFIX)
            .unwrap()
            .split_once(PRIZE_IN_BETWEEN)
            .unwrap();

        let button_a = ivec2(ax.parse().unwrap(), ay.parse().unwrap());
        let button_b = ivec2(bx.parse().unwrap(), by.parse().unwrap());
        let prize = ivec2(px.parse().unwrap(), py.parse().unwrap());

        Some(Machine {
            button_a,
            button_b,
            prize,
        })
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let machines = parse(input);

    let mut needed_tokens = 0;

    for machine in machines {
        let a = machine.button_a.x;
        let b = machine.button_b.x;
        let c = machine.button_a.y;
        let d = machine.button_b.y;

        if let Some((a_count, b_count)) =
            int_linear_solve2(machine.prize.x, machine.prize.y, a, b, c, d)
                .and_then(|(a, b)| Some((u32::try_from(a).ok()?, u32::try_from(b).ok()?)))
        {
            needed_tokens += 3 * a_count + b_count;
        }
    }

    Some(needed_tokens)
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = parse(input);

    let mut needed_tokens = 0;

    for machine in machines {
        let a: i64 = machine.button_a.x.into();
        let b: i64 = machine.button_b.x.into();
        let c: i64 = machine.button_a.y.into();
        let d: i64 = machine.button_b.y.into();

        let prize = I64Vec2::from(machine.prize) + i64vec2(10_000_000_000_000, 10_000_000_000_000);

        if let Some((a_count, b_count)) = int_linear_solve2(prize.x, prize.y, a, b, c, d)
            .and_then(|(a, b)| Some((u64::try_from(a).ok()?, u64::try_from(b).ok()?)))
        {
            needed_tokens += 3 * a_count + b_count;
        }
    }

    Some(needed_tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }
}
