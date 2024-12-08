advent_of_code::solution!(3);

use std::sync::LazyLock;
use regex::Regex;

static RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"mul\((\d+),(\d+)\)").expect("Failed to compile regex")
});

pub fn part_one(input: &str) -> Option<u32> {
    // Extract info from the entire input (not breaking up into lines):
    // Find all "mul(X,Y)" where X and Y are 1-3 digit numbers.
    // Use regex.
    // Parse each instance into a tuple of (X, Y) as u32 each.
    // Sum all of the multiplications.
    // Return the sum.
    let sum: u32 = RE
        .captures_iter(input)
        .map(|cap| {
            let x: u32 = cap[1].parse().unwrap();
            let y: u32 = cap[2].parse().unwrap();
            x * y
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
