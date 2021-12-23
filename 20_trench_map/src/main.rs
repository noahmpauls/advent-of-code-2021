use clap::{App, Arg};
// use regex::Regex;
use std::fs;

use trench_map::ImageEnhancer;

fn main() {
    let part_validate = |s| {
        if s == "1" || s == "2" {
            return Ok(());
        }
        return Err(String::from("part must be either 1 or 2"));
    };

    let matches = App::new("Trench Map")
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
    let (enhancer, pixels) = parse_input(&contents);

    let mut trench_map = ImageEnhancer::new(&pixels, enhancer);
    (0..2).for_each(|_| trench_map.enhance());
    let lit_count = match trench_map.lit_count() {
        Some(count) => count.to_string(),
        None => String::from("infinity"),
    };
    println!("{}", lit_count);
}

fn part_two(file: &str) {
    let contents = fs::read_to_string(file).unwrap();
    let (enhancer, pixels) = parse_input(&contents);
    
    let mut trench_map = ImageEnhancer::new(&pixels, enhancer);
    (0..50).for_each(|_| trench_map.enhance());
    let lit_count = match trench_map.lit_count() {
        Some(count) => count.to_string(),
        None => String::from("infinity"),
    };
    println!("{}", lit_count);
}

fn parse_input(input: &str) -> (&str, Vec<&str>) {
    let enhancer = input.lines().next().unwrap();

    let pixels = input.lines().skip(2).collect();

    (enhancer, pixels)
}
