use std::collections::{BTreeSet, HashMap};

use array2d::Array2D;
use priority_queue::PriorityQueue;

type Point = (i64, i64);

pub fn solve_part_1(input: &str) -> String {
    let mut vertical_walls = Vec::<(Point, Point)>::new();
    let mut horizontal_walls = Vec::<(Point, Point)>::new();
    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut dir = 0;
    let (mut x, mut y) = (0, 0);
    let mut interesting_points_x = BTreeSet::new();
    let mut interesting_points_y = BTreeSet::new();
    for instr in input.split(",") {
        let dist = if let Some(dist) = instr.strip_prefix("L") {
            dir = (dir + 3) % 4;
            dist
        } else if let Some(dist) = instr.strip_prefix("R") {
            dir = (dir + 1) % 4;
            dist
        } else {
            panic!("unparsable instruction {instr}");
        };
        let dist = dist.parse::<i64>().unwrap() - 1;
        let (endx, endy) = (x + dirs[dir].0 * dist, y + dirs[dir].1 * dist);
        let insert_to_walls = match dirs[dir] {
            (_, 0) => &mut horizontal_walls,
            (0, _) => &mut vertical_walls,
            _ => panic!("unexpected direction"),
        };
        let wall = ((x.min(endx), y.min(endy)), (x.max(endx), y.max(endy)));
        interesting_points_x.insert(wall.0.0 - 1);
        interesting_points_x.insert(wall.1.0 + 1);
        interesting_points_y.insert(wall.0.1 - 1);
        interesting_points_y.insert(wall.1.1 + 1);
        insert_to_walls.push(wall);
        x = endx + dirs[dir].0;
        y = endy + dirs[dir].1;
    }
    let (endx, endy) = (x, y);
    interesting_points_x.insert(endx);
    interesting_points_y.insert(endy);
    interesting_points_x.insert(0);
    interesting_points_y.insert(0);
    let x_lower_bound = *interesting_points_x.iter().min().unwrap() - 1;
    let x_upper_bound = *interesting_points_x.iter().max().unwrap() + 1;
    let y_lower_bound = *interesting_points_y.iter().min().unwrap() - 1;
    let y_upper_bound = *interesting_points_y.iter().max().unwrap() + 1;
    interesting_points_x.insert(x_lower_bound);
    interesting_points_x.insert(x_upper_bound);
    interesting_points_y.insert(y_lower_bound);
    interesting_points_y.insert(y_upper_bound);
    let mut interesting_points = Array2D::filled_with(
        (0, 0),
        interesting_points_x.len(),
        interesting_points_y.len(),
    );
    let mut interesting_points_reverse = HashMap::new();
    for (i, x) in interesting_points_x.iter().enumerate() {
        for (j, y) in interesting_points_y.iter().enumerate() {
            interesting_points[(i, j)] = (*x, *y);
            interesting_points_reverse.insert((*x, *y), (i, j));
        }
    }
    let mut gscore = HashMap::<Point, i64>::new();
    let mut queue = PriorityQueue::new();
    queue.push((0, 0), 0);
    gscore.insert((0, 0), 0);
    while let Some(((x, y), _)) = queue.pop() {
        if (x, y) == (endx, endy) {
            return gscore[&(x, y)].to_string();
        }
        let current_gscore = gscore[&(x, y)];
        let (i, j) = interesting_points_reverse.get(&(x, y)).unwrap();
        for (ni, nj) in [
            (i + 1, *j),
            (i.wrapping_sub(1), *j),
            (*i, j + 1),
            (*i, j.wrapping_sub(1)),
        ] {
            if ni >= interesting_points.num_rows() || nj >= interesting_points.num_columns() {
                continue;
            }
            let (nx, ny) = interesting_points[(ni, nj)];
            let mut intersects_with_a_wall = false;
            if nj == *j {
                let (leftx, rightx) = if nx > x { (x + 1, nx) } else { (nx, x - 1) };
                intersects_with_a_wall |= horizontal_walls.iter().any(|(from, to)| {
                    assert_eq!(from.1, to.1);
                    from.1 == ny
                        && (leftx <= from.0 && from.0 <= rightx || leftx <= to.0 && to.0 <= rightx)
                });
                intersects_with_a_wall |= vertical_walls.iter().any(|(from, to)| {
                    assert_eq!(from.0, to.0);
                    (leftx <= from.0 && from.0 <= rightx) && (from.1 <= ny && ny <= to.1)
                });
            } else {
                let (lefty, righty) = if ny > y { (y + 1, ny) } else { (ny, y - 1) };
                intersects_with_a_wall |= vertical_walls.iter().any(|(from, to)| {
                    assert_eq!(from.0, to.0);
                    from.0 == nx
                        && (lefty <= from.1 && from.1 <= righty || lefty <= to.1 && to.1 <= righty)
                });
                intersects_with_a_wall |= horizontal_walls.iter().any(|(from, to)| {
                    assert_eq!(from.1, to.1);
                    (lefty <= from.1 && from.1 <= righty) && (from.0 <= nx && nx <= to.0)
                });
            }
            if intersects_with_a_wall {
                continue;
            }
            let new_dist = current_gscore
                .wrapping_add_unsigned(nx.abs_diff(x))
                .wrapping_add_unsigned(ny.abs_diff(y));
            if new_dist < *gscore.get(&(nx, ny)).unwrap_or(&i64::MAX) {
                gscore.insert((nx, ny), new_dist);
                queue.push(
                    (nx, ny),
                    (-new_dist)
                        .wrapping_sub_unsigned(nx.abs_diff(endx))
                        .wrapping_sub_unsigned(ny.abs_diff(ny)),
                );
            }
        }
    }
    panic!("exit not found");
}

pub fn solve_part_2(input: &str) -> String {
    solve_part_1(input)
}

pub fn solve_part_3(input: &str) -> String {
    solve_part_1(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!(
            "16",
            solve_part_1("L6,L3,L6,R3,L6,L3,L3,R6,L6,R6,L6,L6,R3,L3,L3,R3,R3,L6,L6,L3")
        );
    }
}
