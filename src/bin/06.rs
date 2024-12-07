use std::{collections::HashSet, ops::Add};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::from(input);

    Some(map.walk())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

struct Map {
    area: Vec<Vec<u8>>,
    pos: Point,
    dir: Dir,
    visited: HashSet<Point>,
}

impl Map {
    fn get(&self, &Point(x, y): &Point) -> Option<&u8> {
        self.area.get(x as usize)?.get(y as usize)
    }

    fn walk(&mut self) -> u32 {
        let mut count = 1;

        loop {
            let offset = self.dir.offset();
            let new_pos = self.pos + offset;

            match self.get(&new_pos) {
                Some(&b'#') => self.dir = self.dir.turn_right(),
                None => break,
                _ => {
                    self.pos = new_pos;
                    if self.visited.insert(new_pos) {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Point(i32, i32);

impl Add<(i32, i32)> for Point {
    type Output = Self;

    fn add(self, other: (i32, i32)) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}

impl Dir {
    fn offset(&self) -> (i32, i32) {
        match self {
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
            Dir::Right => (0, 1),
            Dir::Left => (0, -1),
        }
    }

    fn turn_right(&mut self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let area: Vec<Vec<u8>> = input.lines().map(|row| row.bytes().collect()).collect();

        let pos = area
            .iter()
            .enumerate()
            .flat_map(|(row_idx, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(col_idx, &col)| (col, row_idx, col_idx))
            })
            .find(|&(col, _, _)| col == b'^')
            .map(|(_, row_idx, col_idx)| Point(row_idx as i32, col_idx as i32))
            .unwrap_or_default();

        let visited = HashSet::from([pos]); // Insert starting position

        Self {
            area,
            pos,
            dir: Dir::Up,
            visited,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one("test input");
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two("test input");
        assert_eq!(result, None);
    }
}
