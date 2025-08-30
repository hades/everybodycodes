use std::collections::{HashMap, HashSet};

fn solve_maze(
    platform_initial_offsets: &HashMap<(usize, usize), u8>,
    start_positions: &[(usize, usize)],
    end_position: (usize, usize),
) -> usize {
    let mut state_front = HashSet::new();
    for (i, j) in start_positions {
        state_front.insert((*i, *j, 0));
    }
    let mut t = 0;
    let mut visited = HashSet::new();
    loop {
        let mut new_state_front = HashSet::new();
        for (i, j, z) in state_front {
            if (i, j) == end_position {
                return t;
            }
            visited.insert((i, j));
            for (di, dj) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                if i == 0 && di < 0 {
                    continue;
                }
                if j == 0 && dj < 0 {
                    continue;
                }
                let ni = (i as isize + di) as usize;
                let nj = (j as isize + dj) as usize;
                if visited.contains(&(ni, nj)) {
                    continue;
                }
                if let Some(platform_initial_z) = platform_initial_offsets.get(&(ni, nj)) {
                    if *platform_initial_z == z {
                        new_state_front.insert((ni, nj, z));
                    }
                }
            }
            new_state_front.insert((i, j, (z + 1) % 10));
            new_state_front.insert((i, j, (z + 9) % 10));
        }
        state_front = new_state_front;
        t += 1;
    }
}

pub fn solve_part_1(input: &str) -> String {
    let mut platform_initial_offsets: HashMap<(usize, usize), u8> = HashMap::new();
    let mut start_position = None;
    let mut end_position = None;
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            match ch {
                'S' => {
                    start_position = Some((i, j));
                    platform_initial_offsets.insert((i, j), 0);
                }
                'E' => {
                    end_position = Some((i, j));
                    platform_initial_offsets.insert((i, j), 0);
                }
                '#' => {}
                ' ' => {}
                ch => {
                    platform_initial_offsets.insert((i, j), ch as u8 - '0' as u8);
                }
            }
        }
    }
    let start_position = start_position.unwrap();
    let end_position = end_position.unwrap();
    solve_maze(&platform_initial_offsets, &[start_position], end_position).to_string()
}

pub fn solve_part_2(input: &str) -> String {
    solve_part_1(input)
}

pub fn solve_part_3(input: &str) -> String {
    let mut platform_initial_offsets: HashMap<(usize, usize), u8> = HashMap::new();
    let mut start_positions = vec![];
    let mut end_position = None;
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            match ch {
                'S' => {
                    start_positions.push((i, j));
                }
                'E' => {
                    end_position = Some((i, j));
                    platform_initial_offsets.insert((i, j), 0);
                }
                '#' => {}
                ' ' => {}
                ch => {
                    platform_initial_offsets.insert((i, j), ch as u8 - '0' as u8);
                }
            }
        }
    }
    let end_position = end_position.unwrap();
    solve_maze(
        &platform_initial_offsets,
        start_positions.as_slice(),
        end_position,
    )
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_part_1() {
        assert_eq!(
            "28",
            solve_part_1(
                "#######
#6769##
S50505E
#97434#
#######"
            )
        );
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            "14",
            solve_part_3(
                "SSSSSSSSSSS
S674345621S
S###6#4#18S
S53#6#4532S
S5450E0485S
S##7154532S
S2##314#18S
S971595#34S
SSSSSSSSSSS"
            )
        );
    }
}
