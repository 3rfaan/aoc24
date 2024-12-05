advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from(input);
    let offsets = vec![
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // down
        (-1, 0),  // up
        (1, 1),   // up left
        (-1, -1), // up right
        (1, -1),  // down left
        (-1, 1),  // down right
    ];

    Some(
        (0..grid.rows)
            .flat_map(|row| (0..grid.cols).map(move |col| (row, col)))
            .flat_map(|(row, col)| offsets.iter().map(move |dir| (row, col, dir)))
            .filter(|&(row, col, dir)| grid.xmas_finder(row, col, &dir))
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from(input);

    Some(
        (0..grid.rows)
            .flat_map(|row| (0..grid.cols).map(move |col| (row, col)))
            .filter(|&(row, col)| grid.cross_finder(row, col))
            .count() as u32,
    )
}

macro_rules! off {
    ($self:expr,($row:ident $op1:tt $off1:literal),($col:ident $op2:tt $off2:literal)) => {
        $self.grid
            .get(($row as isize $op1 $off1) as usize)
            .and_then(|c| c.get(($col as isize $op2 $off2) as usize))
    };
}

struct Grid {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn xmas_finder(&self, row: usize, col: usize, off: &(isize, isize)) -> bool {
        let word = "XMAS";
        let (off_x, off_y) = off;

        word.chars().enumerate().all(|(i, word_char)| {
            let new_row = row as isize + i as isize * off_x;
            let new_col = col as isize + i as isize * off_y;

            let (nx, ny) = (new_row as usize, new_col as usize);

            match self.grid.get(nx).and_then(|row| row.get(ny)) {
                Some(&letter) if letter == word_char => true,
                _ => false,
            }
        })
    }

    fn cross_finder(&self, row: usize, col: usize) -> bool {
        if self.grid[row][col] != 'A' {
            return false;
        }

        let ul = off!(self, (row - 1), (col - 1)); // up left
        let ur = off!(self, (row + 1), (col - 1)); // up right
        let dl = off!(self, (row - 1), (col + 1)); // down left
        let dr = off!(self, (row + 1), (col + 1)); // down right

        let (mut ul_to_dr, mut ur_to_dl) = (false, false);

        if let (Some(&ul), Some(&ur), Some(&dl), Some(&dr)) = (ul, ur, dl, dr) {
            ul_to_dr = (ul == 'M' && dr == 'S') || (ul == 'S' && dr == 'M');
            ur_to_dl = (ur == 'M' && dl == 'S') || (ur == 'S' && dl == 'M');
        }

        ul_to_dr && ur_to_dl
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
        let (rows, cols) = (grid.len(), grid[0].len());

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
