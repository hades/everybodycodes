use log::debug;
use z3::{
    Config, Context, FuncDecl, Optimize, Sort,
    ast::{Ast, Dynamic, Int},
};

fn number_of_beetles_for_brightness(stamps: &[i64], brightness: i64) -> i64 {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Optimize::new(&ctx);
    let count_funcs: Vec<FuncDecl> = stamps
        .iter()
        .map(|stamp_value| {
            FuncDecl::new(
                &ctx,
                format!("stamp{}", *stamp_value),
                &[],
                &Sort::int(&ctx),
            )
        })
        .collect();
    // S_i >= 0
    count_funcs
        .iter()
        .for_each(|f| solver.assert(&f.apply(&[]).as_int().unwrap().ge(&Int::from_i64(&ctx, 0))));
    // sum = \sum S_i * V_i
    let mut sum_ast: Box<dyn Ast> = Box::new(Int::from_i64(&ctx, 0));
    for (i, stamp_value) in stamps.iter().enumerate() {
        sum_ast = Box::new(
            (count_funcs[i].apply(&[]).as_int().unwrap() * Int::from_i64(&ctx, *stamp_value))
                + Dynamic::from_ast(sum_ast.as_ref()).as_int().unwrap(),
        );
    }
    let sum_func = FuncDecl::new(&ctx, "sum", &[], &Sort::int(&ctx));
    solver.assert(
        &(sum_func
            .apply(&[])
            ._eq(&Dynamic::from_ast(sum_ast.as_ref()))),
    );
    // sum = brightness
    solver.assert(
        &(sum_func
            .apply(&[])
            ._eq(&Dynamic::from_ast(&Int::from_i64(&ctx, brightness)))),
    );

    let mut count_ast: Box<dyn Ast> = Box::new(Int::from_i64(&ctx, 0));
    for func in count_funcs {
        count_ast = Box::new(
            func.apply(&[]).as_int().unwrap()
                + Dynamic::from_ast(count_ast.as_ref()).as_int().unwrap(),
        );
    }
    let count_func = FuncDecl::new(&ctx, "count", &[], &Sort::int(&ctx));
    solver.assert(
        &(count_func
            .apply(&[])
            ._eq(&Dynamic::from_ast(count_ast.as_ref()))),
    );
    solver.minimize(&count_func.apply(&[]));
    debug!("{}", solver.to_string());
    match solver.check(&[]) {
        z3::SatResult::Sat => {
            debug!("{:?}", solver.get_model());
            solver
                .get_model()
                .unwrap()
                .eval(&count_func.apply(&[]), false)
                .unwrap()
                .as_int()
                .unwrap()
                .as_i64()
                .unwrap()
        }
        e => panic!("solver failed: {:?}", e),
    }
}

