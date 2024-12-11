advent_of_code::solution!(9);

const DEBUG: bool = false;

use std::collections::BTreeMap;

#[derive(Debug)]
enum Block {
    /// Numbered starting from 0, left to right when parsing input.
    Id(u16),

    /// Free space.
    Free,
}

impl Block {
    fn is_free(&self) -> bool {
        match self {
            Block::Free => true,
            Block::Id(_) => false,
        }
    }
}

#[derive(Debug)]
struct Disk {
    blocks: Vec<Block>,
}

impl Disk {
    /// Parse input such as "2333133121414131402" as follows:
    /// Start from the left, and the digits alternate in interpretation:
    /// - Number of blocks of current ID (start with 0 for the first).
    /// - Number of blocks of free space.
    fn new(input: &str) -> Self {
        let mut blocks = vec![];

        let mut id = 0;
        let mut is_id = true;

        // Trim newline at end if any.
        let bytes = input.trim_end().bytes();

        // Iterate through the ASCII bytes.
        for byte in bytes {
            let size = byte - b'0';
            if size != 0 {
                if is_id {
                    for _ in 0..size {
                        blocks.push(Block::Id(id));
                    }
                    id += 1;
                } else {
                    for _ in 0..size {
                        blocks.push(Block::Free);
                    }
                }
            }
            is_id = !is_id;
        }

        Self { blocks }
    }

    /// Suitable for single-digit IDs only.
    fn small_id_debug(&self) -> String {
        self.blocks
            .iter()
            .map(|block| match block {
                Block::Id(id) => id.to_string(),
                Block::Free => ".".to_string(),
            })
            .collect::<Vec<_>>()
            .join("")
    }

    /// Compact the disk by making one step at a time, each
    /// step involving:
    ///
    /// - Sweep a pointer i0 from the left to the right looking for
    ///   the first free space block.
    /// - Sweep a pointer i1 from the right to the left looking for
    ///   the rightmost non-free block.
    /// - Note that if the pointers cross, end the whole loop.
    /// - If a pair of (left free, right non-free) is found, swap
    ///   them on the disk, then continue the process.
    fn compact(&mut self) {
        if DEBUG {
            eprintln!("{}", self.small_id_debug());
        }

        let mut i0 = 0;
        let mut i1 = self.blocks.len();

        loop {
            // Find the first free space block.
            while i0 < i1 && !self.blocks[i0].is_free() {
                i0 += 1;
            }

            // Find the rightmost non-free block.
            while i0 < i1 && self.blocks[i1 - 1].is_free() {
                i1 -= 1;
            }

            // If the pointers cross, end the loop.
            if i0 >= i1 {
                break;
            }

            // Swap the blocks.
            self.blocks.swap(i0, i1 - 1);
            if DEBUG {
                eprintln!("{}", self.small_id_debug());
            }
        }
    }

    /// Return checksum which is the sum of the block index times
    /// block ID for each position on the disk. Ignore free space.
    fn checksum(&self) -> u64 {
        self.blocks
            .iter()
            .enumerate()
            .fold(0, |acc, (i, block)| match block {
                Block::Id(id) => acc + (i as u64 * *id as u64),
                Block::Free => acc,
            })
    }
}

#[derive(Debug)]
struct File {
    // Index of start in blocks vector.
    start: u32,

    // Size of contiguous region of the same ID.
    // Always a single digit, so 1-9. Empty is not created.
    size: u8,
}

#[derive(Debug)]
struct FreeSpace {
    size: u8,
}

#[derive(Debug)]
struct Disk2 {
    /// Map ID as index to File.
    files: Vec<File>,

    /// Map start address to free space block.
    free_space: BTreeMap<u32, FreeSpace>,

    /// Number of blocks in entire disk.
    size: u32,
}

