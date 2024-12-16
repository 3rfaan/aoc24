use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    let test = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    Some(Map::from(test).walk().0)
}

pub fn part_two(input: &str) -> Option<usize> {
    let test = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    Some(Map::from(test).walk().1)
}

struct Map {
    grid: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
}

impl Map {
    fn get(&self, row: usize, col: usize) -> Option<u8> {
        self.grid.get(row)?.get(col).copied()
    }

    fn reachable_nines(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut q = VecDeque::from([(row, col)]);
        let mut seen = Vec::new();
        while let Some((r, c)) = q.pop_front() {
            if self.grid[r][c] == b'9' {
                seen.push((r, c));
                continue;
            }
            for (rr, cc) in [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)] {
                if *self.grid.get(rr).and_then(|row| row.get(cc)).unwrap_or(&0)
                    == self.grid[r][c] + 1
                {
                    q.push_back((rr, cc));
                }
            }
        }
        seen
    }

    fn walk(&self) -> (usize, usize) {
        let map = (0..self.rows)
            .flat_map(|row| (0..self.cols).map(move |col| (row, col)))
            .filter(|&(row, col)| self.grid[row][col] == b'0')
            .fold((0, 0), |(mut p1, mut p2), (row, col)| {
                let scores = self.reachable_nines(row, col);
                p1 += scores.iter().collect::<HashSet<_>>().len();
                p2 += scores.len();
                (p1, p2)
            });
        map
    }
}

impl From<&str> for Map {
    fn from(puzzle: &str) -> Self {
        let grid: Vec<Vec<u8>> = puzzle.lines().map(|line| line.bytes().collect()).collect();
        let (rows, cols) = (grid.len(), grid.first().map_or(0, |row| row.len()));

        Self { grid, rows, cols }
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
