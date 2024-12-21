#![allow(dead_code, unused_variables)]

use advent_of_code::{Dir, Horizontal, Vertical};
use glam::{uvec2, UVec2};

advent_of_code::solution!();

/* -------------------------------------------------------------------------- */

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    movement: Movement,
    press_a: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Movement {
    NoMove,
    OneDirection { dir: Dir, distance: u32 },
    TwoDirection { possibilities: Vec<Vec<Dir>> },
}

fn parse_numerical_sequence(input: &[u8]) -> impl Iterator<Item = NumericButton> + '_ {
    input.iter().copied().map(|b| match b {
        b'A' => NumericButton::A,
        b'0' => NumericButton::N0,
        b'1' => NumericButton::N1,
        b'2' => NumericButton::N2,
        b'3' => NumericButton::N3,
        b'4' => NumericButton::N4,
        b'5' => NumericButton::N5,
        b'6' => NumericButton::N6,
        b'7' => NumericButton::N7,
        b'8' => NumericButton::N8,
        b'9' => NumericButton::N9,
        _ => unreachable!(),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NumericButton {
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    A,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DirectionalButton {
    Direction(Dir),
    A,
}

impl NumericButton {
    fn col(self) -> u32 {
        match self {
            NumericButton::N7 | NumericButton::N4 | NumericButton::N1 => 0,
            NumericButton::N0 | NumericButton::N2 | NumericButton::N5 | NumericButton::N8 => 1,
            NumericButton::A | NumericButton::N3 | NumericButton::N6 | NumericButton::N9 => 2,
        }
    }

    fn row(self) -> u32 {
        match self {
            NumericButton::N7 | NumericButton::N8 | NumericButton::N9 => 0,
            NumericButton::N4 | NumericButton::N5 | NumericButton::N6 => 1,
            NumericButton::N1 | NumericButton::N2 | NumericButton::N3 => 2,
            NumericButton::N0 | NumericButton::A => 3,
        }
    }

    fn pos(self) -> UVec2 {
        uvec2(self.col(), self.row())
    }
}

impl DirectionalButton {
    fn col(self) -> u32 {
        match self {
            DirectionalButton::Direction(Dir::Left) => 0,
            DirectionalButton::Direction(Dir::Up) | DirectionalButton::Direction(Dir::Down) => 1,
            DirectionalButton::A | DirectionalButton::Direction(Dir::Right) => 2,
        }
    }

    fn row(self) -> u32 {
        match self {
            DirectionalButton::Direction(Dir::Up) | DirectionalButton::A => 0,
            DirectionalButton::Direction(Dir::Left)
            | DirectionalButton::Direction(Dir::Down)
            | DirectionalButton::Direction(Dir::Right) => 1,
        }
    }

    fn pos(self) -> UVec2 {
        uvec2(self.col(), self.row())
    }
}

fn compute_sequence_for_numerical_keyboard(
    sequence: impl IntoIterator<Item = NumericButton>,
) -> Vec<Node> {
    let mut nodes = Vec::new();
    let mut current_position = NumericButton::A.pos();

    for goal in sequence {
        let pos = goal.pos();

        let horizontal = match current_position.x.cmp(&pos.x) {
            std::cmp::Ordering::Less => Some((Horizontal::Right, pos.x - current_position.x)),
            std::cmp::Ordering::Equal => None,
            std::cmp::Ordering::Greater => Some((Horizontal::Left, current_position.x - pos.x)),
        };

        let vertical = match current_position.y.cmp(&pos.y) {
            std::cmp::Ordering::Less => Some((Vertical::Down, pos.y - current_position.y)),
            std::cmp::Ordering::Equal => None,
            std::cmp::Ordering::Greater => Some((Vertical::Up, current_position.y - pos.y)),
        };

        let movement = match (horizontal, vertical) {
            (None, None) => Movement::NoMove,
            (None, Some((dir, distance))) => Movement::OneDirection {
                dir: dir.into(),
                distance,
            },
            (Some((dir, distance)), None) => Movement::OneDirection {
                dir: dir.into(),
                distance,
            },
            (Some((horizontal, h_distance)), Some((vertical, v_distance))) => {
                todo!();
            }
        };

        nodes.push(Node {
            movement,
            press_a: 1,
        });

        current_position = pos;
    }

    nodes
}

fn compute_sequence_for_directional_keyboard(
    sequence: impl IntoIterator<Item = Node>,
) -> Vec<Node> {
    let nodes = Vec::new();
    let current_position = DirectionalButton::A.pos();

    for Node { movement, press_a } in sequence {
        match movement {
            Movement::NoMove => { /* nothing  */ }
            Movement::OneDirection { dir, distance } => {
                // TODO: move to dir.
                // TODO: press A for `distance`
                let goal = DirectionalButton::Direction(dir);

                todo!();
            }
            Movement::TwoDirection { possibilities } => todo!(),
        }

        // TODO: move to A.
        // TODO: press A for `press_a`.
        let goal = DirectionalButton::A;
    }

    nodes
}

fn compute_shortest_sequence(
    numerical_sequence: impl IntoIterator<Item = NumericButton>,
) -> Vec<DirectionalButton> {
    // let first_keyboard = compute_sequence_for_numerical_keyboard(numerical_sequence);
    // let seconds_keyboard = compute_sequence_for_directional_keyboard(dbg!(first_keyboard));
    // let third_keyboard = compute_sequence_for_directional_keyboard(dbg!(seconds_keyboard));

    // dbg!(third_keyboard)

    todo!();
}

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_shortest_sequence() {
        let tests = [
            (
                b"029A",
                &b"<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"[..],
            ),
            (
                b"980A",
                &b"<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A"[..],
            ),
            (
                b"179A",
                &b"<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"[..],
            ),
            (
                b"456A",
                &b"<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A"[..],
            ),
            (
                b"379A",
                &b"<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"[..],
            ),
        ];

        for (input, output) in tests {
            assert_eq!(
                compute_shortest_sequence(parse_numerical_sequence(input)).len(),
                output.len(),
                "input: '{}'",
                core::str::from_utf8(input).unwrap()
            );
        }
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
