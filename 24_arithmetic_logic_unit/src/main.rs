use clap::{App, Arg};
// use regex::Regex;
use std::fs;

use arithmetic_logic_unit::{Program, monad_max, monad_min};

fn main() {
    let part_validate = |s| {
        if s == "1" || s == "2" {
            return Ok(());
        }
        return Err(String::from("part must be either 1 or 2"));
    };

    let matches = App::new("Arithmetic Logic Unit")
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

    let program = Program::parse(&contents);
    let result = monad_max(&program);

    println!("{}", result.unwrap());

}

fn part_two(file: &str) {
    let contents = fs::read_to_string(file).unwrap();

    let program = Program::parse(&contents);
    let result = monad_min(&program);

    println!("{}", result.unwrap());
}
