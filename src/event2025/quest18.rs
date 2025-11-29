use regex::Regex;
use z3::{
    Optimize, Params,
    ast::{Bool, Int},
};

#[derive(Default)]
struct Plant {
    thickness: i64,
    free: Option<i64>,
    connected: Vec<(usize, i64)>,
}

fn parse_plant_spec(input: &str) -> Plant {
    let mut result = Plant::default();
    let first_re = Regex::new(r"Plant \d+ with thickness (\d+):").unwrap();
    let free_re = Regex::new(r"- free branch with thickness (\d+)").unwrap();
    let branch_re = Regex::new(r"- branch to Plant (\d+) with thickness (-?\d+)").unwrap();
    for line in input.lines() {
        if let Some((_, [thickness])) = first_re.captures(line).map(|c| c.extract()) {
            result.thickness = thickness.parse().unwrap();
        } else if let Some((_, [thickness])) = free_re.captures(line).map(|c| c.extract()) {
            result.free = Some(thickness.parse().unwrap());
        } else if let Some((_, [plant, thickness])) = branch_re.captures(line).map(|c| c.extract())
        {
            result
                .connected
                .push((plant.parse().unwrap(), thickness.parse().unwrap()));
        } else {
            panic!("cannot parse line: {line}");
        }
    }
    result
}

fn eval_plant(plants: &[Plant], number: usize, free_branches: &[i64]) -> i64 {
    let plant = &plants[number - 1];
    if plant.free.is_some() {
        assert_eq!(1, plant.thickness);
        assert_eq!(1, plant.free.unwrap());
        free_branches[number - 1]
    } else {
        let incoming = plant
            .connected
            .iter()
            .map(|&(plant_number, branch_thickness)| {
                eval_plant(plants, plant_number, free_branches) * branch_thickness
            })
            .sum::<i64>();
        if incoming >= plant.thickness {
            incoming
        } else {
            0
        }
    }
}

pub fn solve_part_1(input: &str) -> String {
    let plants = input
        .split("\n\n")
        .map(parse_plant_spec)
        .collect::<Vec<_>>();
    eval_plant(&plants, plants.len(), &vec![1; plants.len()]).to_string()
}

pub fn solve_part_2(input: &str) -> String {
    let (plants, tests) = input.split_once("\n\n\n").unwrap();
    let plants = plants
        .split("\n\n")
        .map(parse_plant_spec)
        .collect::<Vec<_>>();
    tests
        .lines()
        .map(|test| {
            eval_plant(
                &plants,
                plants.len(),
                &test
                    .split(" ")
                    .map(|v| v.parse().unwrap())
                    .collect::<Vec<i64>>(),
            )
        })
        .sum::<i64>()
        .to_string()
}

fn eval_plant_z3(plants: &[Plant], number: usize, free_branches: &[Option<Bool>]) -> Int {
    let plant = &plants[number - 1];
    if plant.free.is_some() {
        assert_eq!(1, plant.thickness);
        assert_eq!(1, plant.free.unwrap());
        free_branches[number - 1]
            .as_ref()
            .unwrap()
            .ite(&Int::from_i64(1), &Int::from_i64(0))
    } else {
        let incoming = plant
            .connected
            .iter()
            .map(|&(plant_number, branch_thickness)| {
                eval_plant_z3(plants, plant_number, free_branches) * Int::from_i64(branch_thickness)
            })
            .reduce(|a, b| a + b);
        let incoming = incoming.unwrap_or_else(|| Int::from_i64(0));
        incoming
            .ge(Int::from_i64(plant.thickness))
            .ite(&incoming, &Int::from_i64(0))
    }
}

fn maximum_achievable_brightness(plants: &[Plant]) -> i64 {
    let mut free_branches = vec![None; plants.len()];
    plants.iter().enumerate().for_each(|(i, p)| {
        if p.free.is_some() {
            free_branches[i] = Some(Bool::fresh_const("free"));
        }
    });
    let solver = Optimize::new();
    let mut params = Params::new();
    params.set_symbol("opt.maxsat_engine", "wmax");
    solver.set_params(&params);
    let brightness = eval_plant_z3(plants, plants.len(), &free_branches);
    solver.maximize(&brightness);
    match solver.check(&[]) {
        z3::SatResult::Sat => solver
            .get_model()
            .unwrap()
            .eval(&brightness, true)
            .unwrap()
            .as_i64()
            .unwrap(),
        _ => panic!("unsat"),
    }
}

pub fn solve_part_3(input: &str) -> String {
    let (plants, tests) = input.split_once("\n\n\n").unwrap();
    let plants = plants
        .split("\n\n")
        .map(parse_plant_spec)
        .collect::<Vec<_>>();
    let maximum = maximum_achievable_brightness(&plants);
    tests
        .lines()
        .map(|test| {
            eval_plant(
                &plants,
                plants.len(),
                &test
                    .split(" ")
                    .map(|v| v.parse().unwrap())
                    .collect::<Vec<i64>>(),
            )
        })
        .map(|v| if v > 0 { maximum - v } else { 0 })
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!(
            "774",
            solve_part_1(
                "Plant 1 with thickness 1:
- free branch with thickness 1

Plant 2 with thickness 1:
- free branch with thickness 1

Plant 3 with thickness 1:
- free branch with thickness 1

Plant 4 with thickness 17:
- branch to Plant 1 with thickness 15
- branch to Plant 2 with thickness 3

Plant 5 with thickness 24:
- branch to Plant 2 with thickness 11
- branch to Plant 3 with thickness 13

Plant 6 with thickness 15:
- branch to Plant 3 with thickness 14

Plant 7 with thickness 10:
- branch to Plant 4 with thickness 15
- branch to Plant 5 with thickness 21
- branch to Plant 6 with thickness 34"
            )
        );
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(
            "324",
            solve_part_2(
                "Plant 1 with thickness 1:
- free branch with thickness 1

Plant 2 with thickness 1:
- free branch with thickness 1

Plant 3 with thickness 1:
- free branch with thickness 1

Plant 4 with thickness 10:
- branch to Plant 1 with thickness -25
- branch to Plant 2 with thickness 17
- branch to Plant 3 with thickness 12

Plant 5 with thickness 14:
- branch to Plant 1 with thickness 14
- branch to Plant 2 with thickness -26
- branch to Plant 3 with thickness 15

Plant 6 with thickness 150:
- branch to Plant 4 with thickness 5
- branch to Plant 5 with thickness 6


1 0 1
0 0 1
0 1 1"
            )
        );
    }

    #[test]
    fn test_solve_part_3() {
        assert_eq!(
            "946",
            solve_part_3(
                "Plant 1 with thickness 1:
- free branch with thickness 1

Plant 2 with thickness 1:
- free branch with thickness 1

Plant 3 with thickness 1:
- free branch with thickness 1

Plant 4 with thickness 1:
- free branch with thickness 1

Plant 5 with thickness 8:
- branch to Plant 1 with thickness -8
- branch to Plant 2 with thickness 11
- branch to Plant 3 with thickness 13
- branch to Plant 4 with thickness -7

Plant 6 with thickness 7:
- branch to Plant 1 with thickness 14
- branch to Plant 2 with thickness -9
- branch to Plant 3 with thickness 12
- branch to Plant 4 with thickness 9

Plant 7 with thickness 23:
- branch to Plant 5 with thickness 17
- branch to Plant 6 with thickness 18


0 1 0 0
0 1 0 1
0 1 1 1
1 1 0 1"
            )
        );
    }
}
