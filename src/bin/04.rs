advent_of_code::solution!(4);

use aho_corasick::AhoCorasick;

fn extract_all_sequences(input: &str) -> Vec<Vec<u8>> {
    let mut sequences = Vec::new();

    let lines: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let num_rows = lines.len();
    let num_cols = lines[0].len();

    // Extract rows.
    sequences.extend(lines.iter().map(|line| line.to_vec()));

    // Extract columns.
    for col_idx in 0..num_cols {
        let col_bytes: Vec<u8> = lines.iter().map(|line| line[col_idx]).collect();
        sequences.push(col_bytes);
    }

    // Extract left-right top-bottom diagonals toward bottom left.
    for diag_idx in 0..num_rows {
        let diag_bytes: Vec<u8> = (0..num_rows - diag_idx)
            .map(|i| lines[diag_idx + i][i])
            .collect();
        sequences.push(diag_bytes);
    }

    // Extract left-right top-bottom diagonals toward top right.
    for diag_idx in 1..num_cols {
        let diag_bytes: Vec<u8> = (0..num_cols - diag_idx)
            .map(|i| lines[i][diag_idx + i])
            .collect();
        sequences.push(diag_bytes);
    }

    // Extract right-left bottom-top diagonals from top left.
    for diag_idx in 0..num_rows {
        let diag_bytes: Vec<u8> = (0..diag_idx + 1).map(|i| lines[diag_idx - i][i]).collect();
        sequences.push(diag_bytes);
    }

    // Extract right-left bottom-top diagonals to bottom right.
    for diag_idx in 1..num_cols {
        let diag_bytes: Vec<u8> = (0..num_cols - diag_idx)
            .map(|i| lines[num_rows - 1 - i][diag_idx + i])
            .collect();
        sequences.push(diag_bytes);
    }

    // Debug output sequences by turning them into strings.
    if false {
        for sequence in sequences.iter() {
            dbg!(String::from_utf8(sequence.clone()).unwrap());
        }
    }

    sequences
}

pub fn part_one(input: &str) -> Option<u32> {
    // Search forward and backward simultaneously.
    let patterns = &[b"XMAS", b"SAMX"];
    let ac = AhoCorasick::new(patterns).unwrap();

    let sequences = extract_all_sequences(input);
    Some(
        sequences
            .iter()
            .map(|haystack| ac.find_overlapping_iter(haystack).count() as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let num_rows = lines.len();
    let num_cols = lines[0].len();

    // Find all X-MAS patterns in the 2d grid. Define X-MAS
    // as a 3x3 grid with a pattern of two diagonals spelling out MAS
    // in any direction, e.g.
    //
    // M.S
    // .A.
    // M.S
    //
    // where the . are any character.
    let mut sum = 0;
    for row_idx in 0..num_rows - 2 {
        for col_idx in 0..num_cols - 2 {
            // First check the middle of the grid for an A.
            // Then check the top-left to bottom-right diagonal for two
            // possible directions of the M and S. Then check
            // the top-right to bottom-left diagonal for the other
            // two possible directions of the M and S.
            if lines[row_idx + 1][col_idx + 1] == b'A'
                && (lines[row_idx][col_idx] == b'M' && lines[row_idx + 2][col_idx + 2] == b'S'
                    || lines[row_idx + 2][col_idx + 2] == b'M' && lines[row_idx][col_idx] == b'S')
                && (lines[row_idx][col_idx + 2] == b'M' && lines[row_idx + 2][col_idx] == b'S'
                    || lines[row_idx + 2][col_idx] == b'M' && lines[row_idx][col_idx + 2] == b'S')
            {
                sum += 1;

                // Debug: look at the location of the A.
                if false {
                    dbg!((row_idx + 1, col_idx + 1));
                }
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
