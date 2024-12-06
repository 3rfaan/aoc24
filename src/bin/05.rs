use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let manual = Manual::from(input);
    let mut sum = 0;

    manual
        .updates
        .iter()
        .filter(|&update| manual.is_correct_order(&update))
        .for_each(|update| {
            let mid = update.len() / 2;
            sum += update[mid]
        });

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

struct Manual {
    rules: Vec<(u32, u32)>,
    updates: Vec<Vec<u32>>,
}

impl Manual {
    fn is_correct_order(&self, update: &Vec<u32>) -> bool {
        let map: HashMap<u32, usize> = update
            .iter()
            .enumerate()
            .map(|(idx, &page)| (page, idx))
            .collect();

        self.rules
            .iter()
            .all(|(x, y)| match (map.get(x), map.get(y)) {
                (Some(&x), Some(&y)) => x < y,
                _ => true,
            })
    }
}

impl From<&str> for Manual {
    fn from(input: &str) -> Self {
        let manual: Vec<&str> = input.split("\n\n").collect();
        let (rules, updates) = (manual[0], manual[1]);

        let rules: Vec<(u32, u32)> = rules
            .lines()
            .map(|rule| {
                let parts: Vec<u32> = rule.split('|').filter_map(|num| num.parse().ok()).collect();
                (parts[0], parts[1])
            })
            .collect();

        let updates: Vec<Vec<u32>> = updates
            .lines()
            .map(|update| {
                update
                    .split(',')
                    .filter_map(|num| num.parse::<u32>().ok())
                    .collect()
            })
            .collect();

        Self { rules, updates }
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
