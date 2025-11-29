use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    f64::consts::PI,
};

use itertools::Itertools;
use libm::atan2;
use priority_queue::PriorityQueue;

fn l2(i: usize, j: usize) -> usize {
    i * i + j * j
}

fn erupt(data: &[Vec<char>], vi: usize, vj: usize, r: usize) -> usize {
    let r2 = r * r;
    data.iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.iter()
                .enumerate()
                .filter(move |&(j, v)| *v != '@' && l2(vi.abs_diff(i), vj.abs_diff(j)) <= r2)
        })
        .map(|(_, v)| (*v as u8 - b'0') as usize)
        .sum::<usize>()
}

fn find(data: &[Vec<char>], c: char) -> (usize, usize) {
    data.iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.iter()
                .enumerate()
                .filter(|&(_, v)| *v == c)
                .map(move |(j, _)| (i, j))
        })
        .exactly_one()
        .unwrap()
}

pub fn solve_part_1(input: &str) -> String {
    let data = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (vi, vj) = find(&data, '@');
    erupt(&data, vi, vj, 10).to_string()
}

pub fn solve_part_2(input: &str) -> String {
    let data = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (vi, vj) = find(&data, '@');
    let h = data.len();
    let w = data[0].len();
    let max_r = vi.max(vj).max(h - vi - 1).max(w - vj - 1);
    let mut last_eruption = 0;
    let mut max_eruption = 0;
    let mut max_eruption_r = 0;
    for r in 1..=max_r {
        let eruption = erupt(&data, vi, vj, r);
        let de = eruption - last_eruption;
        if de > max_eruption {
            max_eruption = de;
            max_eruption_r = r;
        }
        last_eruption = eruption;
    }
    (max_eruption_r * max_eruption).to_string()
}

pub fn solve_part_3(input: &str) -> String {
    let data = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let width = data[0].len();
    let height = data.len();
    let (vi, vj) = find(&data, '@');
    let (si, sj) = find(&data, 'S');

    let azimuth = |i: usize, j: usize| atan2(i as f64 - vi as f64, j as f64 - vj as f64);

    let small_rot = |az1: f64, az2: f64| {
        let d = az1 - az2;
        if d > PI {
            d - 2. * PI
        } else if d < -PI {
            d + 2. * PI
        } else {
            d
        }
    };

    let solve = |radius: usize| {
        let r2 = radius * radius;
        let time_limit = ((radius + 1) * 30) as i64;
        let mut queue = PriorityQueue::new();
        let mut rotations = HashMap::new();
        rotations.insert((si, sj, false), 0f64);
        let mut visited = HashSet::new();
        queue.push((si, sj, false), Reverse(0));
        while let Some(((i, j, rotated), Reverse(time))) = queue.pop() {
            if time >= time_limit {
                break;
            }
            visited.insert((i, j, rotated));
            let az = azimuth(i, j);
            let rotation = rotations[&(i, j, rotated)];
            for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (ni, nj) = (i.wrapping_add_signed(di), j.wrapping_add_signed(dj));
                if ni >= height || nj >= width {
                    continue;
                }
                if l2(ni.abs_diff(vi), nj.abs_diff(vj)) <= r2 {
                    continue;
                }
                let is_rotated = if let Some(previous_rotation) = rotations.get(&(ni, nj, false)) {
                    let rotation = rotation + small_rot(azimuth(ni, nj), az);
                    (rotation - previous_rotation).abs() > 6.
                } else {
                    false
                };
                if (ni, nj, is_rotated) == (si, sj, true) {
                    return Some(time);
                }
                if visited.contains(&(ni, nj, is_rotated)) {
                    continue;
                }
                let new_time: i64 = time + (data[ni][nj] as i8 - '0' as i8) as i64;
                let should_update =
                    match queue.push_increase((ni, nj, is_rotated), Reverse(new_time)) {
                        None => true,
                        Some(Reverse(t)) => t > new_time,
                    };
                if should_update {
                    rotations.insert(
                        (ni, nj, is_rotated),
                        rotation + small_rot(azimuth(ni, nj), az),
                    );
                };
            }
        }
        None
    };
    let (radius, time) = (1..(width.min(height) / 2))
        .map(|radius| (radius, solve(radius)))
        .filter(|(_, s)| s.is_some())
        .min()
        .unwrap();
    (radius as i64 * time.unwrap()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!(
            "1573",
            solve_part_1(
                "189482189843433862719
279415473483436249988
432746714658787816631
428219317375373724944
938163982835287292238
627369424372196193484
539825864246487765271
517475755641128575965
685934212385479112825
815992793826881115341
1737798467@7983146242
867597735651751839244
868364647534879928345
519348954366296559425
134425275832833829382
764324337429656245499
654662236199275446914
317179356373398118618
542673939694417586329
987342622289291613318
971977649141188759131"
            )
        );
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(
            "1090",
            solve_part_2(
                "4547488458944
9786999467759
6969499575989
7775645848998
6659696497857
5569777444746
968586@767979
6476956899989
5659745697598
6874989897744
6479994574886
6694118785585
9568991647449"
            )
        );
    }

    #[test]
    fn test_solve_part_3() {
        assert_eq!(
            "592",
            solve_part_3(
                "2645233S5466644
634566343252465
353336645243246
233343552544555
225243326235365
536334634462246
666344656233244
6426432@2366453
364346442652235
253652463426433
426666225623563
555462553462364
346225464436334
643362324542432
463332353552464"
            )
        );
    }

    #[test]
    fn test_solve_part_3_2() {
        assert_eq!(
            "330",
            solve_part_3(
                "545233443422255434324
5222533434S2322342222
523444354223232542432
553522225435232255242
232343243532432452524
245245322252324442542
252533232225244224355
523533554454232553332
522332223232242523223
524523432425432244432
3532242243@4323422334
542524223994422443222
252343244322522222332
253355425454255523242
344324325233443552555
423523225325255345522
244333345244325322335
242244352245522323422
443332352222535334325
323532222353523253542
553545434425235223552"
            )
        );
    }
}
