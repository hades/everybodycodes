pub fn solve_part_1(input: &str) -> String {
    let lengths: Vec<i64> = input.lines().map(|line| line.parse().unwrap()).collect();
    let target = lengths.iter().min().unwrap();
    let strikes: i64 = lengths.iter().map(|l| l - target).sum();
    strikes.to_string()
}

pub fn solve_part_3(input: &str) -> String {
    let mut lengths: Vec<i64> = input.lines().map(|line| line.parse().unwrap()).collect();
    lengths.sort();
    let target = lengths[lengths.len() / 2];
    let strikes: i64 = lengths.iter().map(|l| (l - target).abs()).sum();
    strikes.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!(
            "10",
            solve_part_1(
                "3
4
7
8"
            )
        );
    }

    #[test]
    fn test_solve_part_3() {
        assert_eq!(
            "8",
            solve_part_3(
                "2
4
5
6
8"
            )
        );
    }
}
