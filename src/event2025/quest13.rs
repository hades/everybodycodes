pub fn solve_part_1(input: &str) -> String {
    let numbers = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<i64>>();
    let offset = 2025 % (numbers.len() + 1);
    if offset == 0 {
        1
    } else if offset > numbers.len().div_ceil(2) {
        numbers[(numbers.len() - offset) * 2 + 1]
    } else {
        numbers[(offset - 1) * 2]
    }
    .to_string()
}

fn find_number(ranges: &[(i64, i64)], mut offset: i64, counterclockwise: bool) -> i64 {
    for (from, to) in ranges {
        let segment_size = (to - from) + 1;
        if offset >= segment_size {
            offset -= segment_size;
            continue;
        }
        return if counterclockwise {
            to - offset
        } else {
            from + offset
        };
    }
    panic!("find_number gave up and died");
}

fn solve_part_2_with_turns(input: &str, turns: i64) -> String {
    let ranges = input
        .lines()
        .map(|l| {
            let (l, r) = l.split_once("-").unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        })
        .collect::<Vec<(i64, i64)>>();
    let mut clockwise_length = 0;
    let mut clockwise_ranges = vec![];
    let mut counterclockwise_length = 0;
    let mut counterclockwise_ranges = vec![];
    for (i, (from, to)) in ranges.into_iter().enumerate() {
        if i % 2 == 0 {
            clockwise_length += to - from + 1;
            clockwise_ranges.push((from, to));
        } else {
            counterclockwise_length += to - from + 1;
            counterclockwise_ranges.push((from, to));
        }
    }
    counterclockwise_ranges.reverse();
    let offset = turns % (clockwise_length + counterclockwise_length + 1);
    if offset == 0 {
        1
    } else if offset > clockwise_length {
        find_number(
            &counterclockwise_ranges,
            offset - clockwise_length - 1,
            true,
        )
    } else {
        find_number(&clockwise_ranges, offset - 1, false)
    }
    .to_string()
}

pub fn solve_part_2(input: &str) -> String {
    solve_part_2_with_turns(input, 20252025)
}

pub fn solve_part_3(input: &str) -> String {
    solve_part_2_with_turns(input, 202520252025)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!(
            "67",
            solve_part_1(
                "72
58
47
61
67"
            )
        );
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(
            "30",
            solve_part_2(
                "10-15
12-13
20-21
19-23
30-37"
            )
        );
    }

    #[test]
    fn test_solve_part_3() {}
}
