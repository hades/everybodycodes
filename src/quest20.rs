use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};

use array2d::Array2D;
use log::debug;

pub fn solve_part_1(input: &str) -> String {
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();
    let mut grid = Array2D::<char>::filled_with(' ', width, height);
    let mut start_pos = (0 ,0);
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid[(x, y)] = ch;
            if ch == 'S' {
                start_pos = (x, y);
            }
        }
    }
    const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut states: Vec<_> = (0..4).map(|dir| (start_pos.0, start_pos.1, 1000, dir)).collect();
    let mut visited_states = HashSet::<_>::new();
    for _ in 0..100 {
        let mut next_states = Vec::new();
        for (x, y, z, dir) in states {
            if visited_states.contains(&(x, y, z, dir)) { continue; }
            visited_states.insert((x, y, z, dir));
            for turn in -1..=1 {
                let new_dir = (((dir as isize) + turn + 4) % 4) as usize;
                let (dx, dy) = DIRECTIONS[new_dir];
                if let (Some(nx), Some(ny)) = (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                    if nx >= width || ny >= height || grid[(nx, ny)] == '#' { continue; }
                    let new_z = match grid[(nx, ny)] {
                        '+' => z + 1,
                        '.' => z - 1,
                        'S' => z - 1,
                        '-' => z - 2,
                        _ => unreachable!("{nx} {ny} {}", grid[(nx, ny)]),
                    };
                    next_states.push((nx, ny, new_z, new_dir));
                }
            }
        }
        states = next_states;
    }
    states.into_iter().map(|(_, _, z, _)| z).max().unwrap().to_string()
}

pub fn solve_part_2(input: &str) -> String {
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();
    let mut grid = Array2D::<char>::filled_with(' ', width, height);
    let mut start_pos = (0 ,0);
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid[(x, y)] = ch;
            if ch == 'S' {
                start_pos = (x, y);
            }
        }
    }
    const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut states: Vec<_> = (0..4).map(|dir| (start_pos.0, start_pos.1, 10000, dir, 0)).collect();
    let mut visited_states = HashSet::<_>::new();
    let mut t = 0;
    loop {
        let mut next_states = Vec::new();
        for (x, y, z, dir, checkpoints) in states {
            if visited_states.contains(&(x, y, z, dir, checkpoints)) { continue; }
            visited_states.insert((x, y, z, dir, checkpoints));
            if x == start_pos.0 && y == start_pos.1 && z >= 10000 && checkpoints >= 3 {
                return t.to_string();
            }
            for turn in -1..=1 {
                let new_dir = (((dir as isize) + turn + 4) % 4) as usize;
                let (dx, dy) = DIRECTIONS[new_dir];
                if let (Some(nx), Some(ny)) = (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                    if nx >= width || ny >= height || grid[(nx, ny)] == '#' { continue; }
                    let new_checkpoints = match grid[(nx, ny)] {
                        'A' => max(checkpoints, 1),
                        'B' => max(checkpoints, 2),
                        'C' => max(checkpoints, 3),
                        _ => checkpoints,
                    };
                    if new_checkpoints - checkpoints > 1 { continue; }
                    let new_z = match grid[(nx, ny)] {
                        '+' => z + 1,
                        '.'|'S'|'A'|'B'|'C' => z - 1,
                        '-' => z - 2,
                        _ => unreachable!("{nx} {ny} {}", grid[(nx, ny)]),
                    };
                    next_states.push((nx, ny, new_z, new_dir, new_checkpoints));
                }
            }
        }
        states = next_states;
        t += 1;
    }
}
    
