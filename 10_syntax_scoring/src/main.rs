use clap::{App, Arg};
// use regex::Regex;
use std::fs;

use common::Counter;
use syntax_scoring::*;

fn main() {
    let part_validate = |s| {
        if s == "1" || s == "2" {
            return Ok(());
        }
        return Err(String::from("part must be either 1 or 2"));
    };

    let matches = App::new("Syntax Scoring")
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
    let mut counter = Counter::new();

    contents.lines()
        .filter_map(|chunk| {
            if let ParseResult::Corrupted { actual, .. } = parse_chunk(chunk) {
                Some(actual)
            } else { None }
        })
        .for_each(|char| counter.add(char));

    let result: u32 = counter.with_count_ge(1).iter()
        .map(|c| counter.count(**c).unwrap() * score_corrupted(**c))
        .sum();

    println!("{}", result);
}

fn part_two(file: &str) {
    let contents = fs::read_to_string(file).unwrap();

    let mut completions: Vec<u128> = contents.lines()
        .filter_map(|chunk| {
            if let ParseResult::Incomplete(completion) = parse_chunk(chunk) {
                Some(score_incomplete(&completion))
            } else { None }
        }).collect();

    completions.sort();

    let result = completions[completions.len() / 2];

    println!("{}", result);
}
