use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn solve_part_1(input: &str) -> String {
    let mut barrels: HashSet<(isize, isize)> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, ch)| ch == '#')
                .map(move |(j, _)| (i as isize, j as isize))
        })
        .collect();
    let max_i = barrels.iter().map(|(i, _)| *i).max().unwrap();
    let max_j = barrels.iter().map(|(_, j)| *j).max().unwrap();
    let mut total_active = 0;
    for _ in 0..10 {
        let mut next_barrels = HashSet::new();
        for i in 0..=max_i {
            for j in 0..=max_j {
                let active_neighbours = [(-1, -1), (1, 1), (-1, 1), (1, -1)]
                    .iter()
                    .filter(|(di, dj)| barrels.contains(&(i + di, j + dj)))
                    .count();
                let will_be_active = if barrels.contains(&(i, j)) {
                    active_neighbours % 2 == 1
                } else {
                    active_neighbours % 2 == 0
                };
                if will_be_active {
                    next_barrels.insert((i, j));
                }
            }
        }
        barrels = next_barrels;
        total_active += barrels.len();
    }
    total_active.to_string()
}

pub fn solve_part_2(input: &str) -> String {
    let mut barrels: HashSet<(isize, isize)> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, ch)| ch == '#')
                .map(move |(j, _)| (i as isize, j as isize))
        })
        .collect();
    let max_i = barrels.iter().map(|(i, _)| *i).max().unwrap();
    let max_j = barrels.iter().map(|(_, j)| *j).max().unwrap();
    let mut total_active = 0;
    for _ in 0..2025 {
        let mut next_barrels = HashSet::new();
        for i in 0..=max_i {
            for j in 0..=max_j {
                let active_neighbours = [(-1, -1), (1, 1), (-1, 1), (1, -1)]
                    .iter()
                    .filter(|(di, dj)| barrels.contains(&(i + di, j + dj)))
                    .count();
                let will_be_active = if barrels.contains(&(i, j)) {
                    active_neighbours % 2 == 1
                } else {
                    active_neighbours % 2 == 0
                };
                if will_be_active {
                    next_barrels.insert((i, j));
                }
            }
        }
        barrels = next_barrels;
        total_active += barrels.len();
    }
    total_active.to_string()
}

pub fn solve_part_3(input: &str) -> String {
    let pattern: HashSet<(isize, isize)> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, ch)| ch == '#')
                .map(move |(j, _)| (i as isize, j as isize))
        })
        .collect();
    let pattern_max_i = pattern.iter().map(|(i, _)| *i).max().unwrap();
    let pattern_max_j = pattern.iter().map(|(_, j)| *j).max().unwrap();
    assert_eq!(7, pattern_max_i);
    assert_eq!(7, pattern_max_j);
    let mut barrels: HashSet<(isize, isize)> = HashSet::new();
    let max_i = 33;
    let max_j = 33;
    let mut state_map = HashMap::<Vec<(isize, isize)>, usize>::new();
    state_map.insert(vec![], 0);
    let mut current_round = 0;
    let rounds_to_simulate = 1000000000;
    let mut active_tiles_after_rounds = vec![0];
    while current_round < rounds_to_simulate {
        let mut next_barrels = HashSet::new();
        for i in 0..=max_i {
            for j in 0..=max_j {
                let active_neighbours = [(-1, -1), (1, 1), (-1, 1), (1, -1)]
                    .iter()
                    .filter(|(di, dj)| barrels.contains(&(i + di, j + dj)))
                    .count();
                let will_be_active = if barrels.contains(&(i, j)) {
                    active_neighbours % 2 == 1
                } else {
                    active_neighbours % 2 == 0
                };
                if will_be_active {
                    next_barrels.insert((i, j));
                }
            }
        }
        barrels = next_barrels;
        current_round += 1;
        let state_key: Vec<_> = barrels.iter().copied().collect();
        if let Some(&seen_before_after_round) = state_map.get(&state_key) {
            let loop_length = current_round - seen_before_after_round;
            let loops_remaining = (rounds_to_simulate - current_round) / loop_length;
            let iterations_remaining =
                rounds_to_simulate - current_round - (loops_remaining * loop_length);
            return (active_tiles_after_rounds[0..current_round]
                .iter()
                .sum::<usize>()
                + active_tiles_after_rounds[seen_before_after_round..current_round]
                    .iter()
                    .sum::<usize>()
                    * loops_remaining
                + active_tiles_after_rounds
                    [seen_before_after_round..(seen_before_after_round + iterations_remaining)]
                    .iter()
                    .sum::<usize>())
            .to_string();
        }
        state_map.insert(state_key, current_round);
        let does_pattern_match = (13..=20)
            .cartesian_product(13..=20)
            .all(|(i, j)| barrels.contains(&(i, j)) == pattern.contains(&(i - 13, j - 13)));
        active_tiles_after_rounds.push(if does_pattern_match { barrels.len() } else { 0 });
    }
    active_tiles_after_rounds.iter().sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!(
            "200",
            solve_part_1(
                ".#.##.
##..#.
..##.#
.#.##.
.###..
###.##"
            )
        );
    }
}
