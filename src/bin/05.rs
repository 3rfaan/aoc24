use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let manual = Manual::from(input);
    let mut sum = 0;

    manual
        .updates
        .iter()
        .filter(|&update| manual.correct(&update))
        .for_each(|update| {
            let mid = update.len() / 2;
            sum += update[mid]
        });

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let manual = Manual::from(input);
    let mut sum = 0;

    manual
        .updates
        .iter()
        .filter(|&update| !manual.correct(update)) // Find incorrect updates
        .for_each(|update| {
            let sorted_update = manual.fix(update.clone());

            let mid = sorted_update.len() / 2;
            sum += sorted_update[mid];
        });

    Some(sum)
}

#[derive(Clone)]
struct Manual {
    rules: Vec<(u32, u32)>,
    updates: Vec<Vec<u32>>,
}

impl Manual {
    fn correct(&self, update: &Vec<u32>) -> bool {
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

    fn fix(&self, mut update: Vec<u32>) -> Vec<u32> {
        update.sort_by(|&x, &y| {
            if self.rules.contains(&(x, y)) {
                std::cmp::Ordering::Less
            } else if self.rules.contains(&(y, x)) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });

        update
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
                    .filter_map(|num| num.parse().ok())
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
