use std::cmp;

pub fn solve_part_1(input: &str) -> String {
    let available_blocks: i64 = input.parse().unwrap();
    let pyramid_size_index = available_blocks.isqrt() + 1;
    let missing_blocks = pyramid_size_index * pyramid_size_index - available_blocks;
    let width = pyramid_size_index * 2 - 1;
    (width * missing_blocks).to_string()
}

fn solve_part_2_with_params(input: &str, acolytes: i64, blocks_available: i64) -> String {
    let priests: i64 = input.parse().unwrap();
    let mut blocks_remaining = blocks_available - 1;
    let mut last_layer_thickness = 1;
    let mut width = 1;
    loop {
        width += 2;
        let thickness = (last_layer_thickness * priests) % acolytes;
        let blocks_for_layer_needed = width * thickness;
        if blocks_for_layer_needed > blocks_remaining {
            let extra_blocks_needed = blocks_for_layer_needed - blocks_remaining;
            return (extra_blocks_needed * width).to_string();
        }
        blocks_remaining -= blocks_for_layer_needed;
        last_layer_thickness = thickness;
    } 
}

pub fn solve_part_2(input: &str) -> String {
    solve_part_2_with_params(input, 1111, 20240000)
}

fn solve_part_3_with_params(input: &str, acolytes: i64, blocks_available: i64) -> String {
    let priests: i64 = input.parse().unwrap();
    let mut column_heights: Vec<i64> = vec![1];
    let mut last_layer_thickness = 1;
    let mut width = 1;
    loop {
        width += 2;
        let thickness = (last_layer_thickness * priests) % acolytes + acolytes;
        column_heights.push(0);
        column_heights.iter_mut().for_each(|i| *i += thickness);
        let mut blocks_needed = 0;
        for (i, height) in column_heights.iter().enumerate() {
            let maximum_removable = if i == column_heights.len() - 1 { 0 } else { column_heights[i+1] - 1 };
            let blocks_removed = cmp::min(maximum_removable, (((priests * height) % acolytes) * width) % acolytes);
            if i > 0 {
                blocks_needed += 2 * (height - blocks_removed);
            } else {
                blocks_needed += height - blocks_removed;
            }
        }
        if blocks_needed > blocks_available {
            return (blocks_needed - blocks_available).to_string()
        }
        last_layer_thickness = thickness;
    } 
}

pub fn solve_part_3(input: &str) -> String {
    solve_part_3_with_params(input, 10, 202400000)
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!("21", solve_part_1("13"));
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!("27", solve_part_2_with_params("3", 5, 50));
    }

    #[test]
    fn test_solve_part_3() {
        assert_eq!("2", solve_part_3_with_params("2", 5, 160));
        assert_eq!("1", solve_part_3_with_params("2", 5, 18));
        assert_eq!("1", solve_part_3_with_params("2", 5, 66));
        assert_eq!("1", solve_part_3_with_params("2", 5, 114));
        assert_eq!("1", solve_part_3_with_params("2", 5, 161));
        assert_eq!("1", solve_part_3_with_params("2", 5, 238));
        assert_eq!("1", solve_part_3_with_params("2", 5, 352));
        assert_eq!("1", solve_part_3_with_params("2", 5, 490));
        assert_eq!("1", solve_part_3_with_params("2", 5, 568));
        assert_eq!("1", solve_part_3_with_params("2", 5, 689));
        assert_eq!("1", solve_part_3_with_params("2", 5, 1884));
        assert_eq!("1", solve_part_3_with_params("2", 5, 7600));
        assert_eq!("1", solve_part_3_with_params("2", 5, 30654));
        assert_eq!("1", solve_part_3_with_params("2", 5, 123130));
        assert_eq!("1", solve_part_3_with_params("2", 5, 491004));
        assert_eq!("1", solve_part_3_with_params("2", 5, 1964800));
        assert_eq!("1", solve_part_3_with_params("2", 5, 7863294));
        assert_eq!("1", solve_part_3_with_params("2", 5, 31461370));
    }
}
