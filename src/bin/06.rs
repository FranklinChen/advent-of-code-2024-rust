advent_of_code::solution!(6);

use std::collections::HashSet;

type Location = (usize, usize);

type Direction = (i32, i32);

type Visited = HashSet<(Location, Direction)>;

type Grid = Vec<Vec<bool>>;

type Locations = HashSet<Location>;

fn parse_to_grid(input: &str) -> (Grid, Location) {
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

    (grid, guard_pos)
}

pub fn part_one(input: &str) -> Option<u32> {
    // Parse the input into a grid and the guard's position.
    let (grid, guard_pos) = parse_to_grid(input);

    // Run the original algorithm, returning all location/orientation
    // pairs.
    let (is_infinite_loop, visited) = run(&grid, guard_pos);

    if is_infinite_loop {
        return None;
    }

    // Get the visited locations.
    let locations: Locations = visited.iter().map(|(pos, _)| *pos).collect();

    Some(locations.len() as u32)
}

fn run(grid: &[Vec<bool>], mut guard_pos: Location) -> (bool, Visited) {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let mut guard_dir: (i32, i32) = (-1, 0); // Always start facing up.
    let mut visited: Visited = HashSet::new();

    // Mark the initial state as visited.
    visited.insert((guard_pos, guard_dir));

    loop {
        let (i, j) = guard_pos;
        let (di, dj) = guard_dir;

        // Calculate next position.
        let next_pos = (i as i32 + di, j as i32 + dj);

        // Exit if the guard moves out of bounds.
        if next_pos.0 < 0 || next_pos.1 < 0 || next_pos.0 >= rows || next_pos.1 >= cols {
            return (false, visited);
        }

        let next_pos = (next_pos.0 as usize, next_pos.1 as usize);

        if grid[next_pos.0][next_pos.1] {
            // If facing an obstacle, turn right.
            guard_dir = (dj, -di);
        } else {
            // Check if the state (position and direction) is already visited.
            if !visited.insert((next_pos, guard_dir)) {
                // Infinite loop detected.
                return (true, visited);
            }
            // Move the guard forward.
            guard_pos = next_pos;
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    // Parse the input into a grid and the guard's position.
    let (mut grid, guard_pos) = parse_to_grid(input);

    // Run the original algorithm, returning all location/orientation
    // pairs.
    let (is_infinite_loop, visited) = run(&grid, guard_pos);

    if is_infinite_loop {
        return None;
    }

    // For each position along the visited path, except for the
    // initial one, try placing an obstacle there to form a modified
    // grid.  Rerun and check if an infinite loop is detected.  Count
    // the number of obstacle placements that result in an infinite
    // loop.
    let mut num_infinite_loops = 0;

    // Get the set of all locations (usize, usize)
    // in the visited path other than
    // the initial guard one.
    let locations: Locations = visited
        .iter()
        .filter_map(|(pos, _)| if *pos == guard_pos { None } else { Some(*pos) })
        .collect();

    for &(i, j) in locations.iter() {
        // Temporarily modify the grid.
        // If we were parallelizing, we would need to clone the grid.
        grid[i][j] = true;

        let (is_infinite_loop, _) = run(&grid, guard_pos);

        if is_infinite_loop {
            num_infinite_loops += 1;
        }

        grid[i][j] = false;
    }

    Some(num_infinite_loops)
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
        assert_eq!(result, Some(6));
    }
}
