mod ecclient;
mod quest5;
mod types;

use std::env;

use clap::Parser;

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
    let client = ecclient::EcClient::new(cookie.as_str()).expect("creating EC client");
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
    log::info!("solving Everybody Codes event {} quest {} part {}", args.event, args.quest, args.part);
    log::info!("retrieving puzzle input...");
    match client.get_puzzle_input(&key) {
        Ok(input) => {
            println!("puzzle input: {}", input);
        }
        Err(e) => {
            log::error!("error retrieving puzzle input: {e:#?}");
        }
    }
}
