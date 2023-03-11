#![feature(iter_array_chunks)]

use clap::{Parser, ValueEnum};
use color_eyre::{eyre::Context, Result};
use std::fs::{create_dir_all, read, write};

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

    let package_name = env!("CARGO_PKG_NAME");
    let project_dir =
        directories::ProjectDirs::from("", "AndreasKargSoftware", package_name).unwrap();
    let cache_dir = project_dir.cache_dir();
    let session_cache_dir = cache_dir.join(&args.session_id);
    let cache_file_name = format!("day_{}.txt", args.advent_day);
    let cache_file_path = session_cache_dir.join(cache_file_name);

    let input = if cache_file_path.exists() {
        let stringified_path = cache_file_path.to_str().unwrap().to_owned();
        println!("Using cached inputs from {}...", stringified_path);
        let input = read(cache_file_path)
            .with_context(|| format!("Unable to read cache file at {stringified_path}!"))?;
        String::from_utf8(input).with_context(|| format!("Invalid UTF-8 in {stringified_path}!"))?
    } else {
        println!("No cached input found. Downloading fresh copy...");
        let response_body = input_fetcher::fetch(args.advent_day, &args.session_id);

        create_dir_all(cache_dir)?;
        write(cache_file_path, &response_body)?;

        response_body
    };

    let solver = match (args.advent_day, args.part) {
        (1, Part::One) => days::day_1::solve_part_1,
        (1, Part::Two) => days::day_1::solve_part_2,
        (2, Part::One) => days::day_2::solve_part_1,
        (2, Part::Two) => days::day_2::solve_part_2,
        (3, Part::One) => days::day_3::solve_part_1,
        (3, Part::Two) => days::day_3::solve_part_2,
        (4, Part::One) => days::day_4::solve_part_1,
        (4, Part::Two) => days::day_4::solve_part_2,
        (5, Part::One) => days::day_5::solve_part_1,
        (5, Part::Two) => days::day_5::solve_part_2,
        _ => panic!("Unknown combo of advent day and puzzle part."),
    };

    let solution = solver(&input);

    println!("The puzzle solution is:\n{solution}");

    Ok(())
}