fn number_of_beetles_for_brightness_split(stamps: &[i64], brightness: i64) -> i64 {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Optimize::new(&ctx);
    let count_funcs_left: Vec<FuncDecl> = stamps
        .iter()
        .map(|stamp_value| {
            FuncDecl::new(
                &ctx,
                format!("stamp_left{}", *stamp_value),
                &[],
                &Sort::int(&ctx),
            )
        })
        .collect();
    count_funcs_left
        .iter()
        .for_each(|f| solver.assert(&f.apply(&[]).as_int().unwrap().ge(&Int::from_i64(&ctx, 0))));
    let mut sum_ast_left: Box<dyn Ast> = Box::new(Int::from_i64(&ctx, 0));
    for (i, stamp_value) in stamps.iter().enumerate() {
        sum_ast_left = Box::new(
            (count_funcs_left[i].apply(&[]).as_int().unwrap() * Int::from_i64(&ctx, *stamp_value))
                + Dynamic::from_ast(sum_ast_left.as_ref()).as_int().unwrap(),
        );
    }
    let sum_func_left = FuncDecl::new(&ctx, "sum_left", &[], &Sort::int(&ctx));
    solver.assert(
        &(sum_func_left
            .apply(&[])
            ._eq(&Dynamic::from_ast(sum_ast_left.as_ref()))),
    );

    let count_funcs_right: Vec<FuncDecl> = stamps
        .iter()
        .map(|stamp_value| {
            FuncDecl::new(
                &ctx,
                format!("stamp_right{}", *stamp_value),
                &[],
                &Sort::int(&ctx),
            )
        })
        .collect();
    count_funcs_right
        .iter()
        .for_each(|f| solver.assert(&f.apply(&[]).as_int().unwrap().ge(&Int::from_i64(&ctx, 0))));
    let mut sum_ast_right: Box<dyn Ast> = Box::new(Int::from_i64(&ctx, 0));
    for (i, stamp_value) in stamps.iter().enumerate() {
        sum_ast_right = Box::new(
            (count_funcs_right[i].apply(&[]).as_int().unwrap() * Int::from_i64(&ctx, *stamp_value))
                + Dynamic::from_ast(sum_ast_right.as_ref()).as_int().unwrap(),
        );
    }
    let sum_func_right = FuncDecl::new(&ctx, "sum_right", &[], &Sort::int(&ctx));
    solver.assert(
        &(sum_func_right
            .apply(&[])
            ._eq(&Dynamic::from_ast(sum_ast_right.as_ref()))),
    );

    let sum_func = FuncDecl::new(&ctx, "sum", &[], &Sort::int(&ctx));
    solver.assert(
        &((sum_func_right.apply(&[]).as_int().unwrap()
            + sum_func_left.apply(&[]).as_int().unwrap())
        ._eq(&sum_func.apply(&[]).as_int().unwrap())),
    );
    solver.assert(
        &((sum_func_right.apply(&[]).as_int().unwrap()
            - sum_func_left.apply(&[]).as_int().unwrap())
        .le(&Int::from_i64(&ctx, 100))),
    );
    solver.assert(
        &((sum_func_left.apply(&[]).as_int().unwrap()
            - sum_func_right.apply(&[]).as_int().unwrap())
        .le(&Int::from_i64(&ctx, 100))),
    );
    solver.assert(
        &(sum_func
            .apply(&[])
            .as_int()
            .unwrap()
            ._eq(&Int::from_i64(&ctx, brightness))),
    );

    let mut count_ast: Box<dyn Ast> = Box::new(Int::from_i64(&ctx, 0));
    for func in count_funcs_left {
        count_ast = Box::new(
            func.apply(&[]).as_int().unwrap()
                + Dynamic::from_ast(count_ast.as_ref()).as_int().unwrap(),
        );
    }
    for func in count_funcs_right {
        count_ast = Box::new(
            func.apply(&[]).as_int().unwrap()
                + Dynamic::from_ast(count_ast.as_ref()).as_int().unwrap(),
        );
    }
    let count_func = FuncDecl::new(&ctx, "count", &[], &Sort::int(&ctx));
    solver.assert(
        &(count_func
            .apply(&[])
            ._eq(&Dynamic::from_ast(count_ast.as_ref()))),
    );
    solver.minimize(&count_func.apply(&[]));
    debug!("{}", solver.to_string());
    match solver.check(&[]) {
        z3::SatResult::Sat => {
            debug!("{:?}", solver.get_model());
            solver
                .get_model()
                .unwrap()
                .eval(&count_func.apply(&[]), false)
                .unwrap()
                .as_int()
                .unwrap()
                .as_i64()
                .unwrap()
        }
        e => panic!("solver failed: {:?}", e),
    }
}

pub fn solve_part_1(input: &str) -> String {
    let stamps = [10, 5, 3, 1];
    input
        .lines()
        .map(|l| number_of_beetles_for_brightness(&stamps, l.parse().unwrap()))
        .sum::<i64>()
        .to_string()
}

pub fn solve_part_2(input: &str) -> String {
    let stamps = [30, 25, 24, 20, 16, 15, 10, 5, 3, 1];
    input
        .lines()
        .map(|l| number_of_beetles_for_brightness(&stamps, l.parse().unwrap()))
        .sum::<i64>()
        .to_string()
}

pub fn solve_part_3(input: &str) -> String {
    let stamps = [
        1, 3, 5, 10, 15, 16, 20, 24, 25, 30, 37, 38, 49, 50, 74, 75, 100, 101,
    ];
    input
        .lines()
        .map(|l| number_of_beetles_for_brightness_split(&stamps, l.parse().unwrap()))
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_log::test;

    #[test]
    fn test_part_1() {
        assert_eq!("10", solve_part_1("2\n4\n7\n16"))
    }

    #[test]
    fn test_part_2() {
        assert_eq!("10", solve_part_2("33\n41\n55\n99"));
        assert_eq!("2", solve_part_2("49"));
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            "10449",
            solve_part_3(
                "156488
352486
546212"
            )
        );
    }
}
