pub fn solve_part_1(input: &str) -> String {
    let numbers: Vec<i32> = input.split(",").map(|x| x.parse().unwrap()).collect();
    let mut count = 0;
    for i in 1..numbers.len() {
        if numbers[i].abs_diff(numbers[i - 1]) == 16 {
            count += 1;
        }
    }
    count.to_string()
}

pub fn solve_part_2(input: &str) -> String {
    let numbers: Vec<i32> = input.split(",").map(|x| x.parse().unwrap()).collect();
    let mut lines: Vec<(i32, i32)> = vec![];
    for i in 1..numbers.len() {
        let (a, b) = (numbers[i - 1], numbers[i]);
        if a > b {
            lines.push((b, a));
        } else {
            lines.push((a, b));
        }
    }
    let mut knots = 0;
    for i in 0..lines.len() {
        for j in 0..i {
            let (a, b) = lines[i];
            let (c, d) = lines[j];
            if a == c || a == d || b == c || b == d {
                continue;
            }
            let c_inside = c > a && c < b;
            let d_inside = d > a && d < b;
            if c_inside != d_inside {
                knots += 1;
            }
        }
    }
    knots.to_string()
}

pub fn solve_part_3(input: &str) -> String {
    let numbers: Vec<i32> = input.split(",").map(|x| x.parse().unwrap()).collect();
    let mut lines: Vec<(i32, i32)> = vec![];
    for i in 1..numbers.len() {
        let (a, b) = (numbers[i - 1], numbers[i]);
        if a > b {
            lines.push((b, a));
        } else {
            lines.push((a, b));
        }
    }
    let mut best_cut_threads = i64::MIN;
    for d in 1..=256 {
        for c in 1..d {
            let mut cut_threads = 0;
            for (a, b) in lines.iter().copied() {
                if a == c || a == d || b == c || b == d {
                    if a == c && b == d {
                        cut_threads += 1;
                    }
                    continue;
                }
                let c_inside = c > a && c < b;
                let d_inside = d > a && d < b;
                if c_inside != d_inside {
                    cut_threads += 1;
                }
            }
            if cut_threads > best_cut_threads {
                best_cut_threads = cut_threads;
            }
        }
    }
    best_cut_threads.to_string()
}
