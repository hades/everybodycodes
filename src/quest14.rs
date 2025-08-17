use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

use petgraph::{algo::floyd_warshall, graph::UnGraph};

pub fn solve_part_1(input: &str) -> String {
    let instructions: Vec<(char, i32)> = input
        .split(",")
        .map(|cmd| (cmd.chars().next().unwrap(), cmd[1..].parse().unwrap()))
        .collect();
    let mut height = 0;
    let mut z = 0;
    for (instruction, param) in instructions {
        match instruction {
            'U' => {
                z += param;
                height = max(height, z);
            }
            'D' => {
                z -= param;
            }
            _ => {}
        }
    }
    height.to_string()
}

fn make_tree(
    input: &str,
) -> (
    HashSet<(isize, isize, isize)>,
    HashSet<(isize, isize, isize)>,
) {
    let mut segments = HashSet::<(isize, isize, isize)>::new();
    let mut leaves = HashSet::new();
    for line in input.lines() {
        let instructions: Vec<(char, i32)> = line
            .split(",")
            .map(|cmd| (cmd.chars().next().unwrap(), cmd[1..].parse().unwrap()))
            .collect();
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;
        for (instruction, param) in instructions {
            let dir = match instruction {
                'U' => (0, 0, 1),
                'D' => (0, 0, -1),
                'R' => (1, 0, 0),
                'L' => (-1, 0, 0),
                'F' => (0, 1, 0),
                'B' => (0, -1, 0),
                _ => unreachable!(),
            };
            for _ in 0..param {
                x += dir.0;
                y += dir.1;
                z += dir.2;
                segments.insert((x, y, z));
            }
        }
        leaves.insert((x, y, z));
    }
    (segments, leaves)
}

pub fn solve_part_2(input: &str) -> String {
    make_tree(input).0.len().to_string()
}

pub fn solve_part_3(input: &str) -> String {
    let (tree, leaves) = make_tree(input);
    let mut segment_to_node_id = HashMap::<_, _>::new();
    let mut graph = UnGraph::new_undirected();
    for segment in tree.iter() {
        segment_to_node_id.insert(segment, graph.add_node(()));
    }
    for (x, y, z) in tree.iter() {
        let from_node = segment_to_node_id[&(*x, *y, *z)];
        for (dx, dy, dz) in [
            (0, 0, 1),
            (0, 0, -1),
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
        ] {
            if let Some(to_node) = segment_to_node_id.get(&(x + dx, y + dy, z + dz)) {
                graph.add_edge(from_node, *to_node, ());
            }
        }
    }
    let all_paths_shortest_paths = floyd_warshall(&graph, |_| 1).unwrap();
    segment_to_node_id
        .iter()
        .filter(|((x, y, _), _)| *x == 0 && *y == 0)
        .map(|((_, _, _), trunk_node_id)| {
            leaves
                .iter()
                .map(|leaf| segment_to_node_id[leaf])
                .map(|leaf_node_id| all_paths_shortest_paths[&(*trunk_node_id, leaf_node_id)])
                .sum::<isize>()
        })
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_part_1() {
        assert_eq!("7", solve_part_1("U5,R3,D2,L5,U4,R5,D2"));
    }

    #[test]
    fn test_part_2() {
        assert_eq!("24", solve_part_2("U5,R3,D2,L5,U4,R5,D2"));
        assert_eq!("14", solve_part_2("U6,L1,D2,R3,U2,L1"));
        assert_eq!(
            "32",
            solve_part_2(
                "U5,R3,D2,L5,U4,R5,D2
U6,L1,D2,R3,U2,L1"
            )
        );
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            "5",
            solve_part_3(
                "U5,R3,D2,L5,U4,R5,D2
U6,L1,D2,R3,U2,L1"
            )
        );
        assert_eq!(
            "46",
            solve_part_3(
                "U20,L1,B1,L2,B1,R2,L1,F1,U1
U10,F1,B1,R1,L1,B1,L1,F1,R2,U1
U30,L2,F1,R1,B1,R1,F2,U1,F1
U25,R1,L2,B1,U1,R2,F1,L2
U16,L1,B1,L1,B3,L1,B1,F1"
            )
        );
    }
}
