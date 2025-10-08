use log::debug;
use num::integer::lcm;
use regex::Regex;
use ring_algorithm::chinese_remainder_theorem;

/// Returns disc-based coordinates: (disc_no, disc_position),
/// where disc_no is 1-based, disc position is 0..disc_no.
fn to_disc_coords((x, y): (i64, i64)) -> (i64, i64) {
    (x + y - 1, x - 1)
}

fn to_flat_coords((disc_no, disc_position): (i64, i64)) -> (i64, i64) {
    (disc_position + 1, disc_no - disc_position)
}

pub fn solve_part_one(input: &str) -> String {
    let re = Regex::new(r"x=(\d+) y=(\d+)").unwrap();
    let mut snails = vec![];
    for (_, [x_str, y_str]) in re.captures_iter(input).map(|c| c.extract()) {
        snails.push(to_disc_coords((
            x_str.parse().unwrap(),
            y_str.parse().unwrap(),
        )));
    }
    snails
        .into_iter()
        .map(|(disc_no, disc_position)| {
            let disc_position = (disc_position + 100) % disc_no;
            debug!("{disc_no} {disc_position}");
            let (x, y) = to_flat_coords((disc_no, disc_position));
            x + (100 * y)
        })
        .sum::<i64>()
        .to_string()
}

pub fn solve_part_two(input: &str) -> String {
    let re = Regex::new(r"x=(\d+) y=(\d+)").unwrap();
    let mut snails = vec![];
    for (_, [x_str, y_str]) in re.captures_iter(input).map(|c| c.extract()) {
        snails.push(to_disc_coords((
            x_str.parse().unwrap(),
            y_str.parse().unwrap(),
        )));
    }
    let a: Vec<_> = snails
        .iter()
        .map(|(disc_no, disc_position)| disc_no - disc_position - 1)
        .collect();
    let m: Vec<_> = snails.iter().map(|(disc_no, _)| *disc_no).collect();
    debug!("{a:?} {m:?}");
    let n = chinese_remainder_theorem(a.as_slice(), m.as_slice()).unwrap();
    let lcm = m.into_iter().reduce(lcm).unwrap();
    ((n + lcm) % lcm).to_string()
}

pub fn solve_part_three(input: &str) -> String {
    solve_part_two(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_part_one() {
        assert_eq!(
            "1310",
            solve_part_one(
                "x=1 y=2
x=2 y=3
x=3 y=4
x=4 y=4"
            )
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            "14",
            solve_part_two(
                "x=12 y=2
x=8 y=4
x=7 y=1
x=1 y=5
x=1 y=3"
            )
        );
        assert_eq!(
            "13659",
            solve_part_two(
                "x=3 y=1
x=3 y=9
x=1 y=5
x=4 y=10
x=5 y=3"
            )
        );
    }
}
