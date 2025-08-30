use std::collections::HashMap;

fn count_generations(data: &Vec<(&str, Vec<&str>)>, id: &str, gens: u8) -> u64 {
    let mut pops = HashMap::new();
    pops.insert(id, 1);
    for _ in 0..gens {
        let mut next_day = HashMap::new();
        for (id, pop) in pops.iter() {
            for child in data.iter().find(|e| e.0 == *id).unwrap().1.iter() {
                *next_day.entry(*child).or_insert(0) += pop;
            }
        }
        pops = next_day;
    }
    pops.into_values().sum()
}

pub fn solve_part_1(input: &str) -> String {
    let data: Vec<(&str, Vec<&str>)> = input
        .lines()
        .map(|line| {
            let mut split = line.split(":");
            let id = split.next().unwrap();
            let children = split.next().unwrap().split(',').collect();
            (id, children)
        })
        .collect();
    count_generations(&data, "A", 4).to_string()
}

pub fn solve_part_2(input: &str) -> String {
    let data: Vec<(&str, Vec<&str>)> = input
        .lines()
        .map(|line| {
            let mut split = line.split(":");
            let id = split.next().unwrap();
            let children = split.next().unwrap().split(',').collect();
            (id, children)
        })
        .collect();
    count_generations(&data, "Z", 10).to_string()
}

pub fn solve_part_3(input: &str) -> String {
    let data: Vec<(&str, Vec<&str>)> = input
        .lines()
        .map(|line| {
            let mut split = line.split(":");
            let id = split.next().unwrap();
            let children = split.next().unwrap().split(',').collect();
            (id, children)
        })
        .collect();
    let ids: Vec<&str> = data.iter().map(|entry| entry.0).collect();
    let pops: Vec<u64> = ids
        .iter()
        .map(|id| count_generations(&data, id, 20))
        .collect();
    (pops.iter().max().unwrap() - pops.iter().min().unwrap()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!(
            "8",
            solve_part_1(
                "A:B,C
B:C,A
C:A"
            )
        );
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(
            "144",
            solve_part_2(
                "Z:B,C
B:C,Z
C:Z"
            )
        );
    }

    #[test]
    fn test_solve_part_3() {
        assert_eq!(
            "268815",
            solve_part_3(
                "A:B,C
B:C,A,A
C:A"
            )
        );
    }
}