impl Disk2 {
    /// Parse input such as "2333133121414131402" as follows:
    /// Start from the left, and the digits alternate in interpretation:
    /// - Number of blocks of current ID (start with 0 for the first).
    /// - Number of blocks of free space.
    fn new(input: &str) -> Self {
        let mut files = vec![];
        let mut free_space = BTreeMap::new();

        let mut is_id = true;

        // Trim newline at end if any.
        let bytes = input.trim_end().bytes();

        // Iterate through the ASCII bytes.
        let mut start = 0;
        for byte in bytes {
            let size = byte - b'0';
            if size != 0 {
                if is_id {
                    files.push(File { start, size });
                } else {
                    free_space.insert(start, FreeSpace { size });
                }

                start += size as u32;
            }
            is_id = !is_id;
        }

        Self {
            files,
            free_space,
            size: start,
        }
    }

    /// Suitable for single-digit IDs only.
    fn small_id_debug(&self) -> String {
        // Allocate byte vector of disk size.
        let mut bytes = vec![b'.'; self.size as usize];

        for (id, file) in self.files.iter().enumerate() {
            // Convert ASCII small id to digit as byte.
            let id_byte = b'0' + id as u8;

            for i in 0..file.size {
                let index = file.start + i as u32;
                bytes[index as usize] = id_byte;
            }
        }
        String::from_utf8(bytes).unwrap()
    }

    fn move_file_to_leftmost_free_space(&mut self, file_id: u32) -> bool {
        let file = &mut self.files[file_id as usize];

        let cur_start = file.start;
        let file_size = file.size;

        // Find the leftmost free space that is big enough.
        let mut new_start = 0;
        let mut free_size = 0;

        let mut found = false;
        for (&free_start, free_space) in self.free_space.iter() {
            if free_start > cur_start {
                // Ran out of free space.
                found = false;
                break;
            }

            if free_space.size >= file_size {
                found = true;

                new_start = free_start;
                free_size = free_space.size;

                if DEBUG {
                    eprintln!(
                        "Found space at {} of size {} for file {} of size {}",
                        new_start, free_size, file_id, file_size
                    );
                }

                break;
            }
        }

        if !found {
            return false;
        }

        // Update the file's position
        file.start = new_start;

        // Update free space
        // Rid the old free space region.
        self.free_space.remove(&new_start);

        if file_size < free_size {
            // Split the free space region.
            let new_free_start = new_start + file_size as u32;
            let new_free_size = free_size - file_size;
            self.free_space.insert(
                new_free_start,
                FreeSpace {
                    size: new_free_size,
                },
            );
        }
        true
    }

    /// Compact the disk by moving ID "files" (starting from the
    /// highest ID) to the leftmost free space until all possible
    /// moves are done. We know where all of the ID files are,
    /// from initial compaction.
    fn compact(&mut self) {
        if DEBUG {
            eprintln!("{}", self.small_id_debug());
        }

        for file_id in (0..self.files.len()).rev() {
            if self.move_file_to_leftmost_free_space(file_id as u32) && DEBUG {
                eprintln!("{}", self.small_id_debug());
            }
        }
    }

    /// Return checksum which is the sum of the block index times
    /// block ID for each position on the disk. Ignore free space.
    /// Remember to sum all the blocks in each file using the
    /// start index for each block in the file.
    fn checksum(&self) -> u64 {
        self.files.iter().enumerate().fold(0, |acc, (id, file)| {
            // Sum of indices start to (start + size -1),
            // inclusive. Use math!
            let start: u64 = file.start as u64;
            let size: u64 = file.size as u64;
            let sum_of_indices: u64 = size * (2 * start + size - 1) / 2;

            // Do it naively instead.
            // let sum_of_indices: u64 = (start..(start + size)).sum();

            acc + (id as u64 * sum_of_indices)
        })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk = Disk::new(input);
    disk.compact();
    Some(disk.checksum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk2 = Disk2::new(input);
    disk2.compact();
    Some(disk2.checksum())
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
