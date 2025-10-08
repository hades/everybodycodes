use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

use itertools::Itertools;
use log::debug;
use petgraph::{algo::johnson, prelude::UnGraphMap};

pub fn solve_part_1(input: &str) -> String {
    let mut channels = Vec::new();
    let mut palm_trees = Vec::new();
    let mut width = 0;
    let mut height = 0;
    for (y, line) in input.lines().enumerate() {
        height = max(height, y + 1);
        for (x, ch) in line.chars().enumerate() {
            width = max(width, x + 1);
            match ch {
                'P' => {
                    channels.push((x, y));
                    palm_trees.push((x, y));
                }
                '.' => {
                    channels.push((x, y));
                }
                _ => {}
            }
        }
    }
    debug!("{channels:?}");
    let start_position = *channels
        .iter()
        .filter(|p| (p.0 == 0 || p.0 == width - 1) || (p.1 == 0 || p.1 == height - 1))
        .exactly_one()
        .unwrap();
    let channels: HashSet<_> = HashSet::from_iter(channels);
    let palm_trees: HashSet<_> = HashSet::from_iter(palm_trees);
    let mut t = 0;
    let mut fill_time: HashMap<(usize, usize), isize> = HashMap::new();
    let mut front = vec![start_position];
    while !front.is_empty() {
        let mut new_front = Vec::new();
        for (x, y) in front {
            fill_time.insert((x, y), t);
            for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx < 0 || ny < 0 || !channels.contains(&(nx as usize, ny as usize)) {
                    continue;
                }
                if fill_time.contains_key(&(nx as usize, ny as usize)) {
                    continue;
                }
                new_front.push((nx as usize, ny as usize));
            }
        }
        front = new_front;
        t += 1;
    }
    palm_trees
        .iter()
        .map(|coords| fill_time.get(coords).unwrap())
        .max()
        .unwrap()
        .to_string()
}

pub fn solve_part_2(input: &str) -> String {
    let mut channels = Vec::new();
    let mut palm_trees = Vec::new();
    let mut width = 0;
    let mut height = 0;
    for (y, line) in input.lines().enumerate() {
        height = max(height, y + 1);
        for (x, ch) in line.chars().enumerate() {
            width = max(width, x + 1);
            match ch {
                'P' => {
                    channels.push((x, y));
                    palm_trees.push((x, y));
                }
                '.' => {
                    channels.push((x, y));
                }
                _ => {}
            }
        }
    }
    debug!("{channels:?}");
    let start_positions: Vec<(usize, usize)> = channels
        .clone()
        .into_iter()
        .filter(|p| (p.0 == 0 || p.0 == width - 1) || (p.1 == 0 || p.1 == height - 1))
        .collect();
    let channels: HashSet<_> = HashSet::from_iter(channels);
    let palm_trees: HashSet<_> = HashSet::from_iter(palm_trees);
    let mut t = 0;
    let mut fill_time: HashMap<(usize, usize), isize> = HashMap::new();
    let mut front = start_positions.clone();
    while !front.is_empty() {
        let mut new_front = Vec::new();
        for (x, y) in front {
            fill_time.insert((x, y), t);
            for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx < 0 || ny < 0 || !channels.contains(&(nx as usize, ny as usize)) {
                    continue;
                }
                if fill_time.contains_key(&(nx as usize, ny as usize)) {
                    continue;
                }
                new_front.push((nx as usize, ny as usize));
            }
        }
        front = new_front;
        t += 1;
    }
    palm_trees
        .iter()
        .map(|coords| fill_time.get(coords).unwrap())
        .max()
        .unwrap()
        .to_string()
}

pub fn solve_part_3(input: &str) -> String {
    let mut channels = Vec::new();
    let mut palm_trees = Vec::new();
    let mut graph: UnGraphMap<_, _> = UnGraphMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                'P' => {
                    channels.push((x, y));
                    palm_trees.push((x, y));
                    graph.add_node((x, y));
                }
                '.' => {
                    channels.push((x, y));
                    graph.add_node((x, y));
                }
                _ => {}
            }
        }
    }
    for (x, y) in channels.iter() {
        for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let nx = *x as isize + dx;
            let ny = *y as isize + dy;
            if nx < 0 || ny < 0 || !channels.contains(&(nx as usize, ny as usize)) {
                continue;
            }
            graph.add_edge((*x, *y), (nx as usize, ny as usize), ());
        }
    }
    let shortest_paths = johnson(&graph, |_| 1).unwrap();
    channels
        .into_iter()
        .map(|(start_x, start_y)| {
            if palm_trees.contains(&(start_x, start_y)) {
                usize::MAX
            } else {
                palm_trees
                    .iter()
                    .map(|(palm_x, palm_y)| {
                        shortest_paths[&((start_x, start_y), (*palm_x, *palm_y))]
                    })
                    .sum::<usize>()
            }
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
        assert_eq!(
            "11",
            solve_part_1(
                "##########
..#......#
#.P.####P#
#.#...P#.#
##########"
            )
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            "21",
            solve_part_2(
                "#######################
...P..P...#P....#.....#
#.#######.#.#.#.#####.#
#.....#...#P#.#..P....#
#.#####.#####.#########
#...P....P.P.P.....P#.#
#.#######.#####.#.#.#.#
#...#.....#P...P#.#....
#######################"
            )
        );
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            "12",
            solve_part_3(
                "##########
#.#......#
#.P.####P#
#.#...P#.#
##########"
            )
        );
    }
}
