use std::{
    collections::{HashMap, HashSet},
    time::SystemTime,
};

use itertools::Itertools;
use log::debug;
use petgraph::{algo::dijkstra, prelude::UnGraphMap};
use sorted_vec::SortedVec;

use crate::debug_with_rate;

struct Map {
    start_point: (isize, isize),
    herbs: Vec<(char, isize, isize)>,
    pathways: HashSet<(isize, isize)>,
}

fn parse_map(input: &str) -> Map {
    let mut start_point = None;
    let mut herbs = Vec::new();
    let mut pathways = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            match ch {
                '.' => {
                    pathways.insert((i as isize, j as isize));
                    if i == 0 {
                        start_point = Some((i as isize, j as isize));
                    }
                }
                '#' => {}
                '~' => {}
                herb => {
                    pathways.insert((i as isize, j as isize));
                    herbs.push((herb, i as isize, j as isize));
                }
            }
        }
    }
    Map {
        herbs,
        pathways,
        start_point: start_point.unwrap(),
    }
}

pub fn solve_part_1(input: &str) -> String {
    let map = parse_map(input);
    let mut graph = UnGraphMap::<_, _>::new();
    for pathway in map.pathways.iter() {
        graph.add_node(*pathway);
    }
    for (x, y) in map.pathways.iter() {
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter() {
            let nx = *x + *dx;
            let ny = *y + *dy;
            if map.pathways.contains(&(nx, ny)) {
                graph.add_edge((*x, *y), (nx, ny), ());
            }
        }
    }
    let shortest_paths = dijkstra(&graph, map.start_point, None, |_| 1);
    map.herbs
        .iter()
        .map(|(_, herb_x, herb_y)| shortest_paths[&(*herb_x, *herb_y)] * 2)
        .min()
        .unwrap()
        .to_string()
}

pub fn solve_part_2(input: &str) -> String {
    let map = parse_map(input);
    let mut graph = UnGraphMap::<_, _>::new();
    for pathway in map.pathways.iter() {
        graph.add_node(*pathway);
    }
    for (x, y) in map.pathways.iter() {
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter() {
            let nx = *x + *dx;
            let ny = *y + *dy;
            if map.pathways.contains(&(nx, ny)) {
                graph.add_edge((*x, *y), (nx, ny), ());
            }
        }
    }
    let mut dijkstra_cached_for = HashSet::new();
    let mut shortest_paths_cache = HashMap::new();
    let herb_types: Vec<_> = map
        .herbs
        .iter()
        .map(|(herb, _, _)| *herb)
        .unique()
        .collect();
    debug!("{} herb types (oof)", herb_types.len());
    let mut best_distance = std::i32::MAX;
    for strategy in herb_types.iter().permutations(herb_types.len()) {
        debug_with_rate!("{strategy:?} {best_distance}");
        let (mut x, mut y) = map.start_point;
        let mut distance = 0;
        for herb_to_find in strategy {
            if !dijkstra_cached_for.contains(&(x, y)) {
                let mut shortest_paths_from_here = dijkstra(&graph, (x, y), None, |_| 1);
                shortest_paths_from_here
                    .drain()
                    .for_each(|((to_x, to_y), dist)| {
                        shortest_paths_cache.insert(((x, y), (to_x, to_y)), dist);
                    });
                dijkstra_cached_for.insert((x, y));
            }
            let (_, nx, ny) = map
                .herbs
                .iter()
                .filter(|(herb, _, _)| *herb == *herb_to_find)
                .min_by_key(|(_, herb_x, herb_y)| {
                    shortest_paths_cache[&((x, y), (*herb_x, *herb_y))]
                })
                .unwrap();
            distance += shortest_paths_cache[&((x, y), (*nx, *ny))];
            (x, y) = (*nx, *ny);
            if distance > best_distance {
                break;
            }
        }
        if !dijkstra_cached_for.contains(&(x, y)) {
            let mut shortest_paths_from_here = dijkstra(&graph, (x, y), None, |_| 1);
            shortest_paths_from_here
                .drain()
                .for_each(|((to_x, to_y), dist)| {
                    shortest_paths_cache.insert(((x, y), (to_x, to_y)), dist);
                });
            dijkstra_cached_for.insert((x, y));
        }
        distance += shortest_paths_cache[&((x, y), map.start_point)];
        if distance < best_distance {
            best_distance = distance;
        }
    }
    best_distance.to_string()
}

pub fn solve_part_3(input: &str) -> String {
    let map = parse_map(input);
    let mut state_front = HashSet::new();
    let mut visited = HashSet::new();
    let herb_types: Vec<_> = map
        .herbs
        .iter()
        .map(|(herb, _, _)| *herb)
        .unique()
        .collect();
    let herb_map: HashMap<_, _> = map
        .herbs
        .iter()
        .map(|(herb, hx, hy)| ((*hx, *hy), *herb))
        .collect();
    state_front.insert((map.start_point.0, map.start_point.1, SortedVec::new()));
    let mut t = 0;
    loop {
        let mut new_front = HashSet::new();
        for (x, y, herbs_collected) in state_front.drain() {
            debug_with_rate!("{t}");
            visited.insert((x, y, herbs_collected.clone()));
            if x == map.start_point.0
                && y == map.start_point.1
                && herbs_collected.len() == herb_types.len()
            {
                return t.to_string();
            }
            for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)].into_iter() {
                let nx = x + dx;
                let ny = y + dy;
                if map.pathways.contains(&(nx, ny)) {
                    let mut new_state = (nx, ny, herbs_collected.clone());
                    if let Some(new_herb) = herb_map.get(&(nx, ny)) {
                        if herbs_collected.iter().find(|h| **h == *new_herb).is_none() {
                            new_state.2.insert(*new_herb);
                        }
                    }
                    if !visited.contains(&new_state) {
                        new_front.insert(new_state);
                    }
                }
            }
        }
        t += 1;
        state_front = new_front;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_part_1() {
        assert_eq!(
            "26",
            solve_part_1(
                "#####.#####
#.........#
#.######.##
#.........#
###.#.#####
#H.......H#
###########"
            )
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            "38",
            solve_part_2(
                "##########.##########
#...................#
#.###.##.###.##.#.#.#
#..A#.#..~~~....#A#.#
#.#...#.~~~~~...#.#.#
#.#.#.#.~~~~~.#.#.#.#
#...#.#.B~~~B.#.#...#
#...#....BBB..#....##
#C............#....C#
#####################"
            )
        );
    }
}
