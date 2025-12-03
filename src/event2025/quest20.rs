use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

fn neighbours_of(i: usize, j: usize, side: usize) -> impl Iterator<Item = (usize, usize)> {
    [
        (i, j.wrapping_sub(1)),
        (i, j + 1),
        (
            if (i + j).is_multiple_of(2) {
                i.wrapping_sub(1)
            } else {
                i + 1
            },
            j,
        ),
    ]
    .into_iter()
    .filter(move |&(i, j)| i < side && j >= i && j < (2 * side - i - 1))
}

pub fn solve_part_1(input: &str) -> String {
    let data = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let side = data.len();
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    let mut pairs = 0;
    let mut visited = HashSet::new();
    while let Some((i, j)) = queue.pop_front() {
        if visited.contains(&(i, j)) {
            continue;
        }
        for (ni, nj) in neighbours_of(i, j, side) {
            if visited.contains(&(ni, nj)) {
                continue;
            }
            if data[i][j] == 'T' && data[ni][nj] == 'T' {
                pairs += 1;
            }
            queue.push_back((ni, nj));
        }
        visited.insert((i, j));
    }
    pairs.to_string()
}

pub fn solve_part_2(input: &str) -> String {
    let data = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let side = data.len();
    let mut front = (0..data.len())
        .cartesian_product(0..data[0].len())
        .filter(|&(i, j)| data[i][j] == 'S')
        .collect::<HashSet<_>>();
    let mut visited = HashSet::new();
    let mut steps = 0;
    while !front.is_empty() {
        let mut next_front = HashSet::new();
        for (i, j) in front.drain() {
            if data[i][j] == 'E' {
                return steps.to_string();
            }
            visited.insert((i, j));
            for (ni, nj) in neighbours_of(i, j, side) {
                if (data[ni][nj] == 'T' || data[ni][nj] == 'E') && !visited.contains(&(ni, nj)) {
                    next_front.insert((ni, nj));
                }
            }
        }
        steps += 1;
        front = next_front;
    }
    panic!("exit not found");
}

pub fn rotate(data: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut result = vec![vec!['.'; data[0].len()]; data.len()];
    let side = data.len();
    for i in 0..data.len() {
        for j in i..(data[0].len() - i) {
            if (i + j).is_multiple_of(2) {
                result[i][j] = data[side - (i + j) / 2 - 1][i * 2 + side - (i + j) / 2 - 1];
            } else {
                result[i][j] = data[side - (i + j).div_ceil(2) - 1][side - (j - i).div_ceil(2) + i];
            }
        }
    }
    result
}

pub fn solve_part_3(input: &str) -> String {
    let data = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let side = data.len();
    let data = [data.clone(), rotate(&data), rotate(&rotate(&data))];
    let mut front = (0..data[0].len())
        .cartesian_product(0..data[0][0].len())
        .filter(|&(i, j)| data[0][i][j] == 'S')
        .map(|(i, j)| (i, j, 0))
        .collect::<HashSet<_>>();
    let mut visited = HashSet::new();
    let mut steps = 0;
    while !front.is_empty() {
        let mut next_front = HashSet::new();
        for (i, j, rotation) in front.drain() {
            if data[rotation][i][j] == 'E' {
                return steps.to_string();
            }
            visited.insert((i, j, rotation));
            let next_rotation = (rotation + 1) % 3;
            for (ni, nj) in neighbours_of(i, j, side) {
                if (data[next_rotation][ni][nj] == 'T' || data[next_rotation][ni][nj] == 'E')
                    && !visited.contains(&(ni, nj, next_rotation))
                {
                    next_front.insert((ni, nj, next_rotation));
                }
            }
            if (data[next_rotation][i][j] == 'T' || data[next_rotation][i][j] == 'E')
                && !visited.contains(&(i, j, next_rotation))
            {
                next_front.insert((i, j, next_rotation));
            }
        }
        steps += 1;
        front = next_front;
    }
    panic!("exit not found");
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use test_log::test;

    #[test]
    fn test_neighbours_of() {
        assert_eq!(
            vec![(0, 1)],
            neighbours_of(0, 0, 6).sorted().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![(0, 0), (0, 2), (1, 1)],
            neighbours_of(0, 1, 6).sorted().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![(0, 1), (0, 3)],
            neighbours_of(0, 2, 6).sorted().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![(0, 2), (0, 4), (1, 3)],
            neighbours_of(0, 3, 6).sorted().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![(0, 8), (0, 10), (1, 9)],
            neighbours_of(0, 9, 6).sorted().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![(0, 9)],
            neighbours_of(0, 10, 6).sorted().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![(0, 1), (1, 2)],
            neighbours_of(1, 1, 6).sorted().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![(1, 1), (1, 3), (2, 2)],
            neighbours_of(1, 2, 6).sorted().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![(1, 7), (1, 9), (2, 8)],
            neighbours_of(1, 8, 6).sorted().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![(0, 9), (1, 8)],
            neighbours_of(1, 9, 6).sorted().collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_solve_part_1() {
        assert_eq!(
            "7",
            solve_part_1(
                "T#TTT###T##
.##TT#TT##.
..T###T#T..
...##TT#...
....T##....
.....#....."
            )
        );
        assert_eq!(
            "0",
            solve_part_1(
                "T#T#T#T#T#T
.T#T#T#T#T.
..T#T#T#T..
...T#T#T...
....T#T....
.....T....."
            )
        );
        assert_eq!(
            "0",
            solve_part_1(
                "T#T#T#T#T#T
.#T#T#T#T#.
..#T###T#..
...##T##...
....#T#....
.....#....."
            )
        );
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(
            "32",
            solve_part_2(
                "TTTTTTTTTTTTTTTTT
.TTTT#T#T#TTTTTT.
..TT#TTTETT#TTT..
...TT#T#TTT#TT...
....TTT#T#TTT....
.....TTTTTT#.....
......TT#TT......
.......#TT.......
........S........"
            )
        );
    }
    #[test]
    fn test_rotate() {
        let data = "abcde
.fgh.
..i.."
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let rotated = rotate(&data);
        assert_eq!("igfba", rotated[0].iter().join(""));
        assert_eq!(".hdc.", rotated[1].iter().join(""));
        assert_eq!("..e..", rotated[2].iter().join(""));
    }

    #[test]
    fn test_solve_part_3() {
        assert_eq!(
            "23",
            solve_part_3(
                "T####T#TTT##T##T#T#
.T#####TTTT##TTT##.
..TTTT#T###TTTT#T..
...T#TTT#ETTTT##...
....#TT##T#T##T....
.....#TT####T#.....
......T#TT#T#......
.......T#TTT.......
........TT#........
.........S........."
            )
        );
    }
}
