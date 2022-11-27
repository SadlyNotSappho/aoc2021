use std::fs::read_to_string;

use clap::{arg, command, Command};
fn cli() -> Command {
    command!()
        .arg(arg!([path] "Filepath for today's input")
            .required(true))
}

fn main() {
    let matches = cli().get_matches();

    let path = match matches.get_one::<String>("path") {
        Some(p) => p,
        None => panic!("No path arg")
    };

    let read = read_to_string(path).unwrap();

    aoc2021::day2::part1(read);
}
