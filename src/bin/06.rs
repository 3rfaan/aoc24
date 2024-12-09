use std::{collections::HashSet, ops::Add};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    Some(Lab::from(input).walk().len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lab = Lab::from(input);
    let origin = lab.guard;

    Some(
        lab.walk()
            .iter()
            .filter(|&&obstacle| lab.is_looping(origin, obstacle))
            .count() as u32,
    )
}

struct Lab {
    grid: Vec<Vec<u8>>,
    guard: Guard,
}

impl Lab {
    fn get(&self, Pos(x, y): Pos) -> Option<u8> {
        self.grid.get(x as usize)?.get(y as usize).copied()
    }

    fn set(&mut self, Pos(x, y): Pos, val: u8) {
        if let Some(cell) = self
            .grid
            .get_mut(x as usize)
            .and_then(|row| row.get_mut(y as usize))
        {
            *cell = val;
        }
    }

    fn next(&mut self) -> Option<()> {
        let next = self.guard.pos + self.guard.dir.offset();
        match self.get(next) {
            Some(b'#' | b'O') => {
                self.guard.dir = self.guard.dir.turn();
                Some(())
            }
            Some(_) => {
                self.guard.pos = next;
                Some(())
            }
            None => None,
        }
    }

    fn walk(&mut self) -> HashSet<Pos> {
        let mut visited = HashSet::new();

        loop {
            visited.insert(self.guard.pos);

            if let None = self.next() {
                break;
            }
        }
        visited
    }

    fn is_looping(&mut self, origin: Guard, obstacle: Pos) -> bool {
        let mut visited = HashSet::new();

        self.guard = origin;
        self.set(obstacle, b'O');

        let looping = loop {
            if !visited.insert((self.guard.pos, self.guard.dir)) {
                break true;
            }

            if let None = self.next() {
                break false;
            }
        };
        self.set(obstacle, b'.');
        looping
    }
}

#[derive(Copy, Clone)]
struct Guard {
    pos: Pos,
    dir: Dir,
}

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
struct Pos(i32, i32);

impl Add<Off> for Pos {
    type Output = Self;

    fn add(self, Off(dx, dy): Off) -> Self::Output {
        Pos(self.0 + dx, self.1 + dy)
    }
}

struct Off(i32, i32);

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}

impl Dir {
    fn offset(self) -> Off {
        match self {
            Dir::Up => Off(-1, 0),
            Dir::Down => Off(1, 0),
            Dir::Right => Off(0, 1),
            Dir::Left => Off(0, -1),
        }
    }

    fn turn(self) -> Self {
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
        let grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();
        let pos = grid
            .iter()
            .enumerate()
            .find_map(|(x, row)| {
                row.iter()
                    .position(|&c| c == b'^')
                    .map(|y| Pos(x as i32, y as i32))
            })
            .unwrap_or_default();
        Self {
            grid,
            guard: Guard { pos, dir: Dir::Up },
        }
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
