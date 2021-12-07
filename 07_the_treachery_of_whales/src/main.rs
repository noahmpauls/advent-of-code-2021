use clap::{App, Arg};
// use regex::Regex;
use std::fs;

fn main() {
    let part_validate = |s| {
        if s == "1" || s == "2" {
            return Ok(());
        }
        return Err(String::from("part must be either 1 or 2"));
    };

    let matches = App::new("The Treachery of Whales")
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
    let crabs: Vec<_> = contents.lines().next().unwrap()
        .split(',')
        .map(|num| num.parse::<i32>().unwrap())
        .collect();
    
    let max_crab = *crabs.iter().max().unwrap();
    let min_fuel: i32 = (0..=max_crab).map(|p| {
        crabs.iter().map(|v| (v - p).abs()).sum()
    }).min().unwrap();

    println!("{}", min_fuel);
}

fn part_two(file: &str) {
    let contents = fs::read_to_string(file).unwrap();
    let crabs: Vec<_> = contents.lines().next().unwrap()
        .split(',')
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

    let max_crab = *crabs.iter().max().unwrap();
    let min_fuel: i32 = (0..=max_crab).map(|p| {
        crabs.iter().map(|v| {
            let n = (v - p).abs();
            (n * (n + 1)) / 2
        }).sum()
    }).min().unwrap();

    println!("{}", min_fuel);
}
