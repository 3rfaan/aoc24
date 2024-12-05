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
        .filter_map(|cap| {
            let op1 = cap["op1"].parse::<u32>().ok()?;
            let op2 = cap["op2"].parse::<u32>().ok()?;
            Some(op1 * op2)
        })
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

    let mut enabled = true;

    let sum = re
        .captures_iter(input)
        .filter_map(|cap| {
            let r#match = &cap[0];

            match (r#match, enabled) {
                ("do()", _) => {
                    enabled = true;
                    None
                }
                ("don't()", _) => {
                    enabled = false;
                    None
                }
                (_, true) => {
                    let op1 = cap["op1"].parse::<u32>().ok()?;
                    let op2 = cap["op2"].parse::<u32>().ok()?;
                    Some(op1 * op2)
                }
                _ => None,
            }
        })
        .sum();

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
