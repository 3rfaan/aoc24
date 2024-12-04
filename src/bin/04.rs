advent_of_code::solution!(4);

struct Grid {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn get_word_count(&mut self, word: &str) -> Option<u32> {
        let dirs = Direction::get_dirs();
        let mut count = 0;

        for row in 0..self.rows {
            for col in 0..self.cols {
                for dir in &dirs {
                    if self.walk(word, row, col, dir) {
                        count += 1;
                    }
                }
            }
        }
        Some(count)
    }

    fn walk(&self, word: &str, row: usize, col: usize, dir: &Direction) -> bool {
        let word_chars: Vec<char> = word.chars().collect();
        let (dx, dy) = dir.offset();

        for word_idx in 0..word_chars.len() {
            let new_row = row as isize + word_idx as isize * dx;
            let new_col = col as isize + word_idx as isize * dy;

            if new_row < 0
                || new_col < 0
                || new_row >= self.rows as isize
                || new_col >= self.cols as isize
            {
                return false;
            }

            let (nx, ny) = (new_row as usize, new_col as usize);

            if self.grid[nx][ny] != word_chars[word_idx] {
                return false;
            }
        }
        true
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
        let rows = grid.len();
        let cols = grid[0].len();

        Self { grid, rows, cols }
    }
}

enum Direction {
    Right,
    Left,
    Down,
    Up,
    DiagDownRight,
    DiagUpLeft,
    DiagDownLeft,
    DiagUpRight,
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Right => (0, 1),
            Direction::Left => (0, -1),
            Direction::Down => (1, 0),
            Direction::Up => (-1, 0),
            Direction::DiagDownRight => (1, 1),
            Direction::DiagUpLeft => (-1, -1),
            Direction::DiagDownLeft => (1, -1),
            Direction::DiagUpRight => (-1, 1),
        }
    }

    fn get_dirs() -> Vec<Self> {
        vec![
            Direction::Right,
            Direction::Left,
            Direction::Down,
            Direction::Up,
            Direction::DiagDownRight,
            Direction::DiagUpLeft,
            Direction::DiagDownLeft,
            Direction::DiagUpRight,
        ]
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let word = "XMAS";

    Grid::from(input).get_word_count(word)
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
