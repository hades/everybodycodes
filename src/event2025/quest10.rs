use std::collections::{BTreeSet, HashMap, HashSet};

use itertools::Itertools;

pub fn solve_part_1(input: &str) -> String {
    let board: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut front: HashSet<_> = (0usize..board.len())
        .cartesian_product(0usize..board[0].len())
        .filter(|&(i, j)| board[i][j] == 'D')
        .collect();
    let mut visited = HashSet::new();
    let knight_moves: [(isize, isize); 8] = [
        (2, 1),
        (2, -1),
        (-2, -1),
        (-2, 1),
        (1, 2),
        (1, -2),
        (-1, -2),
        (-1, 2),
    ];
    for _ in 0..=4 {
        let mut next_front = HashSet::new();
        for (i, j) in front.drain() {
            for (di, dj) in knight_moves {
                let (ni, nj) = (i.wrapping_add_signed(di), j.wrapping_add_signed(dj));
                if ni >= board.len() || nj >= board[0].len() {
                    continue;
                }
                if visited.contains(&(ni, nj)) {
                    continue;
                }
                next_front.insert((ni, nj));
            }
            visited.insert((i, j));
        }
        front = next_front;
    }
    visited
        .drain()
        .filter(|&(i, j)| board[i][j] == 'S')
        .count()
        .to_string()
}

fn solve_part_2_with_turns(input: &str, turns: usize) -> String {
    let board: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut front: HashSet<_> = (0usize..board.len())
        .cartesian_product(0usize..board[0].len())
        .filter(|&(i, j)| board[i][j] == 'D')
        .collect();
    let knight_moves: [(isize, isize); 8] = [
        (2, 1),
        (2, -1),
        (-2, -1),
        (-2, 1),
        (1, 2),
        (1, -2),
        (-1, -2),
        (-1, 2),
    ];
    let mut eaten_sheep = HashSet::new();
    for turn in 0..=turns {
        let mut next_front = HashSet::new();
        for (i, j) in front.drain() {
            for (di, dj) in knight_moves {
                let (ni, nj) = (i.wrapping_add_signed(di), j.wrapping_add_signed(dj));
                if ni >= board.len() || nj >= board[0].len() {
                    continue;
                }
                next_front.insert((ni, nj));
            }
            if board[i][j] != '#' {
                if let Some(sheep_i) = (i + 1).checked_sub(turn)
                    && board[sheep_i][j] == 'S'
                {
                    eaten_sheep.insert((sheep_i, j));
                }
                if let Some(sheep_i) = i.checked_sub(turn)
                    && turn != 0
                    && board[sheep_i][j] == 'S'
                {
                    eaten_sheep.insert((sheep_i, j));
                }
            }
        }
        front = next_front;
    }
    eaten_sheep.len().to_string()
}

pub fn solve_part_2(input: &str) -> String {
    solve_part_2_with_turns(input, 20)
}
type VeryComplexType = HashMap<(usize, usize, usize, Vec<(usize, usize)>), usize>;
fn count_winning_sequences(
    turn: usize,
    dragon: (usize, usize),
    hiding_places: &HashSet<(usize, usize)>,
    sheep: BTreeSet<(usize, usize)>,
    height: usize,
    width: usize,
    cache: &mut VeryComplexType,
) -> usize {
    if sheep.is_empty() {
        return 1;
    }
    let cache_key = (
        turn % 2,
        dragon.0,
        dragon.1,
        sheep.iter().cloned().collect(),
    );
    if let Some(result) = cache.get(&cache_key) {
        return *result;
    }
    if turn % 2 == 1 {
        let knight_moves: [(isize, isize); 8] = [
            (2, 1),
            (2, -1),
            (-2, -1),
            (-2, 1),
            (1, 2),
            (1, -2),
            (-1, -2),
            (-1, 2),
        ];
        let (i, j) = dragon;
        let mut total = 0;
        for (di, dj) in knight_moves {
            let (ni, nj) = (i.wrapping_add_signed(di), j.wrapping_add_signed(dj));
            if ni >= height || nj >= width {
                continue;
            }
            if !hiding_places.contains(&(ni, nj)) && sheep.contains(&(ni, nj)) {
                let mut new_sheep = sheep.clone();
                new_sheep.remove(&(ni, nj));
                total += count_winning_sequences(
                    turn + 1,
                    (ni, nj),
                    hiding_places,
                    new_sheep,
                    height,
                    width,
                    cache,
                );
            } else {
                total += count_winning_sequences(
                    turn + 1,
                    (ni, nj),
                    hiding_places,
                    sheep.clone(),
                    height,
                    width,
                    cache,
                );
            }
        }
        cache.insert(cache_key, total);
        total
    } else {
        let mut sheep_moves_available = false;
        let mut total = 0;
        for &(i, j) in sheep.iter() {
            if dragon == (i + 1, j) && !hiding_places.contains(&(i + 1, j)) {
                continue;
            }
            sheep_moves_available = true;
            if i == (height - 1) {
                continue;
            }
            let mut new_sheep = sheep.clone();
            new_sheep.remove(&(i, j));
            new_sheep.insert((i + 1, j));
            total += count_winning_sequences(
                turn + 1,
                dragon,
                hiding_places,
                new_sheep,
                height,
                width,
                cache,
            );
        }
        if !sheep_moves_available {
            return count_winning_sequences(
                turn + 1,
                dragon,
                hiding_places,
                sheep,
                height,
                width,
                cache,
            );
        }
        cache.insert(cache_key, total);
        total
    }
}

pub fn solve_part_3(input: &str) -> String {
    let board: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let dragon = (0usize..board.len())
        .cartesian_product(0usize..board[0].len())
        .filter(|&(i, j)| board[i][j] == 'D')
        .exactly_one()
        .unwrap();
    let sheep = (0usize..board.len())
        .cartesian_product(0usize..board[0].len())
        .filter(|&(i, j)| board[i][j] == 'S')
        .collect::<BTreeSet<_>>();
    let hiding_places = (0usize..board.len())
        .cartesian_product(0usize..board[0].len())
        .filter(|&(i, j)| board[i][j] == '#')
        .collect::<HashSet<_>>();
    let mut cache = HashMap::new();
    count_winning_sequences(
        0,
        dragon,
        &hiding_places,
        sheep,
        board.len(),
        board[0].len(),
        &mut cache,
    )
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_log::test;

    #[test]
    fn test_solve_part_3() {
        assert_eq!(
            "15",
            solve_part_3(
                "SSS
..#
#.#
#D."
            )
        );
        assert_eq!(
            "8",
            solve_part_3(
                "SSS
..#
..#
.##
.D#"
            )
        );
        assert_eq!(
            "44",
            solve_part_3(
                "..S..
.....
..#..
.....
..D.."
            )
        );
        assert_eq!(
            "4406",
            solve_part_3(
                ".SS.S
#...#
...#.
##..#
.####
##D.#"
            )
        );
        assert_eq!(
            "13033988838",
            solve_part_3(
                "SSS.S
.....
#.#.#
.#.#.
#.D.#"
            )
        );
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(
            "27",
            solve_part_2_with_turns(
                "...SSS##.....
.S#.##..S#SS.
..S.##.S#..S.
.#..#S##..SS.
..SSSS.#.S.#.
.##..SS.#S.#S
SS##.#D.S.#..
S.S..S..S###.
.##.S#.#....S
.SSS.#SS..##.
..#.##...S##.
.#...#.S#...S
SS...#.S.#S..",
                3
            )
        );
    }
}
