advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    // Amount of safe reports
    let mut safe_reports = 0;

    // Parsing every line of input into vector of u32 ints and checking if they are safe
    for line in input.lines() {
        let report: Vec<u32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Increment safe_reports counter if report (line) is safe
        if is_safe(&report) {
            safe_reports += 1;
        }
    }

    Some(safe_reports)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Amount of safe reports
    let mut safe_reports = 0;

    // Parsing every line of input into vector of u32 ints and checking if they are safe
    for line in input.lines() {
        let report: Vec<u32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Increment safe_reports counter if report (line) is safe
        if is_safe(&report) {
            safe_reports += 1;
            continue;
        }

        // If report is not safe then we check if problem_dampener can make it safe
        problem_dampener(&report, &mut safe_reports);
    }

    Some(safe_reports)
}

fn is_safe(report: &[u32]) -> bool {
    // Check if report (line in input) is increasing and difference between adjacent levels is
    // between 1 and 3 (included)
    let is_increasing = report
        .windows(2)
        .all(|lvl| lvl[0] <= lvl[1] && (1..=3).contains(&(lvl[1] - lvl[0])));
    // Check if report (line in input) is decreasing and difference between adjacent levels is
    // between 1 and 3 (included)
    let is_decreasing = report
        .windows(2)
        .all(|lvl| lvl[0] >= lvl[1] && (1..=3).contains(&(lvl[0] - lvl[1])));

    // Return true if one of the two conditions is met otherwise false
    is_increasing || is_decreasing
}

fn problem_dampener(report: &[u32], safe_reports: &mut u32) {
    // Clone report vector to avoid borrow checking problems as we are mutating and sending vector
    // to `is_safe` function
    let mut report = report.to_vec();
    for (i, _) in report.clone().iter().enumerate() {
        // Remove in each iteration one level at index i in report and trying if the report becomes safe
        let removed = report.remove(i);

        // If we found a safe report after removing one element at index i, we increase counter of
        // safe_reports and we insert the element back into the report vector
        if is_safe(&report) {
            *safe_reports += 1;
            report.insert(i, removed);
            return;
        }
        // If we didn't find a safe report we also restore the vector
        report.insert(i, removed);
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

    #[test]
    fn test_is_safe_helper_fn() {
        let safe_report = [7, 6, 4, 2, 1];
        let unsafe_report = [1, 3, 2, 4, 5];

        assert!(is_safe(&safe_report));
        assert!(!is_safe(&unsafe_report));
    }
}
