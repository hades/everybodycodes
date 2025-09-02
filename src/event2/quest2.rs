use std::iter::repeat;

pub fn solve_part_1(input: &str) -> String {
    let mut i = 0;
    let mut b = 0;
    let colors = ['R', 'G', 'B'];
    let input: Vec<_> = input.chars().collect();
    while i < input.len() {
        while i < input.len() && input[i] == colors[b % 3] {
            i += 1;
        }
        i += 1;
        b += 1;
    }
    b.to_string()
}

pub fn solve_part_2(input: &str) -> String {
    let mut left: Vec<_> = repeat(input).take(50).flat_map(|s| s.chars()).collect();
    let right = left.clone();
    let mut left_next_balloon = 0;
    let mut right_next_balloon = 0;
    let mut shots_taken = 0;
    let colours = ['R', 'G', 'B'];
    while left_next_balloon < left.len() {
        if ((left.len() - left_next_balloon) + (right.len() - right_next_balloon)) % 2 == 0 {
            if colours[shots_taken % 3] == left[left_next_balloon] {
                right_next_balloon += 1;
            } else {
                left.push(right[right_next_balloon]);
                right_next_balloon += 1;
            }
        }
        left_next_balloon += 1;
        shots_taken += 1;
    }
    shots_taken.to_string()
}

pub fn solve_part_3(input: &str) -> String {
    let mut left: Vec<_> = repeat(input).take(50000).flat_map(|s| s.chars()).collect();
    let right = left.clone();
    let mut left_next_balloon = 0;
    let mut right_next_balloon = 0;
    let mut shots_taken = 0;
    let colours = ['R', 'G', 'B'];
    while left_next_balloon < left.len() {
        if ((left.len() - left_next_balloon) + (right.len() - right_next_balloon)) % 2 == 0 {
            if colours[shots_taken % 3] == left[left_next_balloon] {
                right_next_balloon += 1;
            } else {
                left.push(right[right_next_balloon]);
                right_next_balloon += 1;
            }
        }
        left_next_balloon += 1;
        shots_taken += 1;
    }
    shots_taken.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_part_one() {
        assert_eq!("7", solve_part_1("GRBGGGBBBRRRRRRRR"));
    }

    #[test]
    fn test_part_two() {
        assert_eq!("300", solve_part_2("GGBR"));
    }
}
