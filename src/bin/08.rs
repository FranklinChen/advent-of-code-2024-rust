advent_of_code::solution!(8);

use std::collections::HashMap;
use std::collections::HashSet;

type Location = (usize, usize);

struct Grid {
    bytes: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
    marks: HashMap<u8, Vec<Location>>,
}

/// A valid antipode within the grid, formed using any two
/// non-dots in the grid with the same mark, by going to either
/// side along a line, using the same distance away from each
/// pair of non-dots, in the opposite directions, while remaining
/// on the bytes grid.
struct Antipodes {
    antipodes: HashMap<u8, Vec<Location>>,
}

impl Antipodes {
    /// For each mark type, find all antipodes of the type.
    fn new(grid: &Grid) -> Self {
        let mut antipodes = HashMap::new();

        for (mark, locations) in &grid.marks {
            let mut antipode_list = vec![];

            // For each pair of locations that is a non-dot.
            for i in 0..locations.len() {
                for j in (i + 1)..locations.len() {
                    // Call the two points point 1 and 2.
                    // We want to look for possible point 0 and 3.
                    // Make sure to do arithmetic without overflow.
                    let (x1, y1) = locations[i];
                    let (x2, y2) = locations[j];

                    // Get the differences in coordinates in both
                    // directions, using signed arithmetic.
                    let dx = x2 as isize - x1 as isize;
                    let dy = y2 as isize - y1 as isize;

                    // Try to get antipode 0.
                    let x0 = x1 as isize - dx;
                    let y0 = y1 as isize - dy;

                    // Try to get antipode 3.
                    let x3 = x2 as isize + dx;
                    let y3 = y2 as isize + dy;

                    // Check if the antipodes are within the grid.
                    // Push without worrying about duplicates for now.
                    if x0 >= 0 && x0 < grid.rows as isize && y0 >= 0 && y0 < grid.cols as isize {
                        antipode_list.push((x0 as usize, y0 as usize));
                    }

                    if x3 >= 0 && x3 < grid.rows as isize && y3 >= 0 && y3 < grid.cols as isize {
                        antipode_list.push((x3 as usize, y3 as usize));
                    }
                }
            }

            antipodes.insert(*mark, antipode_list);
        }

        Antipodes { antipodes }
    }

    /// Just like new, except that we create a lot more antipodes:
    /// All grid locations evenly spaced on the line extending beyond
    /// both ends of any two original marks, including those original
    /// marks, is an antipode. So we can't just look for two anymore,
    /// but must loop in both directions till off the grid.
    fn new_on_line(grid: &Grid) -> Self {
        let mut antipodes = HashMap::new();

        for (mark, locations) in &grid.marks {
            let mut antipode_list = vec![];

            for i in 0..locations.len() {
                for j in (i + 1)..locations.len() {
                    let (x1, y1) = locations[i];
                    let (x2, y2) = locations[j];

                    let dx = x2 as isize - x1 as isize;
                    let dy = y2 as isize - y1 as isize;

                    // Try to get antipode 0.
                    let mut x0 = x1 as isize - dx;
                    let mut y0 = y1 as isize - dy;

                    // Try to get antipode 3.
                    let mut x3 = x2 as isize + dx;
                    let mut y3 = y2 as isize + dy;

                    // Push the original marks.
                    antipode_list.push((x1, y1));
                    antipode_list.push((x2, y2));

                    // Push all antipodes on the line.
                    while x0 >= 0 && x0 < grid.rows as isize && y0 >= 0 && y0 < grid.cols as isize {
                        antipode_list.push((x0 as usize, y0 as usize));
                        x0 -= dx;
                        y0 -= dy;
                    }

                    while x3 >= 0 && x3 < grid.rows as isize && y3 >= 0 && y3 < grid.cols as isize {
                        antipode_list.push((x3 as usize, y3 as usize));
                        x3 += dx;
                        y3 += dy;
                    }
                }
            }

            antipodes.insert(*mark, antipode_list);
        }

        Antipodes { antipodes }
    }

    fn unique_locations(self) -> HashSet<Location> {
        self.antipodes
            .into_iter()
            .flat_map(|(_, locations)| locations)
            .collect()
    }
}

impl Grid {
    /// Read in a row of lines, each ASCII character of which is either
    /// . (a dot) or a digit or lower ASCII letter or upper ASCII
    /// letter. If the character is not a dot, store its grid location in
    /// a map. Return the grid matrix (using bytes) as well as number of
    /// rows, number of columns, and the map of all non-dot locations
    /// keyed by the on-dot.
    fn new(input: &str) -> Self {
        let mut bytes = vec![];
        let mut marks = HashMap::new();

        let mut rows = 0;
        for line in input.lines() {
            let mut row = vec![];

            for (col, byte) in line.bytes().enumerate() {
                row.push(byte);

                if byte != b'.' {
                    marks.entry(byte).or_insert_with(Vec::new).push((rows, col));
                }
            }

            bytes.push(row);
            rows += 1;
        }

        let cols = bytes[0].len();

        Grid {
            bytes,
            rows,
            cols,
            marks,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::new(input);
    let antipodes = Antipodes::new(&grid);
    let unique_locations = antipodes.unique_locations();

    Some(unique_locations.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::new(input);
    let antipodes = Antipodes::new_on_line(&grid);
    let unique_locations = antipodes.unique_locations();

    Some(unique_locations.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
