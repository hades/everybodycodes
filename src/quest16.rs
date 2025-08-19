use std::cmp::{max, min};
use std::collections::HashMap;

use itertools::Itertools;

fn parse_machine(input: &str) -> (Vec<usize>, Vec<Vec<&str>>) {
    let shifts: Vec<usize> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut wheels = Vec::new();
    wheels.extend(shifts.iter().map(|_| Vec::new()));
    for line in input.lines().skip(2) {
        let mut start_pos = 0;
        let mut wheel_no = 0;
        loop {
            if line.len() < start_pos + 3 {
                break;
            }
            let face = &line[start_pos..start_pos + 3];
            if face != "   " {
                wheels[wheel_no].push(face);
            }
            start_pos += 4;
            wheel_no += 1;
        }
    }
    (shifts, wheels)
}

pub fn solve_part_1(input: &str) -> String {
    let (shifts, wheels) = parse_machine(input);
    (0..wheels.len())
        .map(|i| wheels[i][(100 * shifts[i]) % wheels[i].len()])
        .join(" ")
}

fn evaluate(combination: &str) -> usize {
    let eyes: Vec<char> = combination
        .chars()
        .enumerate()
        .filter(|(i, _)| i % 4 != 1 && i % 4 != 3)
        .map(|(_, ch)| ch)
        .collect();
    eyes.into_iter()
        .counts()
        .into_values()
        .map(|count| match count {
            ..3 => 0,
            more_than_three => more_than_three - 2,
        })
        .sum()
}

fn solve_part_2_with_count(input: &str, count: usize) -> String {
    let (shifts, wheels) = parse_machine(input);
    let mut total_winnings = 0;
    let mut positions = Vec::new();
    let mut winnings_after_pull = vec![0];
    let mut cycle_length = 0;
    positions.extend(shifts.iter().map(|_| 0));
    loop {
        cycle_length += 1;
        shifts.iter().enumerate().for_each(|(wheel_no, shift)| {
            positions[wheel_no] = (positions[wheel_no] + shift) % wheels[wheel_no].len()
        });
        let combination = positions
            .iter()
            .enumerate()
            .map(|(wheel_no, position)| wheels[wheel_no][*position])
            .join(" ");
        total_winnings += evaluate(&combination);
        winnings_after_pull.push(total_winnings);
        if positions.iter().all(|shift| *shift == 0) {
            break;
        }
    }
    let remainder = count % cycle_length;
    let cycle_count = count / cycle_length;
    (total_winnings * cycle_count + winnings_after_pull[remainder]).to_string()
}

pub fn solve_part_2(input: &str) -> String {
    solve_part_2_with_count(input, 202420242024)
}

pub fn solve_part_3(input: &str) -> String {
    let (shifts, wheels) = parse_machine(input);
    let mut current_states: HashMap<Vec<_>, (usize, usize)> = HashMap::new();
    current_states.insert(shifts.iter().map(|_| 0).collect(), (0, 0));
    for _ in 1..=256 {
        let mut next_states = HashMap::new();
        for (positions, current_winnings) in current_states.drain() {
            for additional_shift in -1isize..=1isize {
                let mut positions = positions.clone();
                shifts.iter().enumerate().for_each(|(wheel_no, shift)| {
                    positions[wheel_no] =
                        (positions[wheel_no] as isize + *shift as isize + additional_shift)
                            % wheels[wheel_no].len() as isize;
                });
                let combination = positions
                    .iter()
                    .enumerate()
                    .map(|(wheel_no, position)| wheels[wheel_no][*position as usize])
                    .join(" ");
                let winnings = evaluate(&combination);
                let total_winnings = (current_winnings.0 + winnings, current_winnings.1 + winnings);
                match next_states.entry(positions) {
                    std::collections::hash_map::Entry::Occupied(mut e) => {
                        let (old_min_winnings, old_max_winnings) = *e.get();
                        e.get_mut().0 = min(old_min_winnings, total_winnings.0);
                        e.get_mut().1 = max(old_max_winnings, total_winnings.1);
                    }
                    std::collections::hash_map::Entry::Vacant(e) => {
                        e.insert(total_winnings);
                    }
                }
            }
        }
        current_states = next_states;
    }
    let winnings: Vec<_> = current_states.into_values().collect();
    format!(
        "{} {}",
        winnings.iter().map(|(_, m)| m).max().unwrap(),
        winnings.iter().map(|(m, _)| m).min().unwrap()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!(
            ">.- -.- ^,-",
            solve_part_1(
                "1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- >.<
    -.^ ^_^
    >.>"
            )
        );
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(
            "15",
            solve_part_2_with_count(
                "1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- >.<
    -.^ ^_^
    >.>",
                10
            )
        );
        assert_eq!(
            "138",
            solve_part_2_with_count(
                "1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- >.<
    -.^ ^_^
    >.>",
                100
            )
        );
        assert_eq!(
            "1383",
            solve_part_2_with_count(
                "1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- >.<
    -.^ ^_^
    >.>",
                1000
            )
        );
        assert_eq!(
            "138333333333",
            solve_part_2_with_count(
                "1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- >.<
    -.^ ^_^
    >.>",
                100000000000
            )
        );
        assert_eq!(
            "280014668134",
            solve_part_2(
                "1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- >.<
    -.^ ^_^
    >.>"
            )
        );
    }

    #[test]
    fn test_evaluate() {
        assert_eq!(0, evaluate("*;# ~.0 `;+ 0,) U.} S,= #;O %_$ x,G );@"));
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            "627 128",
            solve_part_3(
                "1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- ^.^
    -.^ >.<
    >.>"
            )
        );
    }
}
