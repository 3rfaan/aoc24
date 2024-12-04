advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(
        r"(?x)
            mul                     # match literal `mul`
            \(
            (?<op1>[0-9]{1,3})      # first operand named capture group
            ,
            (?<op2>[0-9]{1,3})      # second operand named capture group
            \)
        ",
    )
    .unwrap();

    let sum = re
        .captures_iter(input)
        .map(|cap| cap["op1"].parse::<u32>().unwrap() * cap["op2"].parse::<u32>().unwrap())
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(
        r"(?x)
            (?<do>do\(\))           # match literal `do()`
            |(?<dont>don't\(\))     # match literal `don't()`
            |mul                    # match literal `mul`
            \(
            (?<op1>[0-9]{1,3})      # first operand
            ,
            (?<op2>[0-9]{1,3})      # second operand
            \)
        ",
    )
    .unwrap();

    let mut sum = 0;
    let mut enabled = true;

    for cap in re.captures_iter(input) {
        if let Some(_) = cap.name("do") {
            enabled = true;
        } else if let Some(_) = cap.name("dont") {
            enabled = false;
        } else if let (Some(op1), Some(op2)) = (cap.name("op1"), cap.name("op2")) {
            if enabled {
                let op1: u32 = op1.as_str().parse().unwrap();
                let op2: u32 = op2.as_str().parse().unwrap();

                sum += op1 * op2;
            }
        }
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
