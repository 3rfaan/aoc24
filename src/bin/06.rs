use std::{collections::HashSet, ops::Add};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lab = Lab::from(input);
    let mut visited = HashSet::from([lab.guard.pos]); // Insert starting position

    Some(lab.walk(&mut visited))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

struct Lab {
    grid: Vec<Vec<u8>>,
    guard: Guard,
}

impl Lab {
    fn get(&self, &Pos(x, y): &Pos) -> Option<&u8> {
        self.grid.get(x as usize)?.get(y as usize)
    }

    fn walk(&mut self, visited: &mut HashSet<Pos>) -> u32 {
        let mut count = 1;

        loop {
            let offset = self.guard.dir.offset();
            let new_pos = self.guard.pos + offset;

            match self.get(&new_pos) {
                Some(&b'#') => self.guard.dir = self.guard.dir.turn_right(),
                None => break,
                _ => {
                    self.guard.pos = new_pos;
                    if visited.insert(new_pos) {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

struct Guard {
    pos: Pos,
    dir: Dir,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Pos(i32, i32);
struct Off(i32, i32);

impl Add<Off> for Pos {
    type Output = Self;

    fn add(self, other: Off) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}

impl Dir {
    fn offset(&self) -> Off {
        match self {
            Dir::Up => Off(-1, 0),
            Dir::Down => Off(1, 0),
            Dir::Right => Off(0, 1),
            Dir::Left => Off(0, -1),
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

impl From<&str> for Lab {
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
            .map(|(_, row_idx, col_idx)| Pos(row_idx as i32, col_idx as i32))
            .unwrap_or_default();

        let guard = Guard { pos, dir: Dir::Up };

        Self { grid: area, guard }
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
