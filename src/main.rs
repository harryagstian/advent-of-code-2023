use clap::{Arg, ArgMatches, Command};
use color_eyre::eyre::Result;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod solver;

fn init() -> Result<ArgMatches> {
    color_eyre::install()?;

    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::INFO)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let matches = Command::new("Advent of Code 2023")
        .version("1.0")
        .author("Harry Agustian <https://harryagustian.xyz>")
        .about("Solution for Advent of Code 2023 in Rust")
        .arg(Arg::new("day").required(true).help("Day to solve"))
        .get_matches();

    Ok(matches)
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = init()?;

    let day = matches.get_one::<String>("day").unwrap().parse::<i32>()?;

    let mut solver = solver::Solver::new(day).await?;
    solver.solve().await?;
    solver.print_answer();

    Ok(())
}
