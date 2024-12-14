use glam::{ivec2, IVec2};
use itertools::Itertools;

advent_of_code::solution!();

struct Robot {
    pos: IVec2,
    vel: IVec2,
}

fn parse(input: &str) -> impl Iterator<Item = Robot> + '_ {
    input.lines().map(|line| {
        let (p, v) = line.split_once(' ').unwrap();

        let p = p.strip_prefix("p=").unwrap();
        let v = v.strip_prefix("v=").unwrap();

        let (px, py) = p.split_once(',').unwrap();
        let (vx, vy) = v.split_once(',').unwrap();

        let pos = ivec2(px.parse().unwrap(), py.parse().unwrap());
        let vel = ivec2(vx.parse().unwrap(), vy.parse().unwrap());

        Robot { pos, vel }
    })
}

fn solve_1(input: &str, area_size: IVec2) -> u32 {
    const SECONDS: i32 = 100;

    let robots = parse(input);
    let mut robot_count_in_quadrants = [0; 4];

    let half = area_size / 2;

    robots.for_each(|Robot { pos, vel }| {
        let pos = pos + vel * SECONDS;

        let mut x = pos.x % area_size.x;
        if x < 0 {
            x += area_size.x;
        }

        let mut y = pos.y % area_size.y;
        if y < 0 {
            y += area_size.y;
        }

        if x == half.x || y == half.y {
            return;
        }

        let idx = match (x < half.x, y < half.y) {
            (true, true) => 0,
            (true, false) => 1,
            (false, true) => 2,
            (false, false) => 3,
        };

        robot_count_in_quadrants[idx] += 1;
    });

    robot_count_in_quadrants
        .into_iter()
        .reduce(|a, b| a * b)
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve_1(input, ivec2(101, 103)))
}

pub fn part_two(input: &str) -> Option<u32> {
    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;

    let mut robots = parse(input).collect_vec();
    let robot_count = i32::try_from(robots.len()).unwrap();

    for i in 1.. {
        robots.iter_mut().for_each(|Robot { pos, vel }| {
            let mut x = (pos.x + vel.x) % WIDTH;
            if x < 0 {
                x += WIDTH;
            }

            let mut y = (pos.y + vel.y) % HEIGHT;
            if y < 0 {
                y += HEIGHT;
            }

            *pos = ivec2(x, y);
        });

        let center = robots
            .iter()
            .map(|robot| robot.pos)
            .fold(IVec2::ZERO, |a, b| a + b)
            / robot_count;

        // NOTE: values found empirically
        if center.x >= 55 && center.y >= 55 {
            return Some(i);
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let result = solve_1(input, ivec2(11, 7));
        assert_eq!(result, 12);
    }
}
