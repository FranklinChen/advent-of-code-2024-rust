advent_of_code::solution!(9);

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
            let digit = byte - b'0';
            if is_id {
                for _ in 0..digit {
                    blocks.push(Block::Id(id));
                }
                id += 1;
            } else {
                for _ in 0..digit {
                    blocks.push(Block::Free);
                }
            }
            is_id = !is_id;
        }

        Self { blocks }
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
        }
    }

    /// Return checksum which is the sum of the block index times
    /// block ID for each position on the disk.
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

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk = Disk::new(input);
    disk.compact();
    Some(disk.checksum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
