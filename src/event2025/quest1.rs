pub fn solve_part_1(input: &str) -> String {
    let (names, instructions) = input.split_once("\n\n").unwrap();
    let names: Vec<&str> = names.split(",").collect();
    let instructions: Vec<&str> = instructions.split(",").collect();
    let mut ptr = 0usize;
    for instruction in instructions {
        if let Some(left_shift_str) = instruction.strip_prefix("L") {
            let left_shift: usize = left_shift_str.parse().unwrap();
            ptr = ptr.saturating_sub(left_shift);
            continue;
        }
        if let Some(right_shift_str) = instruction.strip_prefix("R") {
            let right_shift: usize = right_shift_str.parse().unwrap();
            ptr += right_shift;
            ptr = ptr.min(names.len() - 1);
            continue;
        }
    }
    names[ptr].to_string()
}

pub fn solve_part_2(input: &str) -> String {
    let (names, instructions) = input.split_once("\n\n").unwrap();
    let names: Vec<&str> = names.split(",").collect();
    let instructions: Vec<&str> = instructions.split(",").collect();
    let mut ptr = 0;
    for instruction in instructions {
        if let Some(left_shift_str) = instruction.strip_prefix("L") {
            let left_shift: usize = left_shift_str.parse().unwrap();
            ptr += names.len() - left_shift;
            ptr %= names.len();
            continue;
        }
        if let Some(right_shift_str) = instruction.strip_prefix("R") {
            let right_shift: usize = right_shift_str.parse().unwrap();
            ptr += right_shift;
            ptr %= names.len();
            continue;
        }
    }
    names[ptr].to_string()
}

pub fn solve_part_3(input: &str) -> String {
    let (names, instructions) = input.split_once("\n\n").unwrap();
    let mut names: Vec<&str> = names.split(",").collect();
    let instructions: Vec<&str> = instructions.split(",").collect();
    for instruction in instructions {
        if let Some(left_shift_str) = instruction.strip_prefix("L") {
            let left_shift: usize = left_shift_str.parse().unwrap();
            let ptr = (names.len() - (left_shift % names.len())) % names.len();
            names.swap(0, ptr);
            continue;
        }
        if let Some(right_shift_str) = instruction.strip_prefix("R") {
            let right_shift: usize = right_shift_str.parse().unwrap();
            let ptr = right_shift % names.len();
            names.swap(0, ptr);
            continue;
        }
    }
    names[0].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!(
            "Fyrryn",
            solve_part_1(
                "Vyrdax,Drakzyph,Fyrryn,Elarzris

R3,L2,R3,L1"
            )
        );
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(
            "Elarzris",
            solve_part_2(
                "Vyrdax,Drakzyph,Fyrryn,Elarzris

R3,L2,R3,L1"
            )
        );
    }

    #[test]
    fn test_solve_part_3() {
        assert_eq!(
            "Drakzyph",
            solve_part_3(
                "Vyrdax,Drakzyph,Fyrryn,Elarzris

R3,L2,R3,L3"
            )
        );
    }
}
