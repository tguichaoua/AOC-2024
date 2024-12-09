use num::Integer;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let input = if input.len().is_even() {
        // If the last item is a free space, ignore it
        &input[..input.len() - 1]
    } else {
        input
    };

    let mut data = input.bytes().map(|n| {
        debug_assert!(n.is_ascii_digit());
        n - b'0'
    });

    // The id of the next file from the start
    let mut left_file_id = 0;
    // The id of the next file from the end
    let mut right_file_id = input.len() / 2;
    let mut block_position = 0;
    let mut checksum = 0;
    // The amount of data to transfer from the end
    let mut pending_file_to_transfer = 0;

    'outer: loop {
        let Some(file_size) = data.next() else {
            break;
        };

        for _ in 0..file_size {
            checksum += block_position * left_file_id;
            block_position += 1;
        }

        left_file_id += 1;

        let Some(mut space) = data.next() else {
            break;
        };

        while space > 0 {
            if pending_file_to_transfer == 0 {
                let Some(file_size_at_end) = data.next_back() else {
                    break 'outer;
                };
                // consume space at end (we don't need it)
                data.next_back();

                pending_file_to_transfer = file_size_at_end;
            }

            if pending_file_to_transfer <= space {
                for _ in 0..pending_file_to_transfer {
                    checksum += block_position * right_file_id;
                    block_position += 1;
                }

                space -= pending_file_to_transfer;
                pending_file_to_transfer = 0;
                right_file_id -= 1;
            } else {
                for _ in 0..space {
                    checksum += block_position * right_file_id;
                    block_position += 1;
                }
                pending_file_to_transfer -= space;
                space = 0;
            }
        }
    }

    for _ in 0..pending_file_to_transfer {
        checksum += block_position * right_file_id;
        block_position += 1;
    }

    Some(checksum.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = if input.len().is_even() {
        // If the last item is a free space, ignore it
        &input[..input.len() - 1]
    } else {
        input
    };

    struct File {
        pos: usize,
        id: usize,
        size: usize,
    }

    struct Space {
        pos: usize,
        size: usize,
    }

    let half_len = input.len() / 2;
    let mut files = Vec::with_capacity(half_len + 1);
    let mut spaces = Vec::with_capacity(half_len);

    let data = input
        .bytes()
        .map(|n| {
            debug_assert!(n.is_ascii_digit());
            (n - b'0') as usize
        })
        .enumerate();

    let mut pos = 0;

    for (i, size) in data {
        if i.is_even() {
            files.push(File {
                pos,
                id: i / 2,
                size,
            });
        } else {
            spaces.push(Space { pos, size });
        }
        pos += size;
    }

    for file in files.iter_mut().rev() {
        if let Some(space) = spaces
            .iter_mut()
            .find(|space| space.pos < file.pos && space.size >= file.size)
        {
            file.pos = space.pos;
            space.size -= file.size;
            space.pos += file.size;
        }
    }

    let checksum: usize = files
        .into_iter()
        .map(|File { pos, id, size }| (pos..pos + size).sum::<usize>() * id)
        .sum();

    Some(checksum.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
