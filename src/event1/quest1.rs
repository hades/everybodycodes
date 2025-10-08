use crate::util::concatenate_numbers;

#[derive(Default)]
struct Params {
    a: i64,
    b: i64,
    c: i64,
    x: i64,
    y: i64,
    z: i64,
    m: i64,
}

fn eni(n: i64, exp: i64, modulus: i64) -> i64 {
    let mut res: i64 = 0;
    let mut score = 1;
    for _ in 0..exp {
        let remainder = (score * n) % modulus;
        res = if res != 0 {
            concatenate_numbers(remainder, res)
        } else {
            remainder
        };
        score = remainder;
    }
    res
}

fn eni_part_2(n: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut score = 1;
    if exp > 10 {
        let mut remaining_exp = exp - 5;
        let mut current_power = n;
        while remaining_exp > 0 {
            if remaining_exp & 1 == 1 {
                score = (score * current_power) % modulus;
            }
            current_power = (current_power * current_power) % modulus;
            remaining_exp >>= 1;
        }
        exp = 5;
    }
    let mut res = vec![];
    for _ in 0..exp {
        let remainder = (score * n) % modulus;
        res.push(remainder);
        score = remainder;
    }
    res.reverse();
    res.into_iter().take(5).reduce(concatenate_numbers).unwrap()
}

fn eni_part_3(n: i64, exp: i64, modulus: i64) -> i64 {
    let mut score = 1;
    let mut res = vec![];
    for _ in 0..exp {
        let remainder = (score * n) % modulus;
        if let Some(cycle_start_offset) = res.iter().position(|&prev| prev == remainder) {
            // Cycle of remainders:
            // r_0, r_1, r_2, ..., r_{cycle_start_offset}, ..., r_n
            let cycle_length = res.len() - cycle_start_offset;
            let cycle_count = (exp as usize - cycle_start_offset) / cycle_length;
            let remaining_exponents =
                exp as usize - cycle_start_offset - cycle_length * cycle_count;
            return res.iter().take(cycle_start_offset).sum::<i64>()
                + (cycle_count as i64) * res[cycle_start_offset..].iter().sum::<i64>()
                + res[cycle_start_offset..cycle_start_offset + remaining_exponents]
                    .iter()
                    .sum::<i64>();
        }
        res.push(remainder);
        score = remainder;
    }
    res.iter().sum()
}

fn eval(p: &Params, eni_fn: &mut dyn FnMut(i64, i64, i64) -> i64) -> i64 {
    eni_fn(p.a, p.x, p.m) + eni_fn(p.b, p.y, p.m) + eni_fn(p.c, p.z, p.m)
}

fn parse_params(input: &str) -> Vec<Params> {
    input
        .lines()
        .map(|line| {
            let mut p = Params::default();
            line.split(' ').for_each(|param_def| {
                let mut split = param_def.split('=');
                let prop = split.next().unwrap();
                let val = split.next().unwrap().parse::<i64>().unwrap();
                match prop {
                    "A" => p.a = val,
                    "B" => p.b = val,
                    "C" => p.c = val,
                    "X" => p.x = val,
                    "Y" => p.y = val,
                    "Z" => p.z = val,
                    "M" => p.m = val,
                    _ => unreachable!(),
                }
            });
            p
        })
        .collect()
}

pub fn solve_part_1(input: &str) -> String {
    parse_params(input)
        .iter()
        .map(|p| eval(p, &mut eni))
        .max()
        .unwrap()
        .to_string()
}

pub fn solve_part_2(input: &str) -> String {
    parse_params(input)
        .iter()
        .map(|p| eval(p, &mut eni_part_2))
        .max()
        .unwrap()
        .to_string()
}

pub fn solve_part_3(input: &str) -> String {
    parse_params(input)
        .iter()
        .map(|p| eval(p, &mut eni_part_3))
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_eni() {
        assert_eq!(1342, eni(2, 4, 5));
    }

    #[test]
    fn test_part_one() {
        assert_eq!("114644", solve_part_1("A=4 B=4 C=6 X=3 Y=4 Z=5 M=11"));
        assert_eq!(
            "11611972920",
            solve_part_1(
                "A=4 B=4 C=6 X=3 Y=4 Z=5 M=11
A=8 B=4 C=7 X=8 Y=4 Z=6 M=12
A=2 B=8 C=6 X=2 Y=4 Z=5 M=13
A=5 B=9 C=6 X=8 Y=6 Z=8 M=14
A=5 B=9 C=7 X=6 Y=6 Z=8 M=15
A=8 B=8 C=8 X=6 Y=9 Z=6 M=16"
            )
        );
    }

    #[test]
    fn test_eni_2() {
        assert_eq!(111931, eni_part_2(3, 8, 16));
        assert_eq!(10510510, eni_part_2(5, 6, 15));
        assert_eq!(69696, eni_part_2(9, 16, 15));
        assert_eq!(471134, eni_part_2(7, 18, 15));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            "11051340",
            solve_part_2(
                "A=4 B=4 C=6 X=3 Y=14 Z=15 M=11
A=8 B=4 C=7 X=8 Y=14 Z=16 M=12
A=2 B=8 C=6 X=2 Y=14 Z=15 M=13
A=5 B=9 C=6 X=8 Y=16 Z=18 M=14
A=5 B=9 C=7 X=6 Y=16 Z=18 M=15
A=8 B=8 C=8 X=6 Y=19 Z=16 M=16"
            )
        );
        assert_eq!(
            "1507702060886",
            solve_part_2(
                "A=3657 B=3583 C=9716 X=903056852 Y=9283895500 Z=85920867478 M=188
A=6061 B=4425 C=5082 X=731145782 Y=1550090416 Z=87586428967 M=107
A=7818 B=5395 C=9975 X=122388873 Y=4093041057 Z=58606045432 M=102
A=7681 B=9603 C=5681 X=716116871 Y=6421884967 Z=66298999264 M=196
A=7334 B=9016 C=8524 X=297284338 Y=1565962337 Z=86750102612 M=145"
            )
        );
    }

    #[test]
    fn test_eni_3() {
        assert_eq!(19, eni_part_3(2, 7, 5));
        assert_eq!(48, eni_part_3(3, 8, 16));
    }

    #[test]
    fn test_part_three() {
        assert_eq!(
            "1573000",
            solve_part_3("A=4 B=4 C=6 X=3000 Y=14000 Z=15000 M=110")
        );
        assert_eq!(
            "1439940",
            solve_part_3("A=8 B=4 C=7 X=8000 Y=14000 Z=16000 M=120")
        );
        assert_eq!(
            "2079860",
            solve_part_3("A=2 B=8 C=6 X=2000 Y=14000 Z=15000 M=130")
        );
        assert_eq!(
            "2407850",
            solve_part_3("A=5 B=9 C=6 X=8000 Y=16000 Z=18000 M=140")
        );
        assert_eq!(
            "2099880",
            solve_part_3("A=5 B=9 C=7 X=6000 Y=16000 Z=18000 M=150")
        );
        assert_eq!(
            "3279640",
            solve_part_3("A=8 B=8 C=8 X=6000 Y=19000 Z=16000 M=160")
        );
    }
}
