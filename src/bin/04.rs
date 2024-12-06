advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from(input);

    let mut count = 0;

    (0..grid.rows)
        .flat_map(|row| (0..grid.cols).map(move |col| (row, col)))
        .filter(|&(row, col)| grid.bytes[row][col] == b'X')
        .for_each(|(row, col)| count += grid.xmas_count(row, col));

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from(input);

    Some(
        (0..grid.rows)
            .flat_map(|row| (0..grid.cols).map(move |col| (row, col)))
            .filter(|&(row, col)| grid.bytes[row][col] == b'A')
            .filter(|&(row, col)| grid.crossmas_count(row, col))
            .count() as u32,
    )
}

struct Grid {
    bytes: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn get(&self, row: isize, col: isize) -> u8 {
        *self
            .bytes
            .get(row as usize)
            .and_then(|row| row.get(col as usize))
            .unwrap_or(&b'.')
    }

    fn xmas_count(&self, row: usize, col: usize) -> usize {
        [
            (0, 1),
            (1, 0),
            (1, 1),
            (0, -1),
            (-1, 0),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ]
        .iter()
        .filter(|(off_x, off_y)| {
            (0..4).all(|i| {
                let new_row = row as isize + (off_x * i);
                let new_col = col as isize + (off_y * i);

                self.get(new_row, new_col) == b"XMAS"[i as usize]
            })
        })
        .count()
    }

    fn crossmas_count(&self, row: usize, col: usize) -> bool {
        let (row, col) = (row as isize, col as isize);

        let diag1 = [self.get(row - 1, col - 1), self.get(row + 1, col + 1)];
        let diag2 = [self.get(row - 1, col + 1), self.get(row + 1, col - 1)];

        [diag1, diag2].iter().all(|w| w == b"MS" || w == b"SM")
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let bytes: Vec<Vec<u8>> = input.lines().map(|row| row.bytes().collect()).collect();
        let (rows, cols) = (bytes.len(), bytes.first().map_or(0, |row| row.len()));

        Self { bytes, rows, cols }
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
