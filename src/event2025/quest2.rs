use log::debug;
use std::collections::HashSet;

use regex::Regex;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Number(isize, isize);

impl Number {
    fn add(self: &Number, b: &Number) -> Number {
        Number(self.0 + b.0, self.1 + b.1)
    }

    fn mul(self: &Number, b: &Number) -> Number {
        Number(self.0 * b.0 - self.1 * b.1, self.0 * b.1 + self.1 * b.0)
    }

    fn div(self: &Number, b: &Number) -> Number {
        Number(self.0 / b.0, self.1 / b.1)
    }
}

pub fn solve_part_1(input: &str) -> String {
    let re = Regex::new(r"A=\[(\d+),(\d+)\]").unwrap();
    let (_, [x, y]) = re.captures(input).unwrap().extract();
    let a = Number(x.parse().unwrap(), y.parse().unwrap());
    let mut res = Number(0, 0);
    for _ in 0..3 {
        res = res.mul(&res);
        res = res.div(&Number(10, 10));
        res = res.add(&a);
    }
    format!("[{},{}]", res.0, res.1)
}

pub fn solve_part_2(input: &str) -> String {
    let re = Regex::new(r"A=\[([-0-9]+),([-0-9]+)\]").unwrap();
    let (_, [x, y]) = re.captures(input).unwrap().extract();
    let a = Number(x.parse().unwrap(), y.parse().unwrap());
    let mut engraved_points = 0;
    let mut pts: HashSet<_> = HashSet::new();
    for i in 0..=100 {
        for j in 0..=100 {
            let pt = Number(a.0 + 10 * i, a.1 + 10 * j);
            let mut res = Number(0, 0);
            engraved_points += 1;
            pts.insert(pt.clone());
            for _ in 0..100 {
                res = res.mul(&res);
                res = res.div(&Number(100_000, 100_000));
                res = res.add(&pt);
                if res.0.abs() > 1_000_000 || res.1.abs() > 1_000_000 {
                    engraved_points -= 1;
                    pts.remove(&pt);
                    break;
                }
            }
        }
    }
    for i in 0..=100 {
        debug!(
            "{}",
            (0..=100)
                .map(|j| if pts.contains(&Number(a.0 + 10 * i, a.1 + 10 * j)) {
                    'X'
                } else {
                    '.'
                })
                .collect::<String>()
        );
    }
    engraved_points.to_string()
}

pub fn solve_part_3(input: &str) -> String {
    let re = Regex::new(r"A=\[([-0-9]+),([-0-9]+)\]").unwrap();
    let (_, [x, y]) = re.captures(input).unwrap().extract();
    let a = Number(x.parse().unwrap(), y.parse().unwrap());
    let mut engraved_points = 0;
    for i in 0..=1000 {
        for j in 0..=1000 {
            let pt = Number(a.0 + i, a.1 + j);
            let mut res = Number(0, 0);
            engraved_points += 1;
            for _ in 0..100 {
                res = res.mul(&res);
                res = res.div(&Number(100_000, 100_000));
                res = res.add(&pt);
                if res.0.abs() > 1_000_000 || res.1.abs() > 1_000_000 {
                    engraved_points -= 1;
                    break;
                }
            }
        }
    }
    engraved_points.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!("[357,862]", solve_part_1("A=[25,9]"));
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!("4076", solve_part_2("A=[35300,-64910]"));
    }

    #[test]
    fn test_solve_part_3() {
        assert_eq!("406954", solve_part_3("A=[35300,-64910]"));
    }
}
