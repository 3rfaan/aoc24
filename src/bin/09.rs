advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u32> {
    let test = "2333133121414131402";
    let mut disk_map = DiskMap::from(test);

    disk_map.defragment();

    println!("{:?}", disk_map.disk);

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

struct DiskMap {
    disk: Vec<Block>,
}

impl DiskMap {
    fn defragment(&mut self) {
        let mut disk = Vec::new();

        for block in &self.disk {
            if let Block::File { id, count } = block {
                for _ in 0..*count {
                    disk.push(Block::File { id: *id, count: 1 })
                }
            }
        }

        let free = self
            .disk
            .iter()
            .filter_map(|block| match block {
                Block::Free { count } => Some(*count),
                _ => None,
            })
            .sum();

        for _ in 0..free {
            disk.push(Block::Free { count: 1 });
        }

        self.disk = disk;
    }
}

#[derive(Debug)]
enum Block {
    File { id: usize, count: u32 },
    Free { count: u32 },
}

impl From<&str> for DiskMap {
    fn from(puzzle: &str) -> Self {
        let disk: Vec<Block> = puzzle
            .chars()
            .enumerate()
            .filter_map(|(id, count)| {
                count.to_digit(10).map(|count| {
                    if id % 2 == 0 {
                        Block::File { id: id / 2, count }
                    } else {
                        Block::Free { count }
                    }
                })
            })
            .collect();

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
