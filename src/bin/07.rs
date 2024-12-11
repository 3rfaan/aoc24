advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    run(input, &Concat::Off)
}

pub fn part_two(input: &str) -> Option<u64> {
    run(input, &Concat::On)
}

enum Concat {
    On,
    Off,
}

fn run(puzzle: &str, mode: &Concat) -> Option<u64> {
    Some(
        parse(puzzle)
            .iter()
            .filter(|(test, num)| solve(*test, num, mode))
            .map(|(test, _)| test)
            .sum(),
    )
}

fn concat(a: u64, b: u64) -> u64 {
    let b_len = (b as f64).log10().floor() as u32 + 1;

    a * 10u64.pow(b_len) + b
}

fn solve(test: u64, nums: &[u64], mode: &Concat) -> bool {
    nums.iter()
        .skip(1)
        .fold(vec![nums[0]], |results, &num| {
            results
                .iter()
                .flat_map(|&res| match mode {
                    Concat::Off => vec![res + num, res * num],
                    Concat::On => vec![res + num, res * num, concat(res, num)],
                })
                .collect()
        })
        .contains(&test)
}

fn parse(puzzle: &str) -> Vec<(u64, Vec<u64>)> {
    puzzle
        .lines()
        .filter_map(|line| {
            let (test, nums) = line.split_once(':')?;
            let test = test.parse().ok()?;
            let nums = nums
                .split_whitespace()
                .filter_map(|num| num.parse().ok())
                .collect();

            Some((test, nums))
        })
        .collect()
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
