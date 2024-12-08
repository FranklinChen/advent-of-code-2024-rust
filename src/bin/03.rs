advent_of_code::solution!(3);

use regex::Regex;
use std::sync::LazyLock;

static RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").expect("Failed to compile regex"));

static RE_PART2: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))").expect("Failed to compile regex")
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
    // Similar to part 1, except that we should not only scan for
    // "mul(X,Y) but also for "do()" and "don't()" literal
    // instructions.
    //
    // When interpreting a token stream of these three types of
    // instructions, maintain state such that initially, multiplications
    // are enabled, but whenever seeing "don't()" they are disabled,
    // and whenever seeing "do()" they are enabled.
    // Sum all of the enabled multiplications.
    // Return the sum.
    let mut is_enabled = true; // Initially, multiplications are enabled.
    let mut sum = 0;

    for cap in RE_PART2.captures_iter(input) {
        if cap.get(1).is_some() {
            if is_enabled {
                let x: u32 = cap[2].parse().unwrap();
                let y: u32 = cap[3].parse().unwrap();
                sum += x * y;
            }
        } else if cap.get(4).is_some() {
            is_enabled = true;
        } else if cap.get(5).is_some() {
            is_enabled = false;
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