fn find_best_route_to_south(start_x: usize, start_y: usize, start_z: isize, start_dir: usize, grid: &Array2D<char>) -> Vec<(usize, usize, usize, usize)> {
    let height = grid.num_columns();
    let width = grid.num_rows();
    const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut states = VecDeque::new();
    let mut visited_positions = HashMap::new();
    let mut max_y = start_y;
    let mut max_y_at_ground = 0;
    let mut max_y_at_ground_at_position = None;
    let mut pred = HashMap::new();
    states.push_back((start_x, start_y, start_z, start_dir));
    while let Some((x, y_global, z, dir)) = states.pop_front() {
        if let Some(previous_z) = visited_positions.get(&(x, y_global, dir)) {
            if *previous_z >= z { continue; }
        }
        visited_positions.insert((x, y_global, dir), z);
        max_y = max(y_global, max_y);
        if z == 0 {
            if y_global > max_y_at_ground {
                max_y_at_ground = y_global;
                max_y_at_ground_at_position = Some((x, y_global, dir, 0));
            }
            continue;
        }
        for turn in -1..=1 {
            let new_dir = (((dir as isize) + turn + 4) % 4) as usize;
            let (dx, dy) = DIRECTIONS[new_dir];
            if let (Some(nx), Some(ny_global)) = (x.checked_add_signed(dx), y_global.checked_add_signed(dy)) {
                let ny_local = ny_global % height;
                if nx >= width || grid[(nx, ny_local)] == '#' { continue; }
                let new_z = match grid[(nx, ny_local)] {
                    '+' => z + 1,
                    '.' => z - 1,
                    'S' => z - 1,
                    '-' => z - 2,
                    _ => unreachable!("{nx} {ny_local} {}", grid[(nx, ny_local)]),
                };
                if new_z < 0 { continue; }
                states.push_back((nx, ny_global, new_z, new_dir));
                let pred_value = pred.entry((nx, ny_global, new_dir)).or_insert(((x, y_global, dir), z));
                if z > pred_value.1 {
                    *pred_value = ((x, y_global, dir), z);
                }
            }
        }
    }
    let mut route = vec![max_y_at_ground_at_position.unwrap()];
    while let Some((x, y, dir, _)) = route.last() {
        if *x == start_x && *y == start_y && *dir == start_dir {
            break;
        }
        let ((pred_x, pred_y, pred_dir), pred_z) = pred[&(*x, *y, *dir)];
        route.push((pred_x, pred_y, pred_dir, pred_z as usize));
    }
    route.reverse();
    route
}

