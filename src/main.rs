mod ecclient;
mod quest1;
mod quest2;
mod quest3;
mod quest4;
mod quest5;
mod quest6;
mod quest7;
mod types;

use std::env;
use std::thread;

use clap::Parser;
use types::Part;
use types::PuzzleKey;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    submit: bool,

    #[arg(short, long)]
    cookie: Option<String>,

    event: i16,
    quest: i8,
    part: i8,
}

fn get_solver(puzzle_key: &PuzzleKey) -> Box<dyn Fn(&str) -> String> {
    match puzzle_key {
        PuzzleKey {
            event: 2024,
            quest: 1,
            part: Part::One,
        } => Box::new(quest1::solve_part_1),
        PuzzleKey {
            event: 2024,
            quest: 1,
            part: Part::Two,
        } => Box::new(quest1::solve_part_2),
        PuzzleKey {
            event: 2024,
            quest: 1,
            part: Part::Three,
        } => Box::new(quest1::solve_part_3),
        PuzzleKey {
            event: 2024,
            quest: 2,
            part: Part::One,
        } => Box::new(quest2::solve_part_1),
        PuzzleKey {
            event: 2024,
            quest: 2,
            part: Part::Two,
        } => Box::new(quest2::solve_part_2),
        PuzzleKey {
            event: 2024,
            quest: 2,
            part: Part::Three,
        } => Box::new(quest2::solve_part_3),
        PuzzleKey {
            event: 2024,
            quest: 3,
            part: Part::One,
        } => Box::new(quest3::solve_part_1),
        PuzzleKey {
            event: 2024,
            quest: 3,
            part: Part::Two,
        } => Box::new(quest3::solve_part_2),
        PuzzleKey {
            event: 2024,
            quest: 3,
            part: Part::Three,
        } => Box::new(quest3::solve_part_3),
        PuzzleKey {
            event: 2024,
            quest: 4,
            part: Part::One,
        } => Box::new(quest4::solve_part_1),
        PuzzleKey {
            event: 2024,
            quest: 4,
            part: Part::Two,
        } => Box::new(quest4::solve_part_1),
        PuzzleKey {
            event: 2024,
            quest: 4,
            part: Part::Three,
        } => Box::new(quest4::solve_part_3),
        PuzzleKey {
            event: 2024,
            quest: 5,
            part: Part::One,
        } => Box::new(quest5::solve_part_1),
        PuzzleKey {
            event: 2024,
            quest: 5,
            part: Part::Two,
        } => Box::new(quest5::solve_part_2),
        PuzzleKey {
            event: 2024,
            quest: 5,
            part: Part::Three,
        } => Box::new(quest5::solve_part_3),
        PuzzleKey {
            event: 2024,
            quest: 6,
            part: Part::One,
        } => Box::new(quest6::solve_part_1),
        PuzzleKey {
            event: 2024,
            quest: 6,
            part: Part::Two,
        } => Box::new(quest6::solve_part_2),
        PuzzleKey {
            event: 2024,
            quest: 6,
            part: Part::Three,
        } => Box::new(quest6::solve_part_3),
        PuzzleKey {
            event: 2024,
            quest: 7,
            part: Part::One,
        } => Box::new(quest7::solve_part_1),
        PuzzleKey {
            event: 2024,
            quest: 7,
            part: Part::Two,
        } => Box::new(quest7::solve_part_2),
        PuzzleKey {
            event: 2024,
            quest: 7,
            part: Part::Three,
        } => Box::new(quest7::solve_part_3),
        _ => panic!("solver not found for {:?}", puzzle_key),
    }
}

fn main() {
    pretty_env_logger::init();
    log::info!("Everybody Codes solver");

    let args = Args::parse();

    // Get the EC cookie, either from the command line, or from the env variable.
    let cookie = if let Some(cookie) = args.cookie {
        cookie
    } else if let Ok(cookie) = env::var("EC_COOKIE") {
        cookie
    } else {
        panic!("you must specify the session cookie with --cookie or EC_COOKIE env variable");
    };
    let key = types::PuzzleKey {
        event: args.event,
        quest: args.quest,
        part: match args.part {
            1 => types::Part::One,
            2 => types::Part::Two,
            3 => types::Part::Three,
            _ => panic!("part should be in [1,2,3], got {}", args.part),
        },
    };
    let solver = get_solver(&key);
    let client = ecclient::EcClient::new(cookie.as_str()).expect("creating EC client");
    log::info!(
        "solving Everybody Codes event {} quest {} part {}",
        args.event,
        args.quest,
        args.part
    );
    log::info!("retrieving puzzle input...");
    match client.get_puzzle_input(&key) {
        Ok(input) => {
            log::info!("solving...");
            let solution = solver(input.as_str());
            log::info!("solution: {}", solution);
            if args.submit {
                if solution.is_empty() {
                    log::warn!("refusing to submit an empty solution");
                } else {
                    if let Some(delay) = client.get_penalty_delay().unwrap() {
                        log::info!("sleeping for {:?} before submitting...", &delay);
                        thread::sleep(delay);
                    }
                    log::info!("submitting the answer...");
                    let result = client.post_answer(&key, solution.as_str());
                    log::info!("result: {:#?}", result);
                }
            }
        }
        Err(e) => {
            log::error!("error retrieving puzzle input: {e:#?}");
        }
    }
}
