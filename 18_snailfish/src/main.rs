use clap::{App, Arg};
// use regex::Regex;
use std::fs;

use snailfish::SnailNum;

fn main() {
    let part_validate = |s| {
        if s == "1" || s == "2" {
            return Ok(());
        }
        return Err(String::from("part must be either 1 or 2"));
    };

    let matches = App::new("Snailfish")
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
    let sum = contents.lines()
        .map(|line| SnailNum::parse(line))
        .reduce(|a, b| a + b).unwrap()
        .magnitude();
    println!("{}", sum);
}

fn part_two(file: &str) {
    let contents = fs::read_to_string(file).unwrap();
    let snails: Vec<_> = contents.lines()
        .map(|line| SnailNum::parse(line))
        .collect();
    
    let mut max_mag = 0;
    for i in 0..snails.len() {
        for j in i..snails.len() {
            if i != j {
                max_mag = std::cmp::max(max_mag, (snails[i].clone() + snails[j].clone()).magnitude());
                max_mag = std::cmp::max(max_mag, (snails[j].clone() + snails[i].clone()).magnitude());
            }
        }
    }

    println!("{}", max_mag);
}
