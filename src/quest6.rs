use std::collections::HashMap;

use itertools::Itertools;
use log::debug;
use petgraph::{
    Graph,
    visit::{DfsEvent::TreeEdge, depth_first_search},
};

pub fn solve_part_1(input: &str) -> String {
    let nodes: Vec<(&str, Vec<&str>)> = input
        .lines()
        .map(|line| {
            let mut splits = line.splitn(2, ":");
            let node = splits.next().unwrap();
            let edges: Vec<&str> = splits.next().unwrap().split(",").collect();
            (node, edges)
        })
        .collect();
    let mut graph: petgraph::Graph<&str, (), petgraph::Directed, usize> = Graph::default();
    let mut node_to_idx = HashMap::new();
    nodes.iter().for_each(|node| {
        node_to_idx.insert(node.0, graph.add_node(node.0));
    });
    nodes.iter().for_each(|node| {
        debug!("{:?}", node);
        node.1.iter().for_each(|to_node| {
            let to_node_idx = match *to_node {
                "@" => graph.add_node("@"),
                _ => match node_to_idx.get(to_node) {
                    Some(idx) => *idx,
                    None => {
                        let idx = graph.add_node(to_node);
                        node_to_idx.insert(to_node, idx);
                        idx
                    }
                },
            };
            graph.add_edge(node_to_idx[node.0], to_node_idx, ());
        });
    });
    let mut node_paths = HashMap::new();
    node_paths.insert(node_to_idx["RR"], "RR".to_string());
    depth_first_search(&graph, [node_to_idx["RR"]], |event| {
        if let TreeEdge(from, to) = event {
            let path = format!("{}{}", node_paths[&from], graph[to]);
            node_paths.insert(to, path);
        }
    });
    let unique_len = node_paths
        .iter()
        .filter(|p| p.1.ends_with("@"))
        .counts_by(|p| p.1.len())
        .into_iter()
        .filter(|(_, counts)| *counts == 1)
        .exactly_one()
        .unwrap()
        .0;
    node_paths
        .into_values()
        .filter(|p| p.ends_with("@") && p.len() == unique_len)
        .exactly_one()
        .unwrap()
}

pub fn solve_part_2(input: &str) -> String {
    "".to_string()
}

pub fn solve_part_3(input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!(
            "RRB@",
            solve_part_1(
                "RR:A,B,C
A:D,E
B:F,@
C:G,H
D:@
E:@
F:@
G:@
H:@"
            )
        );
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(
            "",
            solve_part_2(
                "2 3 4 5
6 7 8 9"
            )
        );
    }

    #[test]
    fn test_solve_part_3() {
        assert_eq!(
            "",
            solve_part_3(
                "2 3 4 5
6 7 8 9"
            )
        );
    }
}
