mod autosubmit;
mod ecclient;
mod event1;
mod event2;
mod event2024;
mod event2025;
mod types;
mod util;

use std::env;
use std::thread;

use autosubmit::submit_with_cache;
use clap::Parser;
use pretty_duration::pretty_duration;
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

macro_rules! try_use_solver {
    ($key: expr, $event: pat, $quest: pat, $part: pat, $solve_fn: path) => {
        if matches!(
            *$key,
            PuzzleKey {
                event: $event,
                quest: $quest,
                part: $part,
            }
        ) {
            return Box::new($solve_fn);
        }
    };
}

fn get_solver(puzzle_key: &PuzzleKey) -> Box<dyn Fn(&str) -> String> {
    try_use_solver!(
        puzzle_key,
        2024,
        1,
        Part::One,
        event2024::quest1::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2024,
        1,
        Part::Two,
        event2024::quest1::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2024,
        1,
        Part::Three,
        event2024::quest1::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2024,
        2,
        Part::One,
        event2024::quest2::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2024,
        2,
        Part::Two,
        event2024::quest2::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2024,
        2,
        Part::Three,
        event2024::quest2::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2024,
        3,
        Part::One,
        event2024::quest3::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2024,
        3,
        Part::Two,
        event2024::quest3::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2024,
        3,
        Part::Three,
        event2024::quest3::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2024,
        4,
        Part::One,
        event2024::quest4::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2024,
        4,
        Part::Two,
        event2024::quest4::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2024,
        4,
        Part::Three,
        event2024::quest4::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2024,
        5,
        Part::One,
        event2024::quest5::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2024,
        5,
        Part::Two,
        event2024::quest5::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2024,
        5,
        Part::Three,
        event2024::quest5::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2024,
        6,
        Part::One,
        event2024::quest6::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2024,
        6,
        Part::Two,
        event2024::quest6::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2024,
        6,
        Part::Three,
        event2024::quest6::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2024,
        7,
        Part::One,
        event2024::quest7::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2024,
        7,
        Part::Two,
        event2024::quest7::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2024,
        7,
        Part::Three,
        event2024::quest7::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2024,
        8,
        Part::One,
        event2024::quest8::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2024,
        8,
        Part::Two,
        event2024::quest8::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2024,
        8,
        Part::Three,
        event2024::quest8::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2024,
        9,
        Part::One,
        event2024::quest9::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2024,
        9,
        Part::Two,
        event2024::quest9::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2024,
        9,
        Part::Three,
        event2024::quest9::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2024,
        10,
        Part::One,
        event2024::quest10::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2024,
        10,
        Part::Two,
        event2024::quest10::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2024,
        10,
        Part::Three,
        event2024::quest10::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2024,
        11,
        Part::One,
        event2024::quest11::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2024,
        11,
        Part::Two,
        event2024::quest11::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2024,
        11,
        Part::Three,
        event2024::quest11::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2024,
        12,
        Part::One,
        event2024::quest12::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2024,
        12,
        Part::Two,
        event2024::quest12::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2024,
        12,
        Part::Three,
        event2024::quest12::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2024,
        13,
        Part::One,
        event2024::quest13::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2024,
        13,
        Part::Two,
        event2024::quest13::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2024,
        13,
        Part::Three,
        event2024::quest13::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2024,
        14,
        Part::One,
        event2024::quest14::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2024,
        14,
        Part::Two,
        event2024::quest14::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2024,
        14,
        Part::Three,
        event2024::quest14::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2024,
        15,
        Part::One,
        event2024::quest15::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2024,
        15,
        Part::Two,
        event2024::quest15::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2024,
        15,
        Part::Three,
        event2024::quest15::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2024,
        16,
        Part::One,
        event2024::quest16::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2024,
        16,
        Part::Two,
        event2024::quest16::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2024,
        16,
        Part::Three,
        event2024::quest16::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2024,
        17,
        Part::One,
        event2024::quest17::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2024,
        17,
        Part::Two,
        event2024::quest17::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2024,
        17,
        Part::Three,
        event2024::quest17::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2024,
        18,
        Part::One,
        event2024::quest18::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2024,
        18,
        Part::Two,
        event2024::quest18::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2024,
        18,
        Part::Three,
        event2024::quest18::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2024,
        19,
        Part::One,
        event2024::quest19::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2024,
        19,
        Part::Two,
        event2024::quest19::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2024,
        19,
        Part::Three,
        event2024::quest19::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2024,
        20,
        Part::One,
        event2024::quest20::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2024,
        20,
        Part::Two,
        event2024::quest20::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2024,
        20,
        Part::Three,
        event2024::quest20::solve_part_3
    );
    try_use_solver!(puzzle_key, 1, 1, Part::One, event1::quest1::solve_part_1);
    try_use_solver!(puzzle_key, 1, 1, Part::Two, event1::quest1::solve_part_2);
    try_use_solver!(puzzle_key, 1, 1, Part::Three, event1::quest1::solve_part_3);
    try_use_solver!(puzzle_key, 1, 2, Part::One, event1::quest2::solve_part_1);
    try_use_solver!(puzzle_key, 1, 2, Part::Two, event1::quest2::solve_part_2);
    try_use_solver!(puzzle_key, 1, 2, Part::Three, event1::quest2::solve_part_3);
    try_use_solver!(puzzle_key, 1, 3, Part::One, event1::quest3::solve_part_one);
    try_use_solver!(puzzle_key, 1, 3, Part::Two, event1::quest3::solve_part_two);
    try_use_solver!(
        puzzle_key,
        1,
        3,
        Part::Three,
        event1::quest3::solve_part_three
    );
    try_use_solver!(puzzle_key, 2, 1, Part::One, event2::quest1::solve_part_1);
    try_use_solver!(puzzle_key, 2, 1, Part::Two, event2::quest1::solve_part_2);
    try_use_solver!(puzzle_key, 2, 1, Part::Three, event2::quest1::solve_part_3);
    try_use_solver!(puzzle_key, 2, 2, Part::One, event2::quest2::solve_part_1);
    try_use_solver!(puzzle_key, 2, 2, Part::Two, event2::quest2::solve_part_2);
    try_use_solver!(puzzle_key, 2, 2, Part::Three, event2::quest2::solve_part_3);
    try_use_solver!(puzzle_key, 2, 3, Part::One, event2::quest3::solve_part_one);
    try_use_solver!(puzzle_key, 2, 3, Part::Two, event2::quest3::solve_part_two);
    try_use_solver!(
        puzzle_key,
        2,
        3,
        Part::Three,
        event2::quest3::solve_part_three
    );
    try_use_solver!(
        puzzle_key,
        2025,
        1,
        Part::One,
        event2025::quest1::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2025,
        1,
        Part::Two,
        event2025::quest1::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2025,
        1,
        Part::Three,
        event2025::quest1::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2025,
        2,
        Part::One,
        event2025::quest2::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2025,
        2,
        Part::Two,
        event2025::quest2::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2025,
        2,
        Part::Three,
        event2025::quest2::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2025,
        3,
        Part::One,
        event2025::quest3::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2025,
        3,
        Part::Two,
        event2025::quest3::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2025,
        3,
        Part::Three,
        event2025::quest3::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2025,
        4,
        Part::One,
        event2025::quest4::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2025,
        4,
        Part::Two,
        event2025::quest4::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2025,
        4,
        Part::Three,
        event2025::quest4::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2025,
        5,
        Part::One,
        event2025::quest5::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2025,
        5,
        Part::Two,
        event2025::quest5::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2025,
        5,
        Part::Three,
        event2025::quest5::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2025,
        6,
        Part::One,
        event2025::quest6::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2025,
        6,
        Part::Two,
        event2025::quest6::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2025,
        6,
        Part::Three,
        event2025::quest6::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2025,
        7,
        Part::One,
        event2025::quest7::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2025,
        7,
        Part::Two,
        event2025::quest7::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2025,
        7,
        Part::Three,
        event2025::quest7::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2025,
        8,
        Part::One,
        event2025::quest8::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2025,
        8,
        Part::Two,
        event2025::quest8::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2025,
        8,
        Part::Three,
        event2025::quest8::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2025,
        9,
        Part::One,
        event2025::quest9::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2025,
        9,
        Part::Two,
        event2025::quest9::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2025,
        9,
        Part::Three,
        event2025::quest9::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2025,
        10,
        Part::One,
        event2025::quest10::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2025,
        10,
        Part::Two,
        event2025::quest10::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2025,
        10,
        Part::Three,
        event2025::quest10::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2025,
        11,
        Part::One,
        event2025::quest11::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2025,
        11,
        Part::Two,
        event2025::quest11::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2025,
        11,
        Part::Three,
        event2025::quest11::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2025,
        12,
        Part::One,
        event2025::quest12::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2025,
        12,
        Part::Two,
        event2025::quest12::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2025,
        12,
        Part::Three,
        event2025::quest12::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2025,
        13,
        Part::One,
        event2025::quest13::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2025,
        13,
        Part::Two,
        event2025::quest13::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2025,
        13,
        Part::Three,
        event2025::quest13::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2025,
        14,
        Part::One,
        event2025::quest14::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2025,
        14,
        Part::Two,
        event2025::quest14::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2025,
        14,
        Part::Three,
        event2025::quest14::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2025,
        15,
        Part::One,
        event2025::quest15::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2025,
        15,
        Part::Two,
        event2025::quest15::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2025,
        15,
        Part::Three,
        event2025::quest15::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2025,
        16,
        Part::One,
        event2025::quest16::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2025,
        16,
        Part::Two,
        event2025::quest16::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2025,
        16,
        Part::Three,
        event2025::quest16::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2025,
        17,
        Part::One,
        event2025::quest17::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2025,
        17,
        Part::Two,
        event2025::quest17::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2025,
        17,
        Part::Three,
        event2025::quest17::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2025,
        18,
        Part::One,
        event2025::quest18::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2025,
        18,
        Part::Two,
        event2025::quest18::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2025,
        18,
        Part::Three,
        event2025::quest18::solve_part_3
    );
    try_use_solver!(
        puzzle_key,
        2025,
        19,
        Part::One,
        event2025::quest19::solve_part_1
    );
    try_use_solver!(
        puzzle_key,
        2025,
        19,
        Part::Two,
        event2025::quest19::solve_part_2
    );
    try_use_solver!(
        puzzle_key,
        2025,
        19,
        Part::Three,
        event2025::quest19::solve_part_3
    );
    panic!("solver not found for {:?}", puzzle_key);
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
                    let result = submit_with_cache(&key, solution.as_str(), |key, answer| {
                        if let Some(delay) = client.get_penalty_delay().unwrap() {
                            log::info!("sleeping for {:?} before submitting...", &delay);
                            thread::sleep(delay);
                        }
                        log::info!("submitting the answer to the server...");
                        client.post_answer(key, answer).unwrap()
                    });
                    if result.cached {
                        log::info!("submission result was provided by the cache in results.toml");
                    }
                    if result.is_answer_correct.unwrap() {
                        log::info!("✅ the answer is correct!");
                        if let Some(details) = result.details {
                            log::info!(
                                "time since event start: {}",
                                pretty_duration(&details.global_time, None)
                            );
                            log::info!(
                                "time since quest opened: {}",
                                pretty_duration(&details.local_time, None)
                            );
                            log::info!(
                                "global score {} (rank {})",
                                details.global_score,
                                details.global_place
                            );
                        }
                    } else {
                        log::info!("❌ the answer was NOT correct, try harder");
                        log::info!(
                            "the first letter of the answer was {}",
                            match result.is_first_character_correct {
                                None => "[no data]",
                                Some(true) => "✅️ correct",
                                Some(false) => "❌ not correct",
                            }
                        );
                        log::info!(
                            "the answer length was {}",
                            match result.is_length_correct {
                                None => "[no data]",
                                Some(true) => "✅️ correct",
                                Some(false) => "❌ not correct",
                            }
                        );
                    }
                }
            }
        }
        Err(e) => {
            log::error!("error retrieving puzzle input: {e:#?}");
        }
    }
}
