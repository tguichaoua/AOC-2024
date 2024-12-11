use std::{collections::HashSet, iter};

use itertools::Itertools;

advent_of_code::solution!();

pub fn part_one(input: &str) -> Option<u32> {
    // Let assume the input contains only ascii chars
    debug_assert!(input.is_ascii());

    let lines = input.lines().map(|line| line.as_bytes()).collect_vec();

    const NEEDLE: &[u8] = b"XMAS";
    const BACK_NEEDLE: &[u8] = b"SAMX";

    let height = lines.len();
    let width = lines.first().unwrap().len();

    debug_assert!(lines.iter().all(|l| l.len() == width));

    let columns = {
        let mut columns = iter::repeat_with(|| Vec::with_capacity(height))
            .take(width)
            .collect_vec();

        for line in &lines {
            for (s, &c) in columns.iter_mut().zip(*line) {
                s.push(c);
            }
        }

        columns
    };

    let tl_to_dr_diagonals = {
        let total_diagonals = height + width - 1;
        // Removes the diagonals that are too short to contains the needle.
        let diagonal_count = total_diagonals - 2 * (NEEDLE.len() - 1);

        let mut diagonals: Vec<Vec<u8>> =
            iter::repeat_with(|| Vec::with_capacity(usize::max(height, width)))
                .take(diagonal_count)
                .collect_vec();

        for (i, line) in lines.iter().enumerate() {
            for (j, &c) in line.iter().enumerate() {
                let Some(d) = ((height - NEEDLE.len()) + j).checked_sub(i) else {
                    continue;
                };
                if let Some(d) = diagonals.get_mut(d) {
                    d.push(c);
                } else {
                    break;
                }
            }
        }

        diagonals
    };

    let dl_to_tr_diagonals = {
        let total_diagonals = height + width - 1;
        // Removes the diagonals that are too short to contains the needle.
        let diagonal_count = total_diagonals - 2 * (NEEDLE.len() - 1);

        let mut diagonals: Vec<Vec<u8>> =
            iter::repeat_with(|| Vec::with_capacity(usize::max(height, width)))
                .take(diagonal_count)
                .collect_vec();

        for (i, line) in lines.iter().enumerate() {
            for (j, &c) in line.iter().enumerate() {
                let Some(d) = (i + j).checked_sub(NEEDLE.len() - 1) else {
                    continue;
                };
                if let Some(d) = diagonals.get_mut(d) {
                    d.push(c);
                } else {
                    break;
                }
            }
        }

        diagonals
    };

    let mut count = 0;

    // Horizontal
    for line in &lines {
        for i in 0..(line.len() - NEEDLE.len() + 1) {
            // Left to right
            if line[i..].starts_with(NEEDLE) {
                count += 1
            }
            // Right to left
            if line[i..].starts_with(BACK_NEEDLE) {
                count += 1
            }
        }
    }

    // Vertical
    for column in &columns {
        for i in 0..(column.len() - NEEDLE.len() + 1) {
            // Top to bottom
            if column[i..].starts_with(NEEDLE) {
                count += 1
            }
            // Bottom to top
            if column[i..].starts_with(BACK_NEEDLE) {
                count += 1
            }
        }
    }

    // Diagonal top-left to down-right
    for diagonal in &tl_to_dr_diagonals {
        for i in 0..(diagonal.len() - NEEDLE.len() + 1) {
            if diagonal[i..].starts_with(NEEDLE) {
                count += 1
            }
            if diagonal[i..].starts_with(BACK_NEEDLE) {
                count += 1
            }
        }
    }

    // Diagonal down-left to top-right
    for diagonal in &dl_to_tr_diagonals {
        for i in 0..(diagonal.len() - NEEDLE.len() + 1) {
            if diagonal[i..].starts_with(NEEDLE) {
                count += 1
            }
            if diagonal[i..].starts_with(BACK_NEEDLE) {
                count += 1
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Let assume the input contains only ascii chars
    debug_assert!(input.is_ascii());

    let lines = input.lines().map(|line| line.as_bytes()).collect_vec();

    const NEEDLE: &[u8] = b"MAS";
    const BACK_NEEDLE: &[u8] = b"SAM";

    let height = lines.len();
    let width = lines.first().unwrap().len();

    debug_assert!(lines.iter().all(|l| l.len() == width));

    let tl_to_dr_diagonals = {
        let total_diagonals = height + width - 1;
        // Removes the diagonals that are too short to contains the needle.
        let diagonal_count = total_diagonals - 2 * (NEEDLE.len() - 1);

        let mut diagonals: Vec<Vec<u8>> =
            iter::repeat_with(|| Vec::with_capacity(usize::max(height, width)))
                .take(diagonal_count)
                .collect_vec();

        for (i, line) in lines.iter().enumerate() {
            for (j, &c) in line.iter().enumerate() {
                let Some(d) = ((height - NEEDLE.len()) + j).checked_sub(i) else {
                    continue;
                };
                if let Some(d) = diagonals.get_mut(d) {
                    d.push(c);
                } else {
                    break;
                }
            }
        }

        diagonals
    };

    let dl_to_tr_diagonals = {
        let total_diagonals = height + width - 1;
        // Removes the diagonals that are too short to contains the needle.
        let diagonal_count = total_diagonals - 2 * (NEEDLE.len() - 1);

        let mut diagonals: Vec<Vec<u8>> =
            iter::repeat_with(|| Vec::with_capacity(usize::max(height, width)))
                .take(diagonal_count)
                .collect_vec();

        for (i, line) in lines.iter().enumerate() {
            for (j, &c) in line.iter().enumerate() {
                let Some(d) = (i + j).checked_sub(NEEDLE.len() - 1) else {
                    continue;
                };
                if let Some(d) = diagonals.get_mut(d) {
                    d.push(c);
                } else {
                    break;
                }
            }
        }

        diagonals
    };

    let mut count = 0;

    let mut tl_to_dr_needle_coordinates = HashSet::new();

    fn tl_to_dr_diag_to_cartesian_coord(height: usize, d: usize, i: usize) -> (usize, usize) {
        if let Some(l) = (height - NEEDLE.len()).checked_sub(d) {
            (l + i, i)
        } else {
            (i, d - (height - NEEDLE.len()) + i)
        }
    }

    fn dl_to_tr_diag_to_cartesian_coord(width: usize, d: usize, i: usize) -> (usize, usize) {
        if d <= (width - NEEDLE.len()) {
            (i, d + (NEEDLE.len() - 1) - i)
        } else {
            (d + i - (width - NEEDLE.len()), width - 1 - i)
        }
    }

    // Diagonal top-left to down-right
    for (d, diagonal) in tl_to_dr_diagonals.iter().enumerate() {
        for i in 0..(diagonal.len() - NEEDLE.len() + 1) {
            if diagonal[i..].starts_with(NEEDLE) {
                tl_to_dr_needle_coordinates.insert(tl_to_dr_diag_to_cartesian_coord(
                    height,
                    d,
                    i + 1,
                ));
            }
            if diagonal[i..].starts_with(BACK_NEEDLE) {
                tl_to_dr_needle_coordinates.insert(tl_to_dr_diag_to_cartesian_coord(
                    height,
                    d,
                    i + 1,
                ));
            }
        }
    }

    // Diagonal down-left to top-right
    for (d, diagonal) in dl_to_tr_diagonals.iter().enumerate() {
        for i in 0..(diagonal.len() - NEEDLE.len() + 1) {
            if diagonal[i..].starts_with(NEEDLE) {
                let crd = dl_to_tr_diag_to_cartesian_coord(width, d, i + 1);
                if tl_to_dr_needle_coordinates.contains(&crd) {
                    count += 1;
                }
            }
            if diagonal[i..].starts_with(BACK_NEEDLE) {
                let crd = dl_to_tr_diag_to_cartesian_coord(width, d, i + 1);
                if tl_to_dr_needle_coordinates.contains(&crd) {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
