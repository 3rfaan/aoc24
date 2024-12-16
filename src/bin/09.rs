use std::collections::VecDeque;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    Some(DiskMap::from(input).decompress().defragment().checksum())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        DiskMap::from(input)
            .defragment_retain_blocks()
            .decompress()
            .checksum(),
    )
}

#[derive(Clone)]
struct DiskMap {
    disk: VecDeque<Block>,
}

impl DiskMap {
    fn checksum(&self) -> usize {
        self.disk
            .iter()
            .enumerate()
            .filter_map(|(i, block)| match block {
                Block::File { id, .. } => Some(i * id),
                _ => None,
            })
            .sum()
    }

    fn decompress(self) -> Self {
        Self {
            disk: self
                .disk
                .into_iter()
                .flat_map(|block| match block {
                    Block::File { id, size } => vec![Block::File { id, size: 1 }; size],
                    Block::Free { size } => vec![Block::Free { size: 1 }; size],
                })
                .collect(),
        }
    }

    fn defragment(&mut self) -> Self {
        let mut disk = VecDeque::new();

        while let Some(block) = self.disk.pop_front() {
            match block {
                Block::File { .. } => disk.push_back(block),
                Block::Free { .. } => {
                    while let Some(block) = self.disk.pop_back() {
                        if let Block::File { .. } = block {
                            disk.push_back(block);
                            break;
                        }
                    }
                }
            }
        }
        Self { disk }
    }

    fn defragment_retain_blocks(&mut self) -> Self {
        let mut disk = VecDeque::new();

        while let Some(block) = self.disk.pop_front() {
            match block {
                Block::File { .. } => disk.push_back(block),
                Block::Free { size: mut free } => {
                    (0..self.disk.len()).rev().for_each(|i| {
                        if let Block::File { size, .. } = self.disk[i] {
                            if size <= free {
                                disk.push_back(self.disk[i]);
                                self.disk.remove(i);
                                self.disk.insert(i, Block::Free { size });
                                free -= size;
                            }
                        }
                    });

                    if free > 0 {
                        disk.push_back(Block::Free { size: free })
                    }
                }
            }
        }
        Self { disk }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Block {
    File { id: usize, size: usize },
    Free { size: usize },
}

impl From<&str> for DiskMap {
    fn from(puzzle: &str) -> Self {
        let disk: VecDeque<Block> =
            puzzle
                .chars()
                .enumerate()
                .fold(VecDeque::new(), |mut block, (id, size)| {
                    if let Some(size) = size.to_digit(10) {
                        match id % 2 {
                            0 => block.push_back(Block::File {
                                id: id / 2,
                                size: size as usize,
                            }),
                            _ => block.push_back(Block::Free {
                                size: size as usize,
                            }),
                        }
                    }
                    block
                });

        Self { disk }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
