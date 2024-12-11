advent_of_code::solution!(10);

const DEBUG: bool = false;

// In order to link src to dest only if dest == src + 1.
use petgraph::graphmap::DiGraphMap;

use petgraph::algo::simple_paths::all_simple_paths;

type Location = (u8, u8);

struct Topo {
    heights: Vec<Vec<u8>>,
    rows: u8,
    cols: u8,
    heads: Vec<Location>,
    nines: Vec<Location>,
    graph: DiGraphMap<Location, ()>,
}

impl Topo {
    /// Read in lines that create a grid of bytes that are digits
    /// '0' to '9'. After creating the grid, create a graph by
    /// checking each cell to create possible directed edges to its
    /// (maximum four) neighbors.
    fn new(input: &str) -> Self {
        let heights: Vec<Vec<u8>> = input
            .lines()
            .map(|line| line.as_bytes().iter().map(|&b| b - b'0').collect())
            .collect();
        let rows = heights.len() as u8;
        let cols = heights[0].len() as u8;

        let mut graph = DiGraphMap::new();
        let mut heads = vec![];
        let mut nines = vec![];

        for i in 0u8..rows {
            for j in 0u8..cols {
                let loc = (i, j);

                // Try to link from four neighbors to current cell.
                // Note that nothing precedes 0.
                if heights[i as usize][j as usize] == 0 {
                    heads.push(loc);
                    continue;
                } else if heights[i as usize][j as usize] == 9 {
                    nines.push(loc);
                }

                let lower_height = heights[i as usize][j as usize] - 1;

                if i > 0 && heights[i as usize - 1][j as usize] == lower_height {
                    graph.add_edge((i - 1, j), loc, ());
                }
                if j > 0 && heights[i as usize][j as usize - 1] == lower_height {
                    graph.add_edge((i, j - 1), loc, ());
                }
                if i < rows - 1 && heights[i as usize + 1][j as usize] == lower_height {
                    graph.add_edge((i + 1, j), loc, ());
                }
                if j < cols - 1 && heights[i as usize][j as usize + 1] == lower_height {
                    graph.add_edge((i, j + 1), loc, ());
                }
            }
        }

        Self {
            heights,
            rows,
            cols,
            heads,
            nines,
            graph,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let topo = Topo::new(input);
    Some(
        topo.heads
            .iter()
            .map(|head| {
                let score = topo
                    .nines
                    .iter()
                    .map(|nine| {
                        if all_simple_paths::<Vec<_>, _>(&topo.graph, *head, *nine, 1, None)
                            .next()
                            .is_some()
                        {
                            1
                        } else {
                            0
                        }
                    })
                    .sum::<usize>();
                if DEBUG {
                    eprintln!("{head:?} to all nines: {score}");
                }
                score
            })
            .sum::<usize>() as u32,
    )
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
