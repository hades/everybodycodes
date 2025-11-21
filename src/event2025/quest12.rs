use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn explode_from_barrel(
    map: &HashMap<(isize, isize), i8>,
    mut exploded: HashSet<(isize, isize)>,
    mut front: HashSet<(isize, isize)>,
) -> HashSet<(isize, isize)> {
    while !front.is_empty() {
        let mut next_front = HashSet::new();
        for (i, j) in front.drain() {
            exploded.insert((i, j));
            let my_size = map[&(i, j)];
            for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
                let (ni, nj) = (i + di, j + dj);
                if exploded.contains(&(ni, nj)) {
                    continue;
                }
                if let Some(neighour_size) = map.get(&(ni, nj))
                    && *neighour_size <= my_size
                {
                    next_front.insert((ni, nj));
                }
            }
        }
        front = next_front;
    }
    exploded
}

pub fn solve_part_1(input: &str) -> String {
    let map: HashMap<(isize, isize), i8> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, ch)| ((i as isize, j as isize), ch as i8 - '0' as i8))
        })
        .collect();
    let exploded = HashSet::<(isize, isize)>::new();
    let front: HashSet<(isize, isize)> = [(0, 0)].into_iter().collect();
    explode_from_barrel(&map, exploded, front).len().to_string()
}

pub fn solve_part_2(input: &str) -> String {
    let map: HashMap<(isize, isize), i8> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, ch)| ((i as isize, j as isize), ch as i8 - '0' as i8))
        })
        .collect();
    let exploded = HashSet::<(isize, isize)>::new();
    let max_i = map.keys().map(|(i, _)| *i).max().unwrap();
    let max_j = map.keys().map(|(_, j)| *j).max().unwrap();
    let front: HashSet<(isize, isize)> = [(0, 0), (max_i, max_j)].into_iter().collect();
    explode_from_barrel(&map, exploded, front).len().to_string()
}

pub fn solve_part_3(input: &str) -> String {
    let map: HashMap<(isize, isize), i8> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, ch)| ((i as isize, j as isize), ch as i8 - '0' as i8))
        })
        .collect();
    let max_i = map.keys().map(|(i, _)| *i).max().unwrap();
    let max_j = map.keys().map(|(_, j)| *j).max().unwrap();
    let best_barrel = (0..=max_i)
        .cartesian_product(0..=max_j)
        .map(|(i, j)| {
            ((i, j), {
                let exploded = HashSet::<(isize, isize)>::new();
                let front: HashSet<(isize, isize)> = [(i, j)].into_iter().collect();
                explode_from_barrel(&map, exploded, front)
            })
        })
        .max_by_key(|(_, exploded)| exploded.len())
        .unwrap();
    let second_best_barrel = (0..=max_i)
        .cartesian_product(0..=max_j)
        .filter(|&(i, j)| !best_barrel.1.contains(&(i, j)))
        .map(|(i, j)| {
            ((i, j), {
                let exploded = best_barrel.1.clone();
                let front: HashSet<(isize, isize)> = [(i, j)].into_iter().collect();
                explode_from_barrel(&map, exploded, front)
            })
        })
        .max_by_key(|(_, exploded)| exploded.len())
        .unwrap();
    let third_best_barrel = (0..=max_i)
        .cartesian_product(0..=max_j)
        .filter(|&(i, j)| !second_best_barrel.1.contains(&(i, j)))
        .map(|(i, j)| {
            ((i, j), {
                let exploded = second_best_barrel.1.clone();
                let front: HashSet<(isize, isize)> = [(i, j)].into_iter().collect();
                explode_from_barrel(&map, exploded, front)
            })
        })
        .max_by_key(|(_, exploded)| exploded.len())
        .unwrap();
    let exploded = HashSet::<(isize, isize)>::new();
    let front: HashSet<(isize, isize)> = [best_barrel.0, second_best_barrel.0, third_best_barrel.0]
        .into_iter()
        .collect();
    explode_from_barrel(&map, exploded, front).len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!(
            "16",
            solve_part_1(
                "989601
857782
746543
766789"
            )
        );
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(
            "58",
            solve_part_2(
                "9589233445
9679121695
8469121876
8352919876
7342914327
7234193437
6789193538
6781219648
5691219769
5443329859"
            )
        );
    }

    #[test]
    fn test_solve_part_3_smol() {
        assert_eq!(
            "14",
            solve_part_3(
                "5411
3362
5235
3112"
            )
        )
    }

    #[test]
    fn test_solve_part_3() {
        assert_eq!(
            "136",
            solve_part_3(
                "41951111131882511179
32112222211518122215
31223333322115122219
31234444432147511128
91223333322176121892
61112222211166431583
14661111166111111746
11111119142122222177
41222118881233333219
71222127839122222196
56111126279711111517"
            )
        );
    }
}
