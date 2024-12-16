advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let test = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let test = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

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
