use clap::{Parser, ValueEnum};
use color_eyre::Result;

mod days;
mod input_fetcher;

#[derive(ValueEnum, Clone, PartialEq, Eq, Debug)]
enum Part {
    One,
    Two,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The advent day to solve
    advent_day: u8,

    /// Which of the two tasks of the day to solve
    #[arg(value_enum)]
    part: Part,

    /// The value of the "session" cookie on the aoc website
    #[arg(short, long, env = "AOC_SESSION_ID")]
    session_id: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let response_body = input_fetcher::fetch(args.advent_day, &args.session_id);

    let solution = match (args.advent_day, args.part) {
        (1, Part::One) => days::day_1_1::solve(&response_body),
        _ => panic!("Unknown combo of advent day and puzzle part."),
    };

    println!("The puzzle solution is:\n{solution}");

    Ok(())
}
