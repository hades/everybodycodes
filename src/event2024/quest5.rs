use std::collections::HashMap;
use std::collections::HashSet;

use crate::util::concatenate_numbers;

fn simulate_one_round(columns: &mut [Vec<usize>], round: usize) {
    let columns_count = columns.len();
    let clapper = columns[round % columns_count].remove(0);
    let target_column = &mut columns[(round + 1) % columns_count];
    let mut effective_clapper = clapper % (2 * target_column.len());
    if effective_clapper == 0 {
        effective_clapper = 2 * target_column.len();
    }
    if effective_clapper <= target_column.len() {
        target_column.insert(effective_clapper - 1, clapper);
    } else {
        target_column.insert(2 * target_column.len() - effective_clapper + 1, clapper);
    }
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    let mut columns: Vec<Vec<usize>> = Vec::new();
    for line in input.lines() {
        for (i, number) in line.split(" ").enumerate() {
            if columns.len() <= i {
                columns.resize_with(i + 1, Vec::new);
            }
            columns[i].push(number.parse().unwrap());
        }
    }
    columns
}

pub fn solve_part_1(input: &str) -> String {
    let mut columns = parse_input(input);
    for round in 0..10 {
        simulate_one_round(&mut columns, round);
    }
    columns
        .iter()
        .map(|c| c[0].to_string())
        .collect::<Vec<String>>()
        .join("")
        .to_string()
}

pub fn solve_part_2(input: &str) -> String {
    let mut columns = parse_input(input);
    let mut number_counts: HashMap<usize, usize> = HashMap::new();
    let mut round = 0;
    loop {
        simulate_one_round(&mut columns, round);
        let number = columns
            .iter()
            .fold(0, |res, column| concatenate_numbers(res, column[0]));
        let mut count = *number_counts.get(&number).unwrap_or(&0);
        count += 1;
        if count == 2024 {
            return (number * (round + 1)).to_string();
        }
        number_counts.insert(number, count);
        round += 1;
    }
}

pub fn solve_part_3(input: &str) -> String {
    let mut columns = parse_input(input);
    let mut states: HashSet<Vec<Vec<usize>>> = HashSet::new();
    let mut round = 0;
    let mut max: usize = 0;
    loop {
        simulate_one_round(&mut columns, round);
        let number = columns
            .iter()
            .fold(0, |res, column| concatenate_numbers(res, column[0]));
        if max < number {
            max = number;
        }
        if states.contains(&columns) {
            break;
        }
        states.insert(columns.clone());
        round += 1;
    }
    max.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!(
            "2323",
            solve_part_1(
                "2 3 4 5
3 4 5 2
4 5 2 3
5 2 3 4"
            )
        );
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(
            "50877075",
            solve_part_2(
                "2 3 4 5
6 7 8 9"
            )
        );
    }

    #[test]
    fn test_simulate_one_round() {
        let mut columns = vec![vec![1, 2], vec![3, 4, 5, 6, 7]];
        simulate_one_round(&mut columns, 0);
        assert_eq!(vec![vec![2], vec![1, 3, 4, 5, 6, 7]], columns);
        let mut columns = vec![vec![2, 2], vec![3, 4, 5, 6, 7]];
        simulate_one_round(&mut columns, 0);
        assert_eq!(vec![vec![2], vec![3, 2, 4, 5, 6, 7]], columns);
        let mut columns = vec![vec![3, 2], vec![3, 4, 5, 6, 7]];
        simulate_one_round(&mut columns, 0);
        assert_eq!(vec![vec![2], vec![3, 4, 3, 5, 6, 7]], columns);
        let mut columns = vec![vec![4, 2], vec![3, 4, 5, 6, 7]];
        simulate_one_round(&mut columns, 0);
        assert_eq!(vec![vec![2], vec![3, 4, 5, 4, 6, 7]], columns);
        let mut columns = vec![vec![5, 2], vec![3, 4, 5, 6, 7]];
        simulate_one_round(&mut columns, 0);
        assert_eq!(vec![vec![2], vec![3, 4, 5, 6, 5, 7]], columns);
        let mut columns = vec![vec![6, 2], vec![1, 1, 1, 61, 71]];
        simulate_one_round(&mut columns, 0);
        assert_eq!(vec![vec![2], vec![1, 1, 1, 61, 71, 6]], columns);
        let mut columns = vec![vec![7, 2], vec![3, 4, 5, 61, 71]];
        simulate_one_round(&mut columns, 0);
        assert_eq!(vec![vec![2], vec![3, 4, 5, 61, 7, 71]], columns);
        let mut columns = vec![vec![8, 2], vec![3, 4, 5, 6, 7]];
        simulate_one_round(&mut columns, 0);
        assert_eq!(vec![vec![2], vec![3, 4, 5, 8, 6, 7]], columns);
        let mut columns = vec![vec![9, 2], vec![3, 4, 5, 6, 7]];
        simulate_one_round(&mut columns, 0);
        assert_eq!(vec![vec![2], vec![3, 4, 9, 5, 6, 7]], columns);
        let mut columns = vec![vec![10, 2], vec![3, 4, 5, 6, 7]];
        simulate_one_round(&mut columns, 0);
        assert_eq!(vec![vec![2], vec![3, 10, 4, 5, 6, 7]], columns);
        let mut columns = vec![vec![11, 2], vec![3, 4, 5, 6, 7]];
        simulate_one_round(&mut columns, 0);
        assert_eq!(vec![vec![2], vec![11, 3, 4, 5, 6, 7]], columns);
        let mut columns = vec![vec![12, 2], vec![3, 4, 5, 6, 7]];
        simulate_one_round(&mut columns, 0);
        assert_eq!(vec![vec![2], vec![3, 12, 4, 5, 6, 7]], columns);
        let mut columns = vec![vec![13, 2], vec![3, 4, 5, 6, 7]];
        simulate_one_round(&mut columns, 0);
        assert_eq!(vec![vec![2], vec![3, 4, 13, 5, 6, 7]], columns);
        let mut columns = vec![vec![14, 2], vec![3, 4, 5, 6, 7]];
        simulate_one_round(&mut columns, 0);
        assert_eq!(vec![vec![2], vec![3, 4, 5, 14, 6, 7]], columns);
        let mut columns = vec![vec![15, 2], vec![3, 4, 5, 6, 7]];
        simulate_one_round(&mut columns, 0);
        assert_eq!(vec![vec![2], vec![3, 4, 5, 6, 15, 7]], columns);
    }

    #[test]
    fn test_solve_part_3() {
        assert_eq!(
            "6584",
            solve_part_3(
                "2 3 4 5
6 7 8 9"
            )
        );
    }
}
