use clap::{App, Arg};
use regex::Regex;
use std::fs;
use std::ops::RangeInclusive;

use trick_shot::{Target, max_y, all_starts};

fn main() {
    let part_validate = |s| {
        if s == "1" || s == "2" {
            return Ok(());
        }
        return Err(String::from("part must be either 1 or 2"));
    };

    let matches = App::new("Trick Shot")
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
    let (x, y) = parse_bounds(&contents);
    let target = Target::new(x, y);

    println!("{}", max_y(target));
}

fn part_two(file: &str) {
    let contents = fs::read_to_string(file).unwrap();
    let (x, y) = parse_bounds(&contents);
    let target = Target::new(x, y);
    let starts = all_starts(target);

    println!("{}", starts.len());
}

fn parse_bounds(target: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let regex = Regex::new(r"-?[0-9]+\.\.-?[0-9]+").unwrap();
    let matches = regex.find_iter(target);
    let mut bounds = matches.map(|m| {
        let mut b = m.as_str().split("..").map(|s| s.parse::<i32>().unwrap());
        let b = b.next().unwrap()..=b.next().unwrap();
        b
    });
    (bounds.next().unwrap(), bounds.next().unwrap())
}
