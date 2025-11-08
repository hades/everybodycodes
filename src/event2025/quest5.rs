use itertools::Itertools;

type Fishbone = Vec<(i64, Option<i64>, Option<i64>)>;

fn parse_fishbone(quality_str: &str) -> Fishbone {
    let mut fishbone: Fishbone = vec![];
    'outer: for num in quality_str.split(",").map(|x| x.parse().unwrap()) {
        for e in fishbone.iter_mut() {
            if num < e.0 && e.1.is_none() {
                e.1 = Some(num);
                continue 'outer;
            }
            if num > e.0 && e.2.is_none() {
                e.2 = Some(num);
                continue 'outer;
            }
        }
        fishbone.push((num, None, None));
    }
    fishbone
}

fn compute_quality(fishbone: &Fishbone) -> i64 {
    fishbone
        .iter()
        .map(|(c, _, _)| c.to_string())
        .join("")
        .parse()
        .unwrap()
}

pub fn solve_part_1(input: &str) -> String {
    let (_, data) = input.split_once(":").unwrap();
    compute_quality(&parse_fishbone(data)).to_string()
}

pub fn solve_part_2(input: &str) -> String {
    let mut worst_quality = i64::MAX;
    let mut best_quality = i64::MIN;
    for sword in input.lines() {
        let (_, data) = sword.split_once(":").unwrap();
        let quality = compute_quality(&parse_fishbone(data));
        worst_quality = worst_quality.min(quality);
        best_quality = best_quality.max(quality);
    }
    (best_quality - worst_quality).to_string()
}

pub fn solve_part_3(input: &str) -> String {
    let mut swords: Vec<_> = input
        .lines()
        .map(|def| {
            let (id, data) = def.split_once(":").unwrap();
            let fishbone = parse_fishbone(data);
            (id.parse::<i64>().unwrap(), fishbone)
        })
        .collect();
    swords.sort_by(|a, b| {
        let cmp = compute_quality(&a.1).cmp(&compute_quality(&b.1));
        if !matches!(cmp, std::cmp::Ordering::Equal) {
            return cmp;
        }
        for (a_seg, b_seg) in a.1.iter().zip(b.1.iter()) {
            let a_val = match a_seg {
                (a, Some(b), Some(c)) => format!("{b}{a}{c}"),
                (a, Some(b), None) => format!("{b}{a}"),
                (a, None, Some(c)) => format!("{a}{c}"),
                (a, None, None) => format!("{a}"),
            };
            let b_val = match b_seg {
                (a, Some(b), Some(c)) => format!("{b}{a}{c}"),
                (a, Some(b), None) => format!("{b}{a}"),
                (a, None, Some(c)) => format!("{a}{c}"),
                (a, None, None) => format!("{a}"),
            };
            let cmp = a_val.parse::<i64>().unwrap().cmp(&b_val.parse().unwrap());
            if !matches!(cmp, std::cmp::Ordering::Equal) {
                return cmp;
            }
        }
        a.0.cmp(&b.0)
    });
    swords.reverse();
    swords
        .into_iter()
        .enumerate()
        .map(|(pos, (id, _))| id * (pos as i64 + 1))
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!("581078", solve_part_1("58:5,3,7,8,9,10,4,5,7,8,8"));
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(
            "77053",
            solve_part_2(
                "1:2,4,1,1,8,2,7,9,8,6
2:7,9,9,3,8,3,8,8,6,8
3:4,7,6,9,1,8,3,7,2,2
4:6,4,2,1,7,4,5,5,5,8
5:2,9,3,8,3,9,5,2,1,4
6:2,4,9,6,7,4,1,7,6,8
7:2,3,7,6,2,2,4,1,4,2
8:5,1,5,6,8,3,1,8,3,9
9:5,7,7,3,7,2,3,8,6,7
10:4,1,9,3,8,5,4,3,5,5"
            )
        );
    }

    #[test]
    fn test_solve_part_3() {
        assert_eq!(
            "260",
            solve_part_3(
                "1:7,1,9,1,6,9,8,3,7,2
2:6,1,9,2,9,8,8,4,3,1
3:7,1,9,1,6,9,8,3,8,3
4:6,1,9,2,8,8,8,4,3,1
5:7,1,9,1,6,9,8,3,7,3
6:6,1,9,2,8,8,8,4,3,5
7:3,7,2,2,7,4,4,6,3,1
8:3,7,2,2,7,4,4,6,3,7
9:3,7,2,2,7,4,1,6,3,7"
            )
        );
    }
}
