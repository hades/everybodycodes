use array2d::Array2D;
use log::debug;
use permutations::Permutation;

const ROTATABLE_OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
];

fn rotate(grid: &mut Array2D<char>, x: usize, y: usize, direction: char) {
    let string: Vec<_> = ROTATABLE_OFFSETS
        .iter()
        .map(|(dx, dy)| grid[(x.wrapping_add_signed(*dx), y.wrapping_add_signed(*dy))])
        .collect();
    let mut idx = 0;
    for (dx, dy) in ROTATABLE_OFFSETS {
        match direction {
            'R' => {
                grid[(x.wrapping_add_signed(dx), y.wrapping_add_signed(dy))] =
                    string[(idx + string.len() - 1) % string.len()]
            }
            'L' => {
                grid[(x.wrapping_add_signed(dx), y.wrapping_add_signed(dy))] =
                    string[(idx + 1) % string.len()]
            }
            _ => unreachable!(),
        }
        idx += 1;
    }
}

fn to_row_major(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

fn from_row_major(position: usize, width: usize) -> (usize, usize) {
    let x = position % width;
    let y = position / width;
    (x, y)
}

fn make_rotate_permutation(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    direction: char,
) -> Permutation {
    let coords_to_rotate: Vec<_> = ROTATABLE_OFFSETS
        .iter()
        .map(|(dx, dy)| {
            to_row_major(
                x.wrapping_add_signed(*dx),
                y.wrapping_add_signed(*dy),
                width,
            )
        })
        .collect();
    let mut idx = 0;
    let mut index_vector: Vec<_> = (0..(width * height)).collect();
    for (dx, dy) in ROTATABLE_OFFSETS {
        let nx = x.wrapping_add_signed(dx);
        let ny = y.wrapping_add_signed(dy);
        match direction {
            'R' => {
                index_vector[to_row_major(nx, ny, width)] =
                    coords_to_rotate[(idx + coords_to_rotate.len() - 1) % coords_to_rotate.len()]
            }
            'L' => {
                index_vector[to_row_major(nx, ny, width)] =
                    coords_to_rotate[(idx + 1) % coords_to_rotate.len()]
            }
            _ => unreachable!(),
        }
        idx += 1;
    }
    Permutation::try_from(index_vector).unwrap()
}

pub fn solve_part_1(input: &str) -> String {
    let width = input.lines().skip(2).next().unwrap().chars().count();
    let height = input.lines().count() - 2;
    let mut grid = Array2D::<char>::filled_with(' ', width, height);
    for (y, line) in input.lines().skip(2).enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid[(x, y)] = ch;
        }
    }
    let key: Vec<_> = input.lines().next().unwrap().chars().collect();
    let mut key_pos = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            debug!("{grid:?}");
            rotate(&mut grid, x, y, key[key_pos]);
            key_pos = (key_pos + 1) % key.len();
        }
    }
    let mut result = String::new();
    for y in 0..height {
        let mut is_in_string = false;
        for x in 0..width {
            if is_in_string {
                if grid[(x, y)] == '<' {
                    return result;
                }
                result.push(grid[(x, y)]);
                continue;
            }
            if grid[(x, y)] == '>' {
                is_in_string = true;
            }
        }
    }
    unreachable!()
}

pub fn solve_part_2(input: &str) -> String {
    let width = input.lines().skip(2).next().unwrap().chars().count();
    let height = input.lines().count() - 2;
    let mut grid = Array2D::<char>::filled_with(' ', width, height);
    for (y, line) in input.lines().skip(2).enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid[(x, y)] = ch;
        }
    }
    let key: Vec<_> = input.lines().next().unwrap().chars().collect();
    let mut key_pos = 0;
    let mut permutation = Permutation::identity(width * height);
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            permutation = make_rotate_permutation(x, y, width, height, key[key_pos]) * permutation;
            key_pos = (key_pos + 1) % key.len();
        }
    }
    permutation = permutation.pow(100);
    let mut result = String::new();
    for y in 0..height {
        let mut is_in_string = false;
        for x in 0..width {
            let coords = from_row_major(permutation.apply(to_row_major(x, y, width)), width);
            let (x, y) = coords;
            if is_in_string {
                if grid[(x, y)] == '<' {
                    return result;
                }
                result.push(grid[(x, y)]);
                continue;
            }
            if grid[(x, y)] == '>' {
                is_in_string = true;
            }
        }
    }
    unreachable!()
}

