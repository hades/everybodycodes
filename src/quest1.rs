fn potions_for(monster: char) -> i64 {
    match monster {
        'A' => 0,
        'B' => 1,
        'C' => 3,
        'D' => 5,
        'x' => 0,
        _ => panic!("unexpected char: {}", monster),
    }
}

pub fn solve_part_1(input: &str) -> String {
    let potions: i64 = input.chars().map(potions_for).sum();
    potions.to_string()
}

pub fn is_monster(ch: char) -> bool {
    ch != 'x'
}

pub fn solve_part_2(input: &str) -> String {
    let mut total_potions: i64 = 0;
    let mut chars = input.chars();
    loop {
        let monster1 = chars.next();
        let monster2 = chars.next();
        if monster1.is_none() || monster2.is_none() {
            break;
        }
        let monster1 = monster1.unwrap();
        let monster2 = monster2.unwrap();
        let mut potions = potions_for(monster1) + potions_for(monster2);
        if is_monster(monster1) && is_monster(monster2) {
            potions += 2;
        }
        total_potions += potions;
    }
    total_potions.to_string()
}

pub fn solve_part_3(input: &str) -> String {
    let mut total_potions: i64 = 0;
    let mut chars = input.chars();
    loop {
        let monster1 = chars.next();
        let monster2 = chars.next();
        let monster3 = chars.next();
        if monster1.is_none() || monster2.is_none() || monster3.is_none() {
            break;
        }
        let monster1 = monster1.unwrap();
        let monster2 = monster2.unwrap();
        let monster3 = monster3.unwrap();
        let potions = potions_for(monster1) + potions_for(monster2) + potions_for(monster3);
        let monster_count = [monster1, monster2, monster3]
            .iter()
            .map(|m| if is_monster(*m) { 1 } else { 0 })
            .sum();
        total_potions += potions
            + match monster_count {
                3 => 6,
                2 => 2,
                _ => 0,
            };
    }
    total_potions.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!("5", solve_part_1("ABBAC"));
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!("28", solve_part_2("AxBCDDCAxD"));
    }

    #[test]
    fn test_solve_part_3() {
        assert_eq!("30", solve_part_3("xBxAAABCDxCC"));
    }
}
