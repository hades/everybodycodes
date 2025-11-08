pub fn solve_part_1(input: &str) -> String {
    let mut crates: Vec<i64> = input.split(",").map(|s| s.parse().unwrap()).collect();
    crates.sort();
    let mut monotonic_subsequence = vec![crates[0]];
    for size in crates.into_iter().skip(1) {
        if size == *monotonic_subsequence.last().unwrap() {
            continue;
        }
        monotonic_subsequence.push(size);
    }
    monotonic_subsequence.iter().sum::<i64>().to_string()
}

pub fn solve_part_2(input: &str) -> String {
    let mut crates: Vec<i64> = input.split(",").map(|s| s.parse().unwrap()).collect();
    crates.sort();
    let mut monotonic_subsequence = vec![crates[0]];
    for size in crates.into_iter().skip(1) {
        if size == *monotonic_subsequence.last().unwrap() {
            continue;
        }
        monotonic_subsequence.push(size);
        if monotonic_subsequence.len() >= 20 {
            break;
        }
    }
    monotonic_subsequence.iter().sum::<i64>().to_string()
}

pub fn solve_part_3(input: &str) -> String {
    let mut crates: Vec<i64> = input.split(",").map(|s| s.parse().unwrap()).collect();
    crates.sort();
    let mut monotonic_subsequences = vec![vec![crates[0]]];
    for size in crates.into_iter().skip(1) {
        let updateable_sequence = monotonic_subsequences
            .iter_mut()
            .find(|v| *v.last().unwrap() < size);
        match updateable_sequence {
            Some(v) => {
                v.push(size);
            }
            None => {
                monotonic_subsequences.push(vec![size]);
            }
        }
    }
    monotonic_subsequences.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!("29", solve_part_1("10,5,1,10,3,8,5,2,2"));
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(
            "781",
            solve_part_2(
                "4,51,13,64,57,51,82,57,16,88,89,48,32,49,49,2,84,65,49,43,9,13,2,3,75,72,63,48,61,14,40,77"
            )
        );
    }

    #[test]
    fn test_solve_part_3() {
        assert_eq!(
            "3",
            solve_part_3(
                "4,51,13,64,57,51,82,57,16,88,89,48,32,49,49,2,84,65,49,43,9,13,2,3,75,72,63,48,61,14,40,77"
            )
        );
    }
}
