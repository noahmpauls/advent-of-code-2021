use clap::{App, Arg};
// use regex::Regex;
use std::fs;

use dive::{Command, Position, Submarine};

fn main() {
    let part_validate = |s| {
        if s == "1" || s == "2" {
            return Ok(());
        }
        return Err(String::from("part must be either 1 or 2"));
    };

    let matches = App::new("Dive!")
        .arg(Arg::with_name("PART")
            .short("p")
            .long("part")
            .takes_value(true)
            .required(true)
            .validator(part_validate)
            .help("part of this day to run"))
        .arg(Arg::with_name("FILE")
            .short("f")
            .long("file")
            .takes_value(true)
            .required(true)
            .help("input file to take"))
        .get_matches();

    let part = matches.value_of("PART").unwrap().parse::<u8>().unwrap();
    let file = matches.value_of("FILE").unwrap();

    run(part, file);
}

fn run(part: u8, file: &str) {
    match part {
        1 => part_one(file),
        2 => part_two(file),
        _ => (),
    }
}

fn part_one(file: &str) {
    let contents = fs::read_to_string(file).unwrap();
    let sub_position = contents.lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let direction = split.next().unwrap();
            let magnitude = split.next().unwrap().parse::<i32>().unwrap();
            Position::from(Command::from(direction, magnitude))
        })
        .reduce(|a, b| a + b).unwrap();
    println!("{}", sub_position.x * sub_position.z);
}

fn part_two(file: &str) {
    let contents = fs::read_to_string(file).unwrap();
    let mut submarine = Submarine::new();
    contents.lines()
        .for_each(|line| {
            let mut split = line.split_whitespace();
            let direction = split.next().unwrap();
            let magnitude = split.next().unwrap().parse::<i32>().unwrap();
            submarine.command(Command::from(direction, magnitude));
        });
    println!("{}", submarine.position.x * submarine.position.z);
}
