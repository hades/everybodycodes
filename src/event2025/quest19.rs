use std::collections::{BTreeMap};

use interval::{
    IntervalSet,
    ops::Range,
    prelude::{Bounded, Empty, Intersection, Union},
};

pub fn solve_part_1(input: &str) -> String {
    let mut data = BTreeMap::new();
    for v in input.lines().map(|l| {
        l.split(",")
            .map(|v| v.parse().unwrap())
            .collect::<Vec<i64>>()
    }) {
        data.entry(v[0]).or_insert(vec![]).push((v[1], v[2]));
    }
    let mut y_ranges = IntervalSet::new(0, 0);
    let mut x = 0;
    for (wall_x, openings) in data.into_iter() {
        let dx = wall_x - x;
        let mut new_ranges = IntervalSet::empty();
        for interval in y_ranges.into_iter() {
            new_ranges = new_ranges.union(&IntervalSet::new(
                interval.lower() - dx,
                interval.upper() + dx,
            ));
        }
        let mut openings_intervalset = IntervalSet::empty();
        for (opening_start, opening_size) in openings {
            openings_intervalset = openings_intervalset.union(&IntervalSet::new(
                opening_start,
                opening_start + opening_size - 1,
            ));
        }
        y_ranges = new_ranges.intersection(&openings_intervalset);
        x = wall_x;
    }
    let y = y_ranges
        .iter()
        .flat_map(|i| (i.lower()..=i.upper()))
        .find(|y| y % 2 == x % 2)
        .unwrap();
    ((y + x) / 2).to_string()
}

pub fn solve_part_2(input: &str) -> String {
    solve_part_1(input)
}
pub fn solve_part_3(input: &str) -> String {
    solve_part_1(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!(
            "24",
            solve_part_1(
                "7,7,2
12,0,4
15,5,3
24,1,6
28,5,5
40,8,2"
            )
        );
    }
}
