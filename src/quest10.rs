use std::ops::{Index, IndexMut};

use array2d::Array2D;
use itertools::Itertools;
use log::debug;

pub fn solve_part_1(input: &str) -> String {
    let mut grid = Array2D::filled_with('.', 8, 8);
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            grid[(i, j)] = ch;
        }
    }
    for i in 2..6 {
        for j in 2..6 {
            grid[(i, j)] = [0, 1, 6, 7]
                .iter()
                .flat_map(|check_column| {
                    let char_in_column = grid[(i, *check_column)];
                    [0, 1, 6, 7]
                        .iter()
                        .map(|check_row| grid[(*check_row, j)])
                        .filter(move |ch| *ch == char_in_column)
                })
                .exactly_one()
                .unwrap();
        }
    }
    String::from_iter((2..6).cartesian_product(2..6).map(|(j, i)| grid[(j, i)]))
}

struct ArrayView<'a, T> {
    array: &'a mut Array2D<T>,
    start_i: usize,
    start_j: usize,
    len_i: usize,
    len_j: usize,
}

impl<'a, T> Index<(usize, usize)> for ArrayView<'a, T> {
    type Output = T;

    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        if row >= self.len_i || column >= self.len_j {
            panic!(
                "ArrayView[] with ({}, {}) exceeds len ({}, {})",
                row, column, self.len_i, self.len_j
            );
        }
        self.array
            .get(row + self.start_i, column + self.start_j)
            .expect("index out of bounds")
    }
}

impl<'a, T> IndexMut<(usize, usize)> for ArrayView<'a, T> {
    fn index_mut(&mut self, (row, column): (usize, usize)) -> &mut T {
        if row >= self.len_i || column >= self.len_j {
            panic!(
                "ArrayView[] with ({}, {}) exceeds len ({}, {})",
                row, column, self.len_i, self.len_j
            );
        }
        self.array
            .get_mut(row + self.start_i, column + self.start_j)
            .expect("index out of bounds")
    }
}

fn find_runic_word(grid: &mut ArrayView<char>) -> String {
    for i in 2..6 {
        for j in 2..6 {
            grid[(i, j)] = [0, 1, 6, 7]
                .iter()
                .flat_map(|check_column| {
                    let char_in_column = grid[(i, *check_column)];
                    [0, 1, 6, 7]
                        .iter()
                        .map(|check_row| grid[(*check_row, j)])
                        .filter(move |ch| *ch == char_in_column)
                })
                .exactly_one()
                .unwrap();
        }
    }
    String::from_iter((2..6).cartesian_product(2..6).map(|(j, i)| grid[(j, i)]))
}

pub fn solve_part_2(input: &str) -> String {
    let map_width = input.lines().next().unwrap().len();
    let map_height = input.lines().count();
    let mut grid = Array2D::filled_with('.', map_height, map_width);
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            grid[(i, j)] = ch;
        }
    }
    let mut start_i = 0;
    let mut start_j = 0;
    let mut total_power = 0;
    while start_i < map_height {
        while start_j < map_width {
            let mut view = ArrayView {
                array: &mut grid,
                start_i,
                start_j,
                len_i: 8,
                len_j: 8,
            };
            let word = find_runic_word(&mut view);
            let power: usize = word
                .chars()
                .enumerate()
                .map(|(i, ch)| (i + 1) * (ch as usize - 'A' as usize + 1))
                .sum();
            debug!("{start_i}, {start_j}, {word}, {power}");
            total_power += power;
            start_j += 9;
        }
        start_i += 9;
        start_j = 0;
    }
    total_power.to_string()
}

