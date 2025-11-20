use num::Integer;

fn one_round(columns: &mut [i64], forward: bool) -> bool {
    let mut modified = false;
    for i in 1..columns.len() {
        if forward {
            if columns[i - 1] > columns[i] {
                columns[i - 1] -= 1;
                columns[i] += 1;
                modified = true;
            }
        } else if columns[i - 1] < columns[i] {
            columns[i - 1] += 1;
            columns[i] -= 1;
            modified = true;
        }
    }
    modified
}

pub fn solve_part_1(input: &str) -> String {
    let mut columns = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<i64>>();
    let mut rounds_remaining = 10;
    while rounds_remaining > 0 {
        if one_round(&mut columns, true) {
            rounds_remaining -= 1;
        } else {
            break;
        }
    }
    while rounds_remaining > 0 {
        if one_round(&mut columns, false) {
            rounds_remaining -= 1;
        } else {
            break;
        }
    }
    columns
        .iter()
        .enumerate()
        .map(|(column_idx, count)| (column_idx + 1) as i64 * *count)
        .sum::<i64>()
        .to_string()
}

pub fn solve_part_2(input: &str) -> String {
    let mut columns = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<i64>>();
    let mut total_rounds = 0;
    loop {
        if one_round(&mut columns, true) {
            total_rounds += 1;
        } else {
            break;
        }
    }
    loop {
        if one_round(&mut columns, false) {
            total_rounds += 1;
        } else {
            break;
        }
    }
    total_rounds.to_string()
}

pub fn solve_part_3(input: &str) -> String {
    let mut columns = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<i64>>();
    let invariant_sum = columns.iter().sum::<i64>();
    let mut total_rounds = 0i64;
    loop {
        let first_gap = (0..columns.len() - 1).find(|&i| columns[i].abs_diff(columns[i + 1]) > 1);
        let last_gap = (0..columns.len() - 1)
            .filter(|&i| columns[i].abs_diff(columns[i + 1]) > 1)
            .next_back();
        if let (Some(first_gap), Some(last_gap)) = (first_gap, last_gap)
            && (last_gap > first_gap)
        {
            let amount_to_fill: i64 = columns
                .iter()
                .take(first_gap + 1)
                .map(|&x| columns[first_gap + 1] - x)
                .sum();
            let amount_available: i64 = columns
                .iter()
                .skip(last_gap + 1)
                .map(|&x| x - columns[last_gap])
                .sum();
            let amount_to_transfer = amount_to_fill.min(amount_available);
            let new_amount_left: i64 =
                columns.iter().take(first_gap + 1).sum::<i64>() + amount_to_transfer;
            let (left_base, remainder) = new_amount_left.div_rem(&((first_gap + 1) as i64));
            for (i, new_value) in columns.iter_mut().enumerate().take(first_gap + 1) {
                *new_value = left_base
                    + if first_gap - i < remainder as usize {
                        1
                    } else {
                        0
                    };
            }
            let new_amount_right: i64 =
                columns.iter().skip(last_gap + 1).sum::<i64>() - amount_to_transfer;
            let (right_base, remainder) =
                new_amount_right.div_rem(&((columns.len() - last_gap - 1) as i64));
            for i in last_gap + 1..columns.len() {
                columns[i] = right_base
                    + if columns.len() - i <= remainder as usize {
                        1
                    } else {
                        0
                    };
            }
            assert_eq!(invariant_sum, columns.iter().sum::<i64>());
            total_rounds += amount_to_transfer;
        } else {
            break;
        }
    }
    loop {
        if one_round(&mut columns, false) {
            total_rounds += 1;
        } else {
            break;
        }
    }
    total_rounds.to_string()

    // a_1 < a_2, ...< a_n
    //   after k_1 steps, k = a_2 - a_1
    // a_2, a_2, a_3, ..., a_n - k_1
    //
    // a_3 - 1, a_3 - 1, a_3, ..., a_n - k_1 - k_2 + 2
    // a_3 - 1, a_3    , a_3, ..., a_n - k_1 - k_2 + 1
    // a_3,     a_3,     a_3, ..., a_n - k_1 - k_2 ;     after k_2 steps, a_2 + k/2 = a_3
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!(
            "109",
            solve_part_1(
                "9
1
1
4
9
6"
            )
        );
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(
            "11",
            solve_part_2(
                "9
1
1
4
9
6"
            )
        );
        assert_eq!(
            "1579",
            solve_part_2(
                "805
706
179
48
158
150
232
885
598
524
423"
            )
        );
    }

    #[test]
    fn test_solve_part_3() {}
}
