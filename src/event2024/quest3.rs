use array2d::Array2D;

pub fn solve_part_1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let width = lines.len();
    let height = lines[0].len();
    let mut depths = Array2D::filled_with(0, width, height);
    let mut amount_dug_out = 0;
    for i in 0..width {
        for j in 0..height {
            if lines[i][j..j + 1] == *"#" {
                depths[(i, j)] = 1;
                amount_dug_out += 1;
            }
        }
    }
    let mut level = 2;
    loop {
        let mut additional_amount_dug_out = 0;
        for i in 0..width {
            for j in 0..height {
                if depths[(i, j)] != level - 1 {
                    continue;
                }
                if level - depths[(i - 1, j)] > 1
                    || level - depths[(i + 1, j)] > 1
                    || level - depths[(i, j - 1)] > 1
                    || level - depths[(i, j + 1)] > 1
                {
                    continue;
                }
                additional_amount_dug_out += 1;
                depths[(i, j)] = level;
            }
        }
        if additional_amount_dug_out == 0 {
            break;
        }
        amount_dug_out += additional_amount_dug_out;
        level += 1;
    }
    amount_dug_out.to_string()
}

pub fn solve_part_2(input: &str) -> String {
    solve_part_1(input)
}

pub fn solve_part_3(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let width = lines.len();
    let height = lines[0].len();
    let mut depths = Array2D::filled_with(0, width, height);
    let mut amount_dug_out = 0;
    for i in 0..width {
        for j in 0..height {
            if lines[i][j..j + 1] == *"#" {
                depths[(i, j)] = 1;
                amount_dug_out += 1;
            }
        }
    }
    let mut level = 2;
    loop {
        let mut additional_amount_dug_out = 0;
        for i in 0..width {
            for j in 0..height {
                if depths[(i, j)] != level - 1 {
                    continue;
                }
                let mut can_dig = true;
                for check_i in ((i as isize) - 1)..(i as isize + 2) {
                    for check_j in ((j as isize) - 1)..(j as isize + 2) {
                        if check_i < 0 || check_j < 0 {
                            can_dig = false;
                            continue;
                        }
                        let check_i = check_i as usize;
                        let check_j = check_j as usize;
                        if check_i >= width || check_j >= height {
                            can_dig = false;
                            continue;
                        }
                        if level - depths[(check_i, check_j)] > 1 {
                            can_dig = false;
                        }
                    }
                }
                if can_dig {
                    additional_amount_dug_out += 1;
                    depths[(i, j)] = level;
                }
            }
        }
        if additional_amount_dug_out == 0 {
            break;
        }
        amount_dug_out += additional_amount_dug_out;
        level += 1;
    }
    amount_dug_out.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!(
            "35",
            solve_part_1(
                "..........
..###.##..
...####...
..######..
..######..
...####...
.........."
            )
        );
    }

    #[test]
    fn test_solve_part_3() {
        assert_eq!(
            "29",
            solve_part_3(
                "..........
..###.##..
...####...
..######..
..######..
...####...
.........."
            )
        );
    }
}
