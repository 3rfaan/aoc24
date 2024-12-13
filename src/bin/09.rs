use std::collections::VecDeque;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    Some(DiskMap::from(input).defragment().checksum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

struct DiskMap {
    disk: VecDeque<Block>,
}

impl DiskMap {
    fn checksum(&self) -> usize {
        self.disk
            .iter()
            .enumerate()
            .filter_map(|(i, block)| match block {
                Block::File(id) => Some(i * id),
                _ => None,
            })
            .sum()
    }

    fn defragment(mut self) -> Self {
        let mut files = VecDeque::new();

        while let Some(block) = self.disk.pop_front() {
            match block {
                Block::File(_) => files.push_back(block),
                Block::Free => {
                    while let Some(block) = self.disk.pop_back() {
                        if let Block::File(back) = block {
                            files.push_back(Block::File(back));
                            break;
                        }
                    }
                }
            }
        }
        Self { disk: files }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Block {
    File(usize),
    Free,
}

impl From<&str> for DiskMap {
    fn from(puzzle: &str) -> Self {
        let disk: VecDeque<Block> =
            puzzle
                .chars()
                .enumerate()
                .fold(VecDeque::new(), |mut block, (id, count)| {
                    if let Some(count) = count.to_digit(10) {
                        match id % 2 {
                            0 => (0..count).for_each(|_| block.push_back(Block::File(id / 2))),
                            _ => (0..count).for_each(|_| block.push_back(Block::Free)),
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
