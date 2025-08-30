//      ....................
//      ......↗→→→→→........
//      .....↗......↘.......
//      ....↗........↘......
//      ...↗..........↘.....
//      .C↗............↘....
//      .B..............↘...
//      .A...............↘..
//      ====================
//    x=012345678901234567890
//                1         2
//  A: start at (1, 0), travel diagonally to (1 + p, p),
//                      then horizontally to (1 + 2p, p),
//                      then diagonally to (1 + 2p + i, p - i)
//                   (if landing on the ground) i = p
//                                              x = 1 + 3p
//  B: start at (1, 1), travel diagonally to (1 + p, 1 + p),
//                      then horizontally to (1 + 2p, 1 + p),
//                      then diagonally to (1 + 2p + i, 1 + p - i)
//                   (if landing on the ground) i = p + 1
//                                              x = 2 + 3p
//  C: start at (1, 2), travel diagonally to (1 + p, 2 + p),
//                      then horizontally to (1 + 2p, 2 + p),
//                      then diagonally to (1 + 2p + i, 2 + p - i)
//                   (if landing on the ground) i = 2 + p
//                                              x = 3 + 3p
//
// For target at (tx, ty):
//   either p - i = ty          (A)
//       or p - i = ty - 1      (B)
//       or p - i = ty - 2      (C)
//      and 1 + 2p + i = tx
// =>    either     3p = tx + ty - 3  (C)
//           or     3p = tx + ty - 2  (B)
//           or     3p = tx + ty - 1  (A)
//
// (tx + ty) mod 3: 0 => p = (tx + ty - 3) / 3  (C)
//                  1 => p = (tx + ty - 1) / 3  (A)
//                  2 => p = (tx + ty - 2) / 3  (B)
//
// ---- METEORS ------
//
// launch time: l
// meteor trajectory: (mx - t, my - t)
// interception points:
//   A (upwards trajectory): mx - t = 1 + t - l, my - t = t - l
//   B (upwards trajectory): mx - t = 1 + t - l, my - t = 1 + t - l
//   C (upwards trajectory): mx - t = 1 + t - l, my - t = 2 + t - l
//
//     (only works if mx - my = {1,0,-1}, i.e. on the 45° line from the catapult)
//
//     2t = mx + l - 1
//     my = mx + l - 1 - l + C = mx + C - 1
//     C = my - mx + 1
//
//   A (coasting):           mx - t = 1 + t - l, my - t = p
//   B (coasting):           mx - t = 1 + t - l, my - t = p + 1
//   C (coasting):           mx - t = 1 + t - l, my - t = p + 2
//
//    mx = 1 + 2t - l
//    my = p + t + C
//
//    if mx + l is odd:
//      t = (mx - 1 + l) / 2
//      p = my - t - C = my - (mx - 1 + l) / 2 - C
//
//   A (downwards):          mx - t = 1 + t - l, my - t = p - (t - 2*p - l)
//   B (downwards):          mx - t = 1 + t - l, my - t = p - (t - 2*p - l) + 1
//   C (downwards):          mx - t = 1 + t - l, my - t = p - (t - 2*p - l) + 2
//
//    mx = 1 + 2t - l
//    my = 3*p + l + C
//
//    if mx + l is odd:
//      t = (mx - 1 + l) / 2
//      p = (my - l - C) / 3

pub fn solve_part_1(input: &str) -> String {
    let mut targets = Vec::new();
    for (y, line) in input.lines().rev().skip(1).enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == 'T' {
                targets.push((x, y));
            } else if ch == 'H' {
                targets.push((x, y));
                targets.push((x, y));
            }
        }
    }
    targets
        .into_iter()
        .map(|(tx, ty)| {
            let (seg_points, p) = match (tx + ty) % 3 {
                0 => (3, (tx + ty - 3) / 3),
                1 => (1, (tx + ty - 1) / 3),
                2 => (2, (tx + ty - 2) / 3),
                _ => unreachable!(),
            };
            seg_points * p
        })
        .sum::<usize>()
        .to_string()
}

pub fn solve_part_2(input: &str) -> String {
    solve_part_1(input)
}

pub fn solve_part_3(input: &str) -> String {
    let meteors: Vec<(i32, i32)> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_at(line.find(' ').unwrap());
            (x.parse().unwrap(), y[1..].parse().unwrap())
        })
        .collect();
    let mut total_value = 0;
    for (mx, my) in meteors.into_iter() {
        let mx = mx + 1;
        let mut interception_y = None;
        let mut value = None;
        if mx - my <= 1 && mx - my >= -1 {
            let p = (mx - (mx % 2)) / 2;
            let cy = my - mx + 1;
            interception_y = Some(my - p);
            value = Some((cy + 1) * p);
        }
        let l = 1 - (mx % 2);
        {
            let t = (mx - 1 + l) / 2;
            let iy = my - t;
            if interception_y.is_none() || interception_y.unwrap() <= iy {
                interception_y = Some(iy);
                for cy in 0..2 {
                    let p = my - t - cy;
                    if !(t > p && t <= 2 * p) {
                        continue;
                    }
                    let v = (cy + 1) * p;
                    if value.is_none() || value.unwrap() > v {
                        value = Some(v);
                    }
                }
            }
        }
        {
            let t = (mx - 1 + l) / 2;
            let cy = (my - l) % 3;
            let p = (my - l - cy) / 3;
            let iy = my - t;
            if t > 2 * p && (interception_y.is_none() || interception_y.unwrap() <= iy) {
                let v = (cy + 1) * p;
                if value.is_none() || value.unwrap() > v {
                    value = Some(v);
                }
            }
        }
        total_value += value.unwrap();
    }
    total_value.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
    #[test]
    fn test_part_1() {
        assert_eq!(
            "13",
            solve_part_1(
                ".............
.C...........
.B......T....
.A......T.T..
============="
            )
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            "22",
            solve_part_2(
                ".............
.C...........
.B......H....
.A......T.H..
============="
            )
        );
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            "11",
            solve_part_3(
                "6 5
6 7
10 5"
            )
        );
    }
}
