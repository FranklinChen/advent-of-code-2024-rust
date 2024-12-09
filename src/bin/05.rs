advent_of_code::solution!(5);

// For creating a graph of constraints.
use std::collections::HashMap;

type Page = u8;

type Graph = HashMap<Page, Vec<Page>>;

// Lines of the form:
//
// 47|53
// 97|13
// 97|61
fn parse_graph(input: &str) -> Graph {
    let mut graph = Graph::new();

    for line in input.lines() {
        let mut parts = line.split('|');
        let src = parts.next().unwrap().parse::<Page>().unwrap();
        let dest = parts.next().unwrap().parse::<Page>().unwrap();

        graph.entry(src).or_default().push(dest);
    }

    graph
}

// Return whether the pages are in a valid order according
// to the constraints encoded in the graph that involve only
// the pages in the list.
//
// Just run through the whole graph.
fn is_valid_order(graph: &Graph, pages: &[Page]) -> bool {
    // Page -> index into pages.
    let position: HashMap<Page, usize> = pages
        .iter()
        .enumerate()
        .map(|(i, &page)| (page, i))
        .collect();

    for (&src, dests) in graph {
        // Ignore constraints if src is not in pages.
        if let Some(&src_pos) = position.get(&src) {
            for &dest in dests {
                // Ignore constraints if dest is not in pages.
                if let Some(&dest_pos) = position.get(&dest) {
                    // Look for a violation.
                    if src_pos >= dest_pos {
                        return false;
                    }
                }
            }
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sections = input.split("\n\n");
    let graph_section = sections.next().unwrap();
    let rules_section = sections.next().unwrap();

    let graph = parse_graph(graph_section);

    // Return sum of the middle page of each valid rule.
    Some(
        rules_section
            .lines()
            .filter_map(|line| {
                let pages = line
                    .split(',')
                    .map(|page| page.parse::<Page>().unwrap())
                    .collect::<Vec<_>>();

                if is_valid_order(&graph, &pages) {
                    let middle = pages[pages.len() / 2];
                    Some(middle as u32)
                } else {
                    None
                }
            })
            .sum(),
    )
}

// Reorder pages to satisfy the graph constraints, ignoring again
// all constraints that don't involve the pages in the list.
//
// Perform a topological sort of pages in a simplistic way,
// good enough.
fn fix_pages(graph: &Graph, pages: &[Page]) -> Vec<Page> {
    // Track the in-degree of each page. Also serves as a way to
    // check if a page is in the list.
    let mut in_degree: HashMap<Page, usize> = HashMap::new();

    // Queue of pages with in-degree 0 that need to be handled.
    let mut queue: Vec<Page> = vec![];

    // Final sorted pages to return.
    let mut sorted = vec![];

    // Only care about pages in the list.
    for &page in pages {
        in_degree.insert(page, 0);
    }

    // Walk through graph, updating dest in-degrees.
    for (&src, dests) in graph {
        if in_degree.contains_key(&src) {
            for &dest in dests {
                if let Some(degree) = in_degree.get_mut(&dest) {
                    *degree += 1;
                }
            }
        }
    }

    // Queue up all pages with in-degree 0 because nothing comes
    // before them.
    for &page in pages {
        if in_degree[&page] == 0 {
            queue.push(page);
        }
    }

    while let Some(page) = queue.pop() {
        // Pull off a page with in-degree 0 and save it.
        sorted.push(page);

        if let Some(dests) = graph.get(&page) {
            for &dest in dests {
                // Decrement neighbor's in-degree.
                if let Some(degree) = in_degree.get_mut(&dest) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push(dest);
                    }
                }
            }
        }
    }

    sorted
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sections = input.split("\n\n");
    let graph_section = sections.next().unwrap();
    let rules_section = sections.next().unwrap();

    let graph = parse_graph(graph_section);

    // Return sum of the middle page of each fixed invalid rule.
    Some(
        rules_section
            .lines()
            .filter_map(|line| {
                let pages = line
                    .split(',')
                    .map(|page| page.parse::<Page>().unwrap())
                    .collect::<Vec<_>>();

                if !is_valid_order(&graph, &pages) {
                    let fixed_pages = fix_pages(&graph, &pages);
                    let middle = fixed_pages[fixed_pages.len() / 2];
                    Some(middle as u32)
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
