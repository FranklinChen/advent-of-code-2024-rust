advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    // Read in the grid of ASCII characters which are either
    // . (open)
    // # (obstacle)
    // ^ (initial position of guard, facing up in grid)
    //
    // The grid indicates whether a position is an obstacle.
    let mut grid = Vec::new();

    // The initial position of the guard.
    let mut guard_pos = (0, 0);
    let mut guard_found = false;

    // Parse the input into a grid, while also finding the guard's
    // position and the number of rows and columns. Use bytes.
    for (i, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (j, c) in line.bytes().enumerate() {
            match c {
                b'.' => row.push(false),
                b'#' => row.push(true),
                b'^' => {
                    guard_pos = (i, j);
                    guard_found = true;
                    row.push(false);
                }
                _ => panic!("Invalid character in input"),
            }
        }
        grid.push(row);
    }

    // Make sure guard was found.
    if !guard_found {
        panic!("Guard not found in input");
    }

    // Number of rows and columns in the grid.
    let rows = grid.len();
    let cols = grid[0].len();

    let last_row_index = rows - 1;
    let last_col_index = cols - 1;

    // Keep track of locations already visited.
    let mut visited = vec![vec![false; cols]; rows];
    visited[guard_pos.0][guard_pos.1] = true;

    // Always start facing up.
    let mut guard_dir: (i32, i32) = (-1, 0);

    // Number of positions of guard before exiting the grid.
    let mut num_positions = 1;

    // Keep moving the guard until it exits the grid. The rules:
    // - Start by moving up from the position of the single ^.
    // - If currently at the edge and moving off it, exit.
    // - If facing . then step there and continue on.
    // - If facing # then turn right and take the next step.

    // Do not check for infinite loop at this point.
    loop {
        // Get the current position of the guard.
        let (i, j) = guard_pos;

        // If the guard is at the edge of the grid, exit.
        if i == 0 || i == last_row_index || j == 0 || j == last_col_index {
            break;
        }

        // If the guard is facing an obstacle, turn right.
        let (di, dj) = guard_dir;
        let next_pos = ((i as i32 + di) as usize, (j as i32 + dj) as usize);
        let (ni, nj) = next_pos;

        if grid[ni][nj] {
            guard_dir = (dj, -di);
        }

        let (di, dj) = guard_dir;
        let next_pos = ((i as i32 + di) as usize, (j as i32 + dj) as usize);
        let (ni, nj) = next_pos;

        // Check if already visited.
        if !visited[ni][nj] {
            // Mark the position as visited.
            visited[ni][nj] = true;
            num_positions += 1;
        }

        // Move the guard in the direction it is facing.
        guard_pos = next_pos;
    }

    Some(num_positions)
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
