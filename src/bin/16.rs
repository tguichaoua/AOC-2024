use std::collections::{BTreeSet, HashMap, HashSet};

use advent_of_code::{cmp_uvec2, parse_ascii_map, Dir};
use glam::UVec2;

advent_of_code::solution!();

struct Input {
    start_position: UVec2,
    end_position: UVec2,
    walls: HashSet<UVec2>,
}

fn parse(input: &str) -> Input {
    let mut start_position = None;
    let mut end_position = None;
    let mut walls = HashSet::new();

    parse_ascii_map(input).for_each(|(pos, c)| {
        let pos = pos.try_into().unwrap();

        match c {
            b'#' => {
                walls.insert(pos);
            }
            b'S' => {
                debug_assert!(start_position.is_none());
                start_position = Some(pos);
            }
            b'E' => {
                debug_assert!(end_position.is_none());
                end_position = Some(pos);
            }
            b'.' => { /* nothing */ }
            _ => unreachable!(),
        }
    });

    Input {
        start_position: start_position.unwrap(),
        end_position: end_position.unwrap(),
        walls,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let Input {
        start_position,
        end_position,
        walls,
    } = parse(input);

    let mut explorer = Explorer::new();

    explorer.new_state(
        Pos {
            position: start_position,
            facing: Dir::East,
        },
        0,
    );

    while let Some(state) = explorer.states.pop_first() {
        // move forward
        {
            let new_pos = state.pos.move_forward();
            let new_score = state.score + 1;

            if new_pos.position == end_position {
                explorer.min_score = explorer.min_score.min(new_score);
                // We reach the end other moves are useless
                continue;
            } else if !walls.contains(&new_pos.position) {
                explorer.new_state(new_pos, new_score);
            }
        }

        // rotate clockwise
        {
            let new_pos = state.pos.rotate_clockwise();
            let new_score = state.score + 1000;
            explorer.new_state(new_pos, new_score);
        }

        // rotate counterclockwise
        {
            let new_pos = state.pos.rotate_counterclockwise();
            let new_score = state.score + 1000;
            explorer.new_state(new_pos, new_score);
        }
    }

    return Some(explorer.min_score);

    struct Explorer {
        visited: HashMap<Pos, u32>,
        states: BTreeSet<State>,
        min_score: u32,
    }

    impl Explorer {
        fn new() -> Self {
            Self {
                visited: HashMap::new(),
                states: BTreeSet::new(),
                min_score: u32::MAX,
            }
        }

        fn new_state(&mut self, pos: Pos, score: u32) {
            let should_add_new_state = if self.min_score <= score {
                false
            } else if let Some(&visited_score) = self.visited.get(&pos) {
                score < visited_score
            } else {
                true
            };

            if should_add_new_state {
                self.states.insert(State { pos, score });
                self.visited.insert(pos, score);
            }
        }
    }

    #[derive(Clone, Copy, Hash, PartialEq, Eq)]
    struct Pos {
        position: UVec2,
        facing: Dir,
    }

    impl Pos {
        fn move_forward(self) -> Self {
            let new_pos = self
                .position
                .wrapping_add_signed(self.facing.as_vec_down_right());
            Self {
                position: new_pos,
                facing: self.facing,
            }
        }

        fn rotate_clockwise(self) -> Self {
            Self {
                position: self.position,
                facing: self.facing.rotated_clockwise(),
            }
        }

        fn rotate_counterclockwise(self) -> Self {
            Self {
                position: self.position,
                facing: self.facing.rotated_anti_clockwise(),
            }
        }
    }

    #[derive(PartialEq, Eq)]
    struct State {
        score: u32,
        pos: Pos,
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.score
                .cmp(&other.score)
                .then(cmp_uvec2(&self.pos.position, &other.pos.position))
                .then(self.pos.facing.cmp(&other.pos.facing))
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let Input {
        start_position,
        end_position,
        walls,
    } = parse(input);

    let mut explorer = Explorer::new();

    explorer.new_state(
        Pos {
            position: start_position,
            facing: Dir::East,
        },
        0,
        HashSet::new(),
    );

    // The best score (lowest) is at the end of `explorer.states`
    while let Some(state) = explorer.states.pop() {
        // move forward
        {
            let new_pos = state.pos.move_forward();
            let new_score = state.score + 1;

            if new_pos.position == end_position {
                match new_score.cmp(&explorer.min_score) {
                    std::cmp::Ordering::Less => {
                        explorer.part_of_min_path.clear();
                        explorer.part_of_min_path.extend(state.visited);
                        explorer.min_score = new_score;
                    }
                    std::cmp::Ordering::Equal => {
                        explorer.part_of_min_path.extend(state.visited);
                    }
                    std::cmp::Ordering::Greater => {
                        // nothing: the current path has a score bigger than the current min
                    }
                }

                // We reach the end other moves are useless
                continue;
            } else if !walls.contains(&new_pos.position) {
                explorer.new_state(new_pos, new_score, state.visited.clone());
            }
        }

        // rotate clockwise
        {
            let new_pos = state.pos.rotate_clockwise();
            let new_score = state.score + 1000;
            explorer.new_state(new_pos, new_score, state.visited.clone());
        }

        // rotate counterclockwise
        {
            let new_pos = state.pos.rotate_counterclockwise();
            let new_score = state.score + 1000;
            explorer.new_state(new_pos, new_score, state.visited);
        }
    }

    let part_of_min_path_count: u32 = explorer.part_of_min_path.len().try_into().unwrap();

    // we add one for the end tile
    return Some(part_of_min_path_count + 1);

    struct Explorer {
        visited: HashMap<Pos, u32>,

        /// States to explore order by score.
        /// The last item has the lowest score.
        states: Vec<State>,

        min_score: u32,
        part_of_min_path: HashSet<UVec2>,
    }

    impl Explorer {
        fn new() -> Self {
            Self {
                visited: HashMap::new(),
                states: Vec::new(),
                min_score: u32::MAX,
                part_of_min_path: HashSet::new(),
            }
        }

        fn new_state(&mut self, pos: Pos, score: u32, mut old_visited: HashSet<UVec2>) {
            let should_add_new_state = if self.min_score < score {
                false
            } else if let Some(&visited_score) = self.visited.get(&pos) {
                score <= visited_score
            } else {
                true
            };

            if should_add_new_state {
                old_visited.insert(pos.position);
                self.visited.insert(pos, score);
                let insert_at = match self
                    .states
                    .binary_search_by(|item| item.score.cmp(&score).reverse())
                {
                    Ok(index) | Err(index) => index,
                };
                self.states.insert(
                    insert_at,
                    State {
                        score,
                        pos,
                        visited: old_visited,
                    },
                );
            }
        }
    }

    #[derive(Clone, Copy, Hash, PartialEq, Eq)]
    struct Pos {
        position: UVec2,
        facing: Dir,
    }

    impl Pos {
        fn move_forward(self) -> Self {
            let new_pos = self
                .position
                .wrapping_add_signed(self.facing.as_vec_down_right());
            Self {
                position: new_pos,
                facing: self.facing,
            }
        }

        fn rotate_clockwise(self) -> Self {
            Self {
                position: self.position,
                facing: self.facing.rotated_clockwise(),
            }
        }

        fn rotate_counterclockwise(self) -> Self {
            Self {
                position: self.position,
                facing: self.facing.rotated_anti_clockwise(),
            }
        }
    }

    struct State {
        score: u32,
        pos: Pos,
        visited: HashSet<UVec2>,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_first() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_second() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }
    #[test]
    fn test_part_two_first() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part_two_second() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(64));
    }
}
