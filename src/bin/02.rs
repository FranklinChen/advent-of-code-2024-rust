advent_of_code::solution!(2);

// Check that level0 and level1 are actually increasing (or
// decreasing), never equal.  Then check whether the increase (or
// decrease) is at least 1 and at most 3.
fn safe_distance(increasing: bool, level0: u32, level1: u32) -> bool {
    if increasing {
        level0 < level1 && level1 - level0 <= 3
    } else {
        level0 > level1 && level0 - level1 <= 3
    }
}

// Return whether a "report" is safe.
// A report is an ordered slice of integers called "levels".
// A report is safe if:
// - The levels are either all increasing or all decreasing (none equal).
// - Any two adjacent levels differ (absolute value) by at least one and at most three.
// Assume that there are at least two levels.
fn report_is_safe(levels: &[u32]) -> bool {
    // Check if the levels are all increasing or all decreasing, by
    // first checking if the first two levels are increasing or
    // decreasing, and then checking if the rest of the levels are
    // increasing or decreasing, using the correct sign for checking
    // difference between levels being >= 1 and <= 3.
    let increasing = levels[0] < levels[1];

    for i in 0..levels.len() - 1 {
        if !safe_distance(increasing, levels[i], levels[i + 1]) {
            return false;
        }
    }

    true
}

// A report is tolerably safe it is either safe, as is, or
// if it would be safe if one level were removed.
fn report_is_tolerably_safe(levels: &[u32]) -> bool {
    // Do the most naive way, super inefficient.
    if report_is_safe(levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut levels = levels.to_vec();
        levels.remove(i);
        if report_is_safe(&levels) {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    // Read one line at a time.

    // For each line, parse into whitespace-delimited integers
    // ("levels") into a vector that is a "report".

    // For each report, determine whether it is safe.
    // Return number of safe reports by summing over iterator.
    Some(
        input
            .lines()
            .filter_map(|line| {
                let levels: Vec<u32> = line
                    .split_whitespace()
                    .filter_map(|num| num.parse::<u32>().ok())
                    .collect();
                if report_is_safe(&levels) {
                    Some(1)
                } else {
                    None
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    // Read one line at a time.

    // For each line, parse into whitespace-delimited integers
    // ("levels") into a vector that is a "report".

    // For each report, determine whether it is tolerably safe.
    // Return number of safe reports by summing over iterator.
    Some(
        input
            .lines()
            .filter_map(|line| {
                let levels: Vec<u32> = line
                    .split_whitespace()
                    .filter_map(|num| num.parse::<u32>().ok())
                    .collect();
                if report_is_tolerably_safe(&levels) {
                    Some(1)
                } else {
                    None
                }
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
