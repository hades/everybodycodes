fn build_wall(numbers: &[i64], length: usize) -> Vec<i64> {
    let mut divisors = vec![0; length];
    for n in numbers {
        for (i, d) in divisors.iter_mut().enumerate() {
            if (i + 1) as i64 % n == 0 {
                *d += 1;
            }
        }
    }
    divisors
}

pub fn solve_part_1(input: &str) -> String {
    let numbers = input
        .split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    build_wall(&numbers, 90).iter().sum::<i64>().to_string()
}

fn solve(
    divisor_masks: &Vec<Vec<i32>>,
    selected_divisors: Vec<i64>,
    values: &Vec<i64>,
    next_divisor_to_try: i64,
) -> Option<Vec<i64>> {
    if values.iter().all(|v| *v == 0) {
        return Some(selected_divisors);
    }
    if values.iter().any(|v| *v < 0) {
        return None;
    }
    if next_divisor_to_try as usize > values.len() {
        return None;
    }
    let next_values = values
        .iter()
        .zip(divisor_masks[(next_divisor_to_try - 1) as usize].iter())
        .map(|(v, d)| *v - *d as i64)
        .collect();
    let mut next_selected_divisors = selected_divisors.clone();
    next_selected_divisors.push(next_divisor_to_try);
    if let Some(result) = solve(
        divisor_masks,
        next_selected_divisors,
        &next_values,
        next_divisor_to_try + 1,
    ) {
        return Some(result);
    }
    solve(
        divisor_masks,
        selected_divisors,
        values,
        next_divisor_to_try + 1,
    )
}

pub fn solve_part_2_raw(input: &str) -> Vec<i64> {
    let values = input
        .split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut divisor_masks = vec![vec![0; values.len()]; values.len()];
    for (i, mask) in divisor_masks.iter_mut().enumerate() {
        let divisor = i + 1;
        for (j, d) in mask.iter_mut().enumerate() {
            let number = j + 1;
            if number % divisor == 0 {
                *d += 1;
            }
        }
    }
    solve(&divisor_masks, vec![], &values, 1).unwrap()
}

pub fn solve_part_2(input: &str) -> String {
    solve_part_2_raw(input).iter().product::<i64>().to_string()
}

pub fn solve_part_3(input: &str) -> String {
    // find max N such that
    // sum_1^N f(i)
    // f(i) sum_{d \in divisors} {1 if i mod d == 0 else 0}
    // f is periodic in lcm(divisors)
    //
    // blocks(N) = N/divisor1 + N/divisor2 + ... + N/divisork
    // max N s.t. block(N) <= blocks
    let divisors = solve_part_2_raw(input);
    let blocks = 202520252025000i64;
    let mut l = 1i64;
    let mut r = 108420091881608i64;
    while r - l > 1 {
        let mid = (r + l) / 2;
        let b = divisors.iter().map(|d| mid / d).sum::<i64>();
        if b > blocks {
            r = mid;
        } else {
            l = mid;
        }
    }
    l.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!("193", solve_part_1("1,2,3,5,9"));
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(
            "270",
            solve_part_2("1,2,2,2,2,3,1,2,3,3,1,3,1,2,3,2,1,4,1,3,2,2,1,3,2,2")
        );
    }

    #[test]
    fn test_solve_part_3() {
        assert_eq!(
            "94439495762954",
            solve_part_3("1,2,2,2,2,3,1,2,3,3,1,3,1,2,3,2,1,4,1,3,2,2,1,3,2,2")
        );
    }
}
