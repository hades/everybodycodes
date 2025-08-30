use std::collections::HashSet;

use petgraph::{
    algo::min_spanning_tree,
    data::{DataMap, FromElements},
    prelude::UnGraphMap,
    visit::{EdgeRef, IntoEdgeReferences, IntoNodeIdentifiers, depth_first_search},
};

fn manhattan(from: (usize, usize), to: (usize, usize)) -> usize {
    from.0.abs_diff(to.0) + from.1.abs_diff(to.1)
}

pub fn solve_part_1(input: &str) -> String {
    let mut graph: UnGraphMap<_, _> = UnGraphMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '*' {
                graph.add_node((x, y));
            }
        }
    }
    let all_nodes: Vec<_> = graph.node_identifiers().collect();
    for from_node in all_nodes.iter() {
        for to_node in all_nodes.iter() {
            if *to_node != *from_node {
                graph.add_edge(*from_node, *to_node, manhattan(*from_node, *to_node));
            }
        }
    }
    let constellation: UnGraphMap<_, _> = UnGraphMap::from_elements(min_spanning_tree(&graph));
    (constellation
        .edge_references()
        .map(|edge_ref| {
            let from = constellation.node_weight(edge_ref.source()).unwrap();
            let to = constellation.node_weight(edge_ref.target()).unwrap();
            manhattan(*from, *to)
        })
        .sum::<usize>()
        + constellation.node_identifiers().count())
    .to_string()
}

pub fn solve_part_2(input: &str) -> String {
    solve_part_1(input)
}

pub fn solve_part_3(input: &str) -> String {
    let mut graph: UnGraphMap<_, _> = UnGraphMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '*' {
                graph.add_node((x, y));
            }
        }
    }
    let all_nodes: Vec<_> = graph.node_identifiers().collect();
    for from_node in all_nodes.iter() {
        for to_node in all_nodes.iter() {
            if *to_node != *from_node {
                let distance = manhattan(*from_node, *to_node);
                if distance < 6 {
                    graph.add_edge(*from_node, *to_node, manhattan(*from_node, *to_node));
                }
            }
        }
    }
    let constellation: UnGraphMap<_, _> = UnGraphMap::from_elements(min_spanning_tree(&graph));
    let mut nodes_remaining: HashSet<_> = HashSet::from_iter(all_nodes.into_iter());
    let mut constellation_sizes = vec![];
    while let Some(next_starting_node) = nodes_remaining.iter().next().cloned() {
        let mut node_count = 0;
        let mut distances_sum = 0;
        depth_first_search(&constellation, Some(next_starting_node), |event| {
            match event {
                petgraph::visit::DfsEvent::Discover(node, _) => {
                    nodes_remaining.remove(&node);
                    node_count += 1;
                }
                petgraph::visit::DfsEvent::TreeEdge(from_node, to_node) => {
                    distances_sum += manhattan(from_node, to_node);
                }
                _ => {}
            }
            ()
        });
        constellation_sizes.push(node_count + distances_sum);
    }
    constellation_sizes.sort();
    constellation_sizes[constellation_sizes.len() - 3..]
        .iter()
        .product::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_part_1() {
        assert_eq!(
            "16",
            solve_part_1(
                "*...*
..*..
.....
.....
*.*.."
            )
        );
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            "15624",
            solve_part_3(
                ".......................................
..*.......*...*.....*...*......**.**...
....*.................*.......*..*..*..
..*.........*.......*...*.....*.....*..
......................*........*...*...
..*.*.....*...*.....*...*........*.....
......................................."
            )
        );
    }
}
