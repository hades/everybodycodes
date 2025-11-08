use num::{BigInt, Integer};

pub fn solve_part_1(input: &str) -> String {
    let gears: Vec<i64> = input.trim().lines().map(|g| g.parse().unwrap()).collect();
    (2025 * gears[0] / gears.last().unwrap()).to_string()
}

pub fn solve_part_2(input: &str) -> String {
    let gears: Vec<i64> = input.trim().lines().map(|g| g.parse().unwrap()).collect();
    let res = (BigInt::parse_bytes(b"10000000000000", 10).unwrap() * gears.last().unwrap())
        .div_ceil(&(BigInt::ZERO + gears[0]));
    res.to_string()
}

pub fn solve_part_3(input: &str) -> String {
    let mut lines = input.trim().lines();
    let first_gear = BigInt::parse_bytes(lines.next().unwrap().as_bytes(), 10).unwrap();
    let mut nominator: BigInt = first_gear * 100;
    let mut denominator: BigInt = BigInt::ZERO + 1;
    for line in lines {
        let mut split = line.split("|");
        denominator *= BigInt::parse_bytes(split.next().unwrap().as_bytes(), 10).unwrap();
        match split.next() {
            Some(size) => {
                nominator *= BigInt::parse_bytes(size.as_bytes(), 10).unwrap();
            }
            None => {
                break;
            }
        }
    }
    (nominator / denominator).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!(
            "32400",
            solve_part_1(
                "128
64
32
16
8"
            )
        );
        assert_eq!(
            "15888",
            solve_part_1(
                "102
75
50
35
13"
            )
        );
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(
            "625000000000",
            solve_part_2(
                "128
64
32
16
8"
            )
        );
        assert_eq!(
            "1274509803922",
            solve_part_2(
                "102
75
50
35
13"
            )
        );
    }

    #[test]
    fn test_solve_part_3() {
        assert_eq!(
            "400",
            solve_part_3(
                "5
5|10
10|20
5"
            )
        );
        assert_eq!(
            "6818",
            solve_part_3(
                "5
7|21
18|36
27|27
10|50
10|50
11"
            )
        );
    }
}
