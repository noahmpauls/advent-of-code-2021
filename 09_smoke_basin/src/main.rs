use clap::{App, Arg};
// use regex::Regex;
use std::fs;

use smoke_basin::{Height, Heightmap};

fn main() {
    let part_validate = |s| {
        if s == "1" || s == "2" {
            return Ok(());
        }
        return Err(String::from("part must be either 1 or 2"));
    };

    let matches = App::new("Smoke Basin")
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
    let heightmap = contents.lines().map(|line| {
        line.chars().map(|v| char::to_digit(v, 10).unwrap()).collect::<Vec<Height>>()
    }).collect();
    let heightmap = Heightmap::new(&heightmap);

    println!("{}", heightmap.risk_level());
}

fn part_two(file: &str) {
    let contents = fs::read_to_string(file).unwrap();
    let heightmap = contents.lines().map(|line| {
        line.chars().map(|v| char::to_digit(v, 10).unwrap()).collect::<Vec<Height>>()
    }).collect();
    let heightmap = Heightmap::new(&heightmap);

    let mut basins: Vec<usize> = heightmap.basins().iter().map(|b| b.len()).collect();
    basins.sort();

    let result: usize = basins.iter().copied().rev().take(3).reduce(|a, b| a * b).unwrap();

    println!("{}", result);
}
