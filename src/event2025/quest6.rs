use std::collections::HashMap;

use itertools::Itertools;

pub fn solve_part_1(input: &str) -> String {
    let mut mentors = 0;
    let mut pairs = 0;
    for ch in input.chars() {
        match ch {
            'A' => mentors += 1,
            'a' => pairs += mentors,
            _ => {}
        }
    }
    pairs.to_string()
}

pub fn solve_part_2(input: &str) -> String {
    let mut mentors: HashMap<char, i64> = HashMap::new();
    let mut pairs = 0;
    for ch in input.chars() {
        match ch {
            'A'..='Z' => *mentors.entry(ch).or_default() += 1,
            'a'..='z' => pairs += *mentors.entry(ch.to_ascii_uppercase()).or_default(),
            _ => panic!("unexpected character {ch}"),
        }
    }
    pairs.to_string()
}

pub fn solve_part_3(input: &str) -> String {
    let data: Vec<_> = input.chars().collect();
    let len = data.len();
    let mentors: HashMap<char, Vec<usize>> = data
        .iter()
        .enumerate()
        .map(|(i, ch)| (*ch, i))
        .into_group_map();
    let mut pairs: i64 = 0;
    for (squire_position, ch) in data.into_iter().enumerate() {
        if ch.is_ascii_lowercase() {
            for mentor_position in mentors.get(&ch.to_ascii_uppercase()).unwrap() {
                if squire_position.abs_diff(*mentor_position) <= 1000 {
                    pairs += 1000;
                } else if (squire_position as isize)
                    .wrapping_sub_unsigned(len)
                    .abs_diff(*mentor_position as isize)
                    <= 1000
                    || (*mentor_position as isize)
                        .wrapping_sub_unsigned(len)
                        .abs_diff(squire_position as isize)
                        <= 1000
                {
                    pairs += 999;
                }
            }
        }
    }
    pairs.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!("5", solve_part_1("ABabACacBCbca"));
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!("11", solve_part_2("ABabACacBCbca"));
    }
}