fn solve_runic_section(grid: &mut ArrayView<char>) -> Option<String> {
    let mut pending_coordinates: Vec<(usize, usize)> = vec![];
    for i in 2..6 {
        for j in 2..6 {
            let row_letters: Vec<char> = [0, 1, 6, 7]
                .iter()
                .map(|border_j| grid[(i, *border_j)])
                .collect();
            let col_letters: Vec<char> = [0, 1, 6, 7]
                .iter()
                .map(|border_i| grid[(*border_i, j)])
                .collect();
            let mut solution = ' ';
            for r in &row_letters {
                for c in &col_letters {
                    if c == r {
                        solution = *c;
                    }
                }
            }
            if solution == ' ' {
                if row_letters.iter().find(|ch| **ch == '?').is_none()
                    && col_letters.iter().find(|ch| **ch == '?').is_none()
                {
                    return None;
                } else {
                    pending_coordinates.push((i, j));
                }
            } else {
                grid[(i, j)] = solution;
            }
        }
    }
    for (i, j) in pending_coordinates {
        let row_letters: Vec<(usize, char)> = [0, 1, 6, 7]
            .iter()
            .map(|border_j| (*border_j, grid[(i, *border_j)]))
            .collect();
        let col_letters: Vec<(usize, char)> = [0, 1, 6, 7]
            .iter()
            .map(|border_i| (*border_i, grid[(*border_i, j)]))
            .collect();
        if let Some(question_idx) = row_letters.iter().find(|ch| ch.1 == '?') {
            // Question mark is in the row
            //  **   B**
            //  **   K**
            //  ?GLWG.WL
            //       B
            //       K
            //       M
            //  **   M**
            //  **   V**

            // Fill the question mark with a character from the column that hasn't been used.
            let solution = col_letters
                .iter()
                .map(|p| p.1)
                .filter(|ch| {
                    (2..6)
                        .find(|i| {
                            debug!("{} {} {} {}", *i, j, *ch, grid[(*i, j)]);
                            grid[(*i, j)] == *ch
                        })
                        .is_none()
                })
                .exactly_one()
                .unwrap();
            grid[(i, j)] = solution;
            grid[(i, question_idx.0)] = solution;
            continue;
        }
        if let Some(question_idx) = col_letters.iter().find(|ch| ch.1 == '?') {
            let solution = row_letters
                .iter()
                .map(|p| p.1)
                .filter(|ch| (2..6).find(|j| grid[(i, *j)] == *ch).is_none())
                .exactly_one()
                .unwrap();
            grid[(i, j)] = solution;
            grid[(question_idx.0, j)] = solution;
            continue;
        }
    }
    Some(String::from_iter(
        (2..6).cartesian_product(2..6).map(|(j, i)| grid[(j, i)]),
    ))
}

pub fn solve_part_3(input: &str) -> String {
    let map_width = input.lines().next().unwrap().len();
    let map_height = input.lines().count();
    let mut grid = Array2D::filled_with('.', map_height, map_width);
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            grid[(i, j)] = ch;
        }
    }
    let mut start_i = 0;
    let mut start_j = 0;
    let mut total_power = 0;
    while (start_i + 7) < map_height {
        while (start_j + 7) < map_width {
            let mut view = ArrayView {
                array: &mut grid,
                start_i,
                start_j,
                len_i: 8,
                len_j: 8,
            };
            if let Some(word) = solve_runic_section(&mut view) {
                let power: usize = word
                    .chars()
                    .enumerate()
                    .map(|(i, ch)| (i + 1) * (ch as usize - 'A' as usize + 1))
                    .sum();
                debug!("{start_i}, {start_j}, {word}, {power}");
                total_power += power;
            }
            start_j += 6;
        }
        start_i += 6;
        start_j = 0;
    }
    total_power.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!(
            "PTBVRCZHFLJWGMNS",
            solve_part_1(
                "**PCBS**
**RLNW**
BV....PT
CR....HZ
FL....JW
SG....MN
**FTZV**
**GMJH**"
            )
        );
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(
            "1851",
            solve_part_2(
                "**PCBS**
**RLNW**
BV....PT
CR....HZ
FL....JW
SG....MN
**FTZV**
**GMJH**"
            )
        );
    }

    #[test]
    fn test_solve_part_3() {
        assert_eq!(
            "3889",
            solve_part_3(
                "**XFZB**DCST**
**LWQK**GQJH**
?G....WL....DQ
BS....H?....CN
P?....KJ....TV
NM....Z?....SG
**NSHM**VKWZ**
**PJGV**XFNL**
WQ....?L....YS
FX....DJ....HV
?Y....WM....?J
TJ....YK....LP
**XRTK**BMSP**
**DWZN**GCJV**"
            )
        );
    }
}
