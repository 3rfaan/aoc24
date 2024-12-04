advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    // Two vectors, first storing left-hand side values,second storing right-hand side values
    let (mut lhs, mut rhs) = (Vec::new(), Vec::new());

    // On every line split at whitespace, parse ints, then push first int to lhs vec and second int
    // to rhs vec
    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        (lhs.push(nums[0]), rhs.push(nums[1]));
    }

    (lhs.sort(), rhs.sort());

    // Subtract int at index i in rhs vec from int at index i in lhs vec, take absolute value and
    // then sum with difference from previous iteration, in the end we store the sum of all
    // differences in variable sum
    let sum = lhs.iter().zip(rhs.iter()).map(|(a, b)| (a - b).abs()).sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let (mut lhs, mut rhs) = (Vec::new(), Vec::new());

    // Same as above
    for line in input.lines() {
        let nums: Vec<u32> = line
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        (lhs.push(nums[0]), rhs.push(nums[1]));
    }

    for num_l in lhs {
        sum += rhs.iter().filter(|&num_r| *num_r == num_l).sum::<u32>();
    }

    Some(sum)
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