pub fn solve_part_3(input: &str) -> String {
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();
    let mut grid = Array2D::<char>::filled_with(' ', width, height);
    let mut start_pos = (0 ,0);
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid[(x, y)] = ch;
            if ch == 'S' {
                start_pos = (x, y);
            }
        }
    }
    /*
    let mut block_reachability = HashMap::<((usize, isize, usize), (usize, isize, usize)), isize>::new();
    for start_x in 0..width {
        for start_y in 0..height {
            if grid[(start_x, start_y)] == '#' { continue; }
            for start_dir in 0..4 {
                let mut z = 0;
                let mut states = vec![(start_x, start_y as isize, z, start_dir)];
                while !states.is_empty() {
                    let mut new_states = Vec::new();
                    for (x, y, z, dir) in states {
                        if let Some(previous_reached_z_at_state) = block_reachability.get(&((start_x, start_y as isize, start_dir), (x, y, dir))) {
                            if *previous_reached_z_at_state >= z {
                                continue;
                            }
                        }
                        if y > (height as isize) { continue; }
                        if y < -1 { continue; }
                        for turn in -1..=1 {
                            let new_dir = (((dir as isize) + turn + 4) % 4) as usize;
                            let (dx, dy) = DIRECTIONS[new_dir];
                            let ny = y + dy;
                            if let Some(nx) = x.checked_add_signed(dx) {
                                let ny_on_grid = (ny + height as isize) as usize % height;
                                if nx >= width || grid[(nx, ny_on_grid)] == '#' { continue; }
                                let new_z = match grid[(nx, ny_on_grid)] {
                                    '+' => z + 1,
                                    '.' => z - 1,
                                    '-' => z - 2,
                                    _ => unreachable!("{nx} {ny} {}", grid[(nx, ny_on_grid)]),
                                };
                                new_states.push((nx, ny, new_z, new_dir));
                            }
                        }
                    }
                    states = new_states;
                }
            }
        }
    }
    let mut states: Vec<_> = (0..4).map(|dir| (start_pos.0, start_pos.1, 10000, dir)).collect();
    while !states.is_empty() {
        let mut next_states = HashMap::<(usize, usize), isize>::new();
        let mut max_y_in_this_block = 0;
        let mut y_in_this_block = 0;
        for (x, y, z, dir) in states {
            y_in_this_block = y;
            for entry in block_reachability.iter() {
                if entry.0.0 == (x, 0, dir) && *entry.1 == 0 {
                    max_y_in_this_block = max(max_y_in_this_block, y.wrapping_add_signed(entry.0.1.1));
                }
            }
            for possible_x_in_next_block in 0..width {
                if grid[(possible_x_in_next_block, 0)] == '#' { continue; }
                for possible_dir_in_next_block in 0..4 {
                    let new_z = z + block_reachability[&((x, (y % height) as isize, dir), (possible_x_in_next_block, height as isize, possible_dir_in_next_block))];
                    if new_z < 0 { continue; }
                    if let Some(already_reached_z) = next_states.get(&(possible_x_in_next_block, possible_dir_in_next_block)) {
                        if *already_reached_z >= new_z {
                            continue;
                        }
                    }
                    next_states.insert((possible_x_in_next_block, possible_dir_in_next_block), new_z);
                }
            }
        }
        if next_states.is_empty() {
            return max_y_in_this_block.to_string();
        }
        states = next_states.into_iter().map(|((x, dir), z)| (x, y_in_this_block + height, z, dir)).collect();
    }
    unreachable!()
    */
    const START_Z: usize = 384400;
    // Find a route that has a height-aligned cycle. Since we don't know at which altitude this will happen, search for
    // the altitude exponentially.
    let mut start_z = 10;
    let possible_routes_with_cycles;
    loop {
        debug!("trying to find route from altitude {start_z}");
        let routes: Vec<_> = (0..4)
            .map(|start_dir| find_best_route_to_south(start_pos.0, start_pos.1, start_z, start_dir, &grid))
            .collect();
        let mut result = Vec::new();
        for route in routes {
            let mut visited_states: HashMap<_, Vec<_>> = HashMap::new();
            for (i, &(x, y, dir, _)) in route.iter().enumerate() {
                if y % height != 0 { continue; }
                if let Some(previous_visits_vec) = visited_states.get_mut(&(x, dir)) {
                    previous_visits_vec.push(i);
                    if previous_visits_vec.len() >= 3 {
                        result.push((route, previous_visits_vec[0], previous_visits_vec[1]));
                        break;
                    }
                } else {
                    visited_states.insert((x, dir), vec![i]);
                }
            }
        }
        if result.len() == 4 {
            possible_routes_with_cycles = result;
            break;
        }
        start_z *= 2;
    }
    let mut best_y = 0;
    for (route, cycle_start_idx, cycle_end_idx) in possible_routes_with_cycles {
        debug!("{:?}", route);
        debug!("{cycle_start_idx} {cycle_end_idx}");
        let (x_end, y_end, dir_end, z_end) = route[cycle_end_idx];
        let prefix_dz = start_z as usize - route[cycle_start_idx].3;
        let cycle_dz = route[cycle_start_idx].3 - z_end;
        let cycle_dy = y_end - route[cycle_start_idx].1;
        let cycle_count = (START_Z - prefix_dz) / cycle_dz;
        let y_suffix = route[cycle_start_idx].1 + cycle_count * cycle_dy;
        let z_suffix = START_Z - prefix_dz - cycle_count * cycle_dz;
        let remaining_route = find_best_route_to_south(x_end, y_suffix, z_suffix as isize, dir_end, &grid);
        best_y = max(best_y, remaining_route.last().unwrap().1);
    }
    (best_y).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_part_1() {
        assert_eq!("1045", solve_part_1("#....S....#
#.........#
#---------#
#.........#
#..+.+.+..#
#.+-.+.++.#
#.........#"));
    }

    #[test]
    fn test_part_2() {
        assert_eq!("24", solve_part_2("####S####
#-.+++.-#
#.+.+.+.#
#-.+.+.-#
#A+.-.+C#
#.+-.-+.#
#.+.B.+.#
#########"));

        assert_eq!("78", solve_part_2("###############S###############
#+#..-.+.-++.-.+.--+.#+.#++..+#
#-+-.+-..--..-+++.+-+.#+.-+.+.#
#---.--+.--..++++++..+.-.#.-..#
#+-+.#+-.#-..+#.--.--.....-..##
#..+..-+-.-+.++..-+..+#-.--..-#
#.--.A.-#-+-.-++++....+..C-...#
#++...-..+-.+-..+#--..-.-+..-.#
#..-#-#---..+....#+#-.-.-.-+.-#
#.-+.#+++.-...+.+-.-..+-++..-.#
##-+.+--.#.++--...-+.+-#-+---.#
#.-.#+...#----...+-.++-+-.+#..#
#.---#--++#.++.+-+.#.--..-.+#+#
#+.+.+.+.#.---#+..+-..#-...---#
#-#.-+##+-#.--#-.-......-#..-##
#...+.-+..##+..+B.+.#-+-++..--#
###############################"));
        assert_eq!("206", solve_part_2("###############S###############
#-----------------------------#
#-------------+++-------------#
#-------------+++-------------#
#-------------+++-------------#
#-----------------------------#
#-----------------------------#
#-----------------------------#
#--A-----------------------C--#
#-----------------------------#
#-----------------------------#
#-----------------------------#
#-----------------------------#
#-----------------------------#
#-----------------------------#
#--------------B--------------#
#-----------------------------#
#-----------------------------#
###############################"));
    }
}
