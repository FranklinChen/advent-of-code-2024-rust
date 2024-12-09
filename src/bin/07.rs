advent_of_code::solution!(7);

/// "45: 14 4 8"
struct Line {
    target: u64,
    xs: Vec<u64>,
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let target = parts.next().unwrap().parse().unwrap();
            let xs = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            Line { target, xs }
        })
        .collect()
}

/// Return true if the target can be formed from the xs using
/// left-to-right evaluation with + and *.  Work backwards,
/// knowing that the inverse of * is hard to achieve with
/// integers.
///
/// Recursive solution.
fn can_equal_target(target: u64, xs: &[u64]) -> bool {
    // Base case.
    if xs.is_empty() {
        return target == 0;
    }
    if target == 0 {
        // There are still xs left.
        return false;
    }

    let last = xs.len() - 1;

    // If there is only one number remaining, it must be the target.
    if last == 0 {
        return xs[0] == target;
    }

    // See if * could be the final operation.
    let last_x = xs[last];
    if target % last_x == 0 {
        // Try either multiplication or addition.
        can_equal_target(target / last_x, &xs[0..last])
            || (target >= last_x && can_equal_target(target - last_x, &xs[0..last]))
    } else {
        target >= last_x && can_equal_target(target - last_x, &xs[0..last])
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    // Return the sum of all targets that can be formed from the xs
    // using left-to-right evaluation with + and *.
    let lines = parse_input(input);
    Some(
        lines
            .iter()
            .filter_map(|line| {
                if can_equal_target(line.target, &line.xs) {
                    Some(line.target)
                } else {
                    None
                }
            })
            .sum()
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