pub fn solve_part_3(input: &str) -> String {
    let width = input.lines().skip(2).next().unwrap().chars().count();
    let height = input.lines().count() - 2;
    let mut grid = Array2D::<char>::filled_with(' ', width, height);
    for (y, line) in input.lines().skip(2).enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid[(x, y)] = ch;
        }
    }
    let key: Vec<_> = input.lines().next().unwrap().chars().collect();
    let mut key_pos = 0;
    let mut permutation = Permutation::identity(width * height);
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            permutation = make_rotate_permutation(x, y, width, height, key[key_pos]) * permutation;
            key_pos = (key_pos + 1) % key.len();
        }
    }
    permutation = permutation.pow(1048576000);
    let mut result = String::new();
    for y in 0..height {
        let mut is_in_string = false;
        for x in 0..width {
            let coords = from_row_major(permutation.apply(to_row_major(x, y, width)), width);
            let (x, y) = coords;
            if is_in_string {
                if grid[(x, y)] == '<' {
                    return result;
                }
                result.push(grid[(x, y)]);
                continue;
            }
            if grid[(x, y)] == '>' {
                is_in_string = true;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_part_1() {
        assert_eq!(
            "WIN",
            solve_part_1(
                "LR

>-IN-
-----
W---<"
            )
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            "VICTORY",
            solve_part_2(
                "RRLL

A.VI..>...T
.CC...<...O
.....EIB.R.
.DHB...YF..
.....F..G..
D.H........"
            )
        );
    }

    #[test]
    fn test_to_row_major() {
        assert_eq!(17, to_row_major(2, 5, 3));
    }

    #[test]
    fn test_from_row_major() {
        assert_eq!((2, 5), from_row_major(17, 3));
    }

    #[test]
    fn test_make_rotate_permutation_right() {
        // A B C . .         D A B . .
        // D E F . .    =>   G E C . .
        // G H I . .         H I F . .
        let perm = make_rotate_permutation(1, 1, 5, 3, 'R');
        assert_eq!((0, 0), from_row_major(perm.apply(to_row_major(1, 0, 5)), 5));
        assert_eq!((1, 0), from_row_major(perm.apply(to_row_major(2, 0, 5)), 5));
        assert_eq!((2, 0), from_row_major(perm.apply(to_row_major(2, 1, 5)), 5));
        assert_eq!((0, 1), from_row_major(perm.apply(to_row_major(0, 0, 5)), 5));
        assert_eq!((1, 1), from_row_major(perm.apply(to_row_major(1, 1, 5)), 5));
        assert_eq!((2, 1), from_row_major(perm.apply(to_row_major(2, 2, 5)), 5));
        assert_eq!((0, 2), from_row_major(perm.apply(to_row_major(0, 1, 5)), 5));
        assert_eq!((1, 2), from_row_major(perm.apply(to_row_major(0, 2, 5)), 5));
        assert_eq!((2, 2), from_row_major(perm.apply(to_row_major(1, 2, 5)), 5));
    }

    #[test]
    fn test_make_rotate_permutation_left() {
        let perm = make_rotate_permutation(1, 1, 5, 3, 'L');
        assert_eq!((1, 0), from_row_major(perm.apply(to_row_major(0, 0, 5)), 5));
        assert_eq!((2, 0), from_row_major(perm.apply(to_row_major(1, 0, 5)), 5));
        assert_eq!((2, 1), from_row_major(perm.apply(to_row_major(2, 0, 5)), 5));
        assert_eq!((0, 0), from_row_major(perm.apply(to_row_major(0, 1, 5)), 5));
        assert_eq!((1, 1), from_row_major(perm.apply(to_row_major(1, 1, 5)), 5));
        assert_eq!((2, 2), from_row_major(perm.apply(to_row_major(2, 1, 5)), 5));
        assert_eq!((0, 1), from_row_major(perm.apply(to_row_major(0, 2, 5)), 5));
        assert_eq!((0, 2), from_row_major(perm.apply(to_row_major(1, 2, 5)), 5));
        assert_eq!((1, 2), from_row_major(perm.apply(to_row_major(2, 2, 5)), 5));
    }
}
