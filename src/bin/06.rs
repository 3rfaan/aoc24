use std::{collections::HashSet, ops::Add};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    Some(Lab::from(input).walk().len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lab = Lab::from(input);

    let origin = lab.guard;
    let visited = lab.walk();

    Some(
        visited
            .iter()
            .filter(|&&obstacle| lab.looping(origin, obstacle))
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

    fn walk(&mut self) -> HashSet<Pos> {
        let mut visited = HashSet::new();

        loop {
            let next = self.guard.pos + self.guard.dir.offset();

            visited.insert(self.guard.pos);

            match self.get(next) {
                Some(b'#') => self.guard.dir = self.guard.dir.turn(),
                Some(_) => self.guard.pos = next,
                None => break,
            }
        }
        visited
    }

    fn looping(&mut self, origin: Guard, obstacle: Pos) -> bool {
        let mut visited = HashSet::new();

        self.guard = origin;
        self.set(obstacle, b'O');

        let looping = loop {
            if !visited.insert((self.guard.pos, self.guard.dir)) {
                break true;
            }
            let next = self.guard.pos + self.guard.dir.offset();

            match self.get(next) {
                Some(b'#' | b'O') => self.guard.dir = self.guard.dir.turn(),
                Some(_) => self.guard.pos = next,
                None => break false,
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

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Pos(i32, i32);

impl Add<Off> for Pos {
    type Output = Self;
    fn add(self, Off(dx, dy): Off) -> Self::Output {
        Pos(self.0 + dx, self.1 + dy)
    }
}

struct Off(i32, i32);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
