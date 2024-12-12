use std::{
    collections::{HashMap, HashSet},
    mem,
};

use advent_of_code::{ascii_array_2d, Dir};
use itertools::Itertools;

advent_of_code::solution!();

struct Group {
    positions: HashSet<(usize, usize)>,
}

fn get_groups(input: &str) -> impl Iterator<Item = Group> {
    let map = ascii_array_2d(input);

    let mut groups_builder = GroupsBuilder::new();

    for ((row, column), &plant) in map.iter_with_index() {
        let pos = (row, column);

        // Check left
        let left_group_idx = if let Some(column) = column.checked_sub(1) {
            let left_pos = (row, column);
            let left = map.get(row, column).copied().unwrap();

            if left == plant {
                Some(groups_builder.insert_pos_in(left_pos, pos))
            } else {
                None
            }
        } else {
            None
        };

        // Check up
        let has_join_the_up_group = if let Some(row) = row.checked_sub(1) {
            let up_pos = (row, column);
            let up = map.get(row, column).copied().unwrap();

            if up == plant {
                if let Some(left_group_idx) = left_group_idx {
                    // plant has join the left group and may also join the up group, we can marge them !

                    let (_, up_group_idx) = groups_builder.get_group_for_mut(up_pos).unwrap();

                    if up_group_idx != left_group_idx {
                        // The two group are different, merge them
                        groups_builder.merge_groups(left_group_idx, up_group_idx);
                    }
                    {
                        // This is the same group and we already append `pos`, so do nothing.
                    }
                } else {
                    groups_builder.insert_pos_in(up_pos, pos);
                }

                true
            } else {
                false
            }
        } else {
            false
        };

        if left_group_idx.is_none() && !has_join_the_up_group {
            groups_builder.new_group_with(pos);
        }
    }

    return groups_builder.into_groups();

    struct GroupsBuilder {
        groups: Vec<Slot>,
        pos_to_group_index: HashMap<(usize, usize), usize>,
    }

    enum Slot {
        Group(Group),
        Merged { index: usize },
    }

    impl GroupsBuilder {
        pub fn new() -> Self {
            Self {
                groups: Vec::new(),
                pos_to_group_index: HashMap::new(),
            }
        }

        /// Insert `pos_to_insert` in the group of `group_pos` and return the index of the group.
        pub fn insert_pos_in(
            &mut self,
            group_pos: (usize, usize),
            pos_to_insert: (usize, usize),
        ) -> usize {
            let (group, index) = self.get_group_for_mut(group_pos).unwrap();
            group.positions.insert(pos_to_insert);
            self.pos_to_group_index.insert(pos_to_insert, index);
            index
        }

        pub fn get_group_for_mut(&mut self, pos: (usize, usize)) -> Option<(&mut Group, usize)> {
            let mut idx = self.pos_to_group_index.get(&pos).copied()?;

            // Yes, this is ugly by it is also a limitation of the borrow checker :/
            let idx = loop {
                match self.groups.get(idx).unwrap() {
                    Slot::Group(_) => break idx,
                    Slot::Merged { index } => idx = *index,
                }
            };

            Some(match self.groups.get_mut(idx).unwrap() {
                Slot::Group(group) => (group, idx),
                Slot::Merged { .. } => unreachable!(),
            })
        }

        pub fn new_group_with(&mut self, pos: (usize, usize)) {
            let mut positions = HashSet::new();
            positions.insert(pos);

            let index = self.groups.len();
            self.groups.push(Slot::Group(Group { positions }));
            self.pos_to_group_index.insert(pos, index);
        }

        pub fn merge_groups(&mut self, a: usize, b: usize) {
            let a = match mem::replace(self.groups.get_mut(a).unwrap(), Slot::Merged { index: b }) {
                Slot::Group(group) => group,
                Slot::Merged { .. } => panic!("group already merged"),
            };

            let b = match self.groups.get_mut(b).unwrap() {
                Slot::Group(group) => group,
                Slot::Merged { .. } => panic!("group already merged"),
            };

            b.positions.extend(a.positions);
        }

        pub fn into_groups(self) -> impl Iterator<Item = Group> {
            self.groups.into_iter().flat_map(|slot| match slot {
                Slot::Group(group) => Some(group),
                Slot::Merged { .. } => None,
            })
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let groups = get_groups(input);

    let total_cost: usize = groups
        .map(|group| {
            let area = group.positions.len();
            let mut fences = 0;

            for &(row, col) in &group.positions {
                // top
                if let Some(row) = row.checked_sub(1) {
                    if !group.positions.contains(&(row, col)) {
                        fences += 1;
                    }
                } else {
                    fences += 1;
                }

                // left
                if let Some(col) = col.checked_sub(1) {
                    if !group.positions.contains(&(row, col)) {
                        fences += 1;
                    }
                } else {
                    fences += 1;
                }

                // bottom
                {
                    let row = row + 1;
                    if !group.positions.contains(&(row, col)) {
                        fences += 1;
                    }
                }

                // right
                {
                    let col = col + 1;
                    if !group.positions.contains(&(row, col)) {
                        fences += 1;
                    }
                }
            }

            area * fences
        })
        .sum();

    Some(total_cost.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let groups = get_groups(input);

    let total_cost = groups
        .map(|group| {
            let area: u32 = group.positions.len().try_into().unwrap();
            let mut fences = Vec::new();

            for &(row, col) in &group.positions {
                let position = (row, col);

                // top
                if let Some(row) = row.checked_sub(1) {
                    if !group.positions.contains(&(row, col)) {
                        fences.push(Fence {
                            dir: Dir::Up,
                            position,
                        });
                    }
                } else {
                    fences.push(Fence {
                        dir: Dir::Up,
                        position,
                    });
                }

                // left
                if let Some(col) = col.checked_sub(1) {
                    if !group.positions.contains(&(row, col)) {
                        fences.push(Fence {
                            dir: Dir::Left,
                            position,
                        });
                    }
                } else {
                    fences.push(Fence {
                        dir: Dir::Left,
                        position,
                    });
                }

                // bottom
                {
                    let row = row + 1;
                    if !group.positions.contains(&(row, col)) {
                        fences.push(Fence {
                            dir: Dir::Down,
                            position,
                        });
                    }
                }

                // right
                {
                    let col = col + 1;
                    if !group.positions.contains(&(row, col)) {
                        fences.push(Fence {
                            dir: Dir::Right,
                            position,
                        });
                    }
                }
            }

            fences.sort_by(|a, b| {
                a.dir.cmp(&b.dir).then_with(|| match a.dir {
                    Dir::Up | Dir::Down => a
                        .position
                        .0
                        .cmp(&b.position.0)
                        .then(a.position.1.cmp(&b.position.1)),
                    Dir::Right | Dir::Left => a
                        .position
                        .1
                        .cmp(&b.position.1)
                        .then(a.position.0.cmp(&b.position.0)),
                })
            });

            let fence_chunks = fences.into_iter().chunk_by(|fence| fence.dir);
            let mut fence_chunks = fence_chunks.into_iter();

            let mut side_count = 0;

            // Top
            {
                let (dir, chunk) = fence_chunks.next().unwrap();
                debug_assert!(dir == Dir::Up);

                side_count += chunk
                    .chunk_by(|fence| fence.position.0)
                    .into_iter()
                    .map(|(_, chunk)| {
                        chunk.map(|fence| fence.position.1).tuple_windows().fold(
                            1,
                            |acc, (a, b)| {
                                debug_assert!(a < b);
                                if (b - a) == 1 {
                                    acc
                                } else {
                                    acc + 1
                                }
                            },
                        )
                    })
                    .sum::<u32>();
            }

            // Right
            {
                let (dir, chunk) = fence_chunks.next().unwrap();
                debug_assert!(dir == Dir::Right);

                side_count += chunk
                    .chunk_by(|fence| fence.position.1)
                    .into_iter()
                    .map(|(_, chunk)| {
                        chunk.map(|fence| fence.position.0).tuple_windows().fold(
                            1,
                            |acc, (a, b)| {
                                debug_assert!(a < b);
                                if (b - a) == 1 {
                                    acc
                                } else {
                                    acc + 1
                                }
                            },
                        )
                    })
                    .sum::<u32>();
            }

            // Down
            {
                let (dir, chunk) = fence_chunks.next().unwrap();
                debug_assert!(dir == Dir::Down);

                side_count += chunk
                    .chunk_by(|fence| fence.position.0)
                    .into_iter()
                    .map(|(_, chunk)| {
                        chunk.map(|fence| fence.position.1).tuple_windows().fold(
                            1,
                            |acc, (a, b)| {
                                debug_assert!(a < b);
                                if (b - a) == 1 {
                                    acc
                                } else {
                                    acc + 1
                                }
                            },
                        )
                    })
                    .sum::<u32>();
            }

            // Left
            {
                let (dir, chunk) = fence_chunks.next().unwrap();
                debug_assert!(dir == Dir::Left);

                side_count += chunk
                    .chunk_by(|fence| fence.position.1)
                    .into_iter()
                    .map(|(_, chunk)| {
                        chunk.map(|fence| fence.position.0).tuple_windows().fold(
                            1,
                            |acc, (a, b)| {
                                debug_assert!(a < b);
                                if (b - a) == 1 {
                                    acc
                                } else {
                                    acc + 1
                                }
                            },
                        )
                    })
                    .sum::<u32>();
            }

            debug_assert!(fence_chunks.next().is_none());

            area * side_count
        })
        .sum();

    return Some(total_cost);

    struct Fence {
        dir: Dir,
        position: (usize, usize),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 11,
        ));
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 12,
        ));
        assert_eq!(result, Some(772));
    }

    #[test]
    fn test_part_one_3() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 13,
        ));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 11,
        ));
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 12,
        ));
        assert_eq!(result, Some(436));
    }

    #[test]
    fn test_part_two_3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 13,
        ));
        assert_eq!(result, Some(1206));
    }

    #[test]
    fn test_part_two_4() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 21,
        ));
        assert_eq!(result, Some(236));
    }

    #[test]
    fn test_part_two_5() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 22,
        ));
        assert_eq!(result, Some(368));
    }
}
