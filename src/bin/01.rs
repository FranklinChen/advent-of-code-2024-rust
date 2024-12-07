advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    // Read in one line at a time.
    // Each line has the form "n0 n1" where n0 and n1 are positive integers.
    // For each line, when it is read, parse it to n0 and n1.
    // Then append n0 into a Vec v0, and n1 into a Vec v1.
    // After all the lines are read, sort v0 and v1 in place from low to high.
    // Then parallel-iterate over v0 and v1, and for each pair of elements n0 and n1, calculate the (absolute value) distance between n0 and n1.
    // Sum all the distances, and return the sum.
    // (Optionally, could do sum in parallel with iterator.)
    let mut v0 = vec![];
    let mut v1 = vec![];
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let n0 = parts.next()?.parse::<u32>().ok()?;
        let n1 = parts.next()?.parse::<u32>().ok()?;
        v0.push(n0);
        v1.push(n1);
    }
    v0.sort_unstable();
    v1.sort_unstable();
    let sum = v0
        .iter()
        .zip(v1.iter())
        .map(|(n0, n1)| if n0 > n1 { n0 - n1 } else { n1 - n0 })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Read in one line at a time.
    // Each line has the form "n0 n1" where n0 and n1 are positive integers.
    // For each line, when it is read, parse it to n0 and n1.
    // Then append n0 into a Vec v.
    // But add n1 to a HashMap counts, which maps n1 to a count of how many times it has been seen.
    // After all the lines are read, iterate through v and compute a similarity score which is n0 times its count in counts, or 0 if not found.
    // Return the sum of all similarity scores using the iterator.
    let mut v = vec![];
    let mut counts = std::collections::HashMap::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let n0 = parts.next()?.parse::<u32>().ok()?;
        let n1 = parts.next()?.parse::<u32>().ok()?;
        v.push(n0);
        *counts.entry(n1).or_insert(0) += 1;
    }
    let sum = v.iter().map(|n0| n0 * counts.get(n0).unwrap_or(&0)).sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
