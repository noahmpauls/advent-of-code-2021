use clap::{App, Arg};
// use regex::Regex;
use std::fs;

use beacon_scanner::{Scanner, ScannerSet, Beacon};

fn main() {
    let part_validate = |s| {
        if s == "1" || s == "2" {
            return Ok(());
        }
        return Err(String::from("part must be either 1 or 2"));
    };

    let matches = App::new("Beacon Scanner")
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
    let scanners = parse_input(&contents);
    let scanner_set = ScannerSet::assemble(scanners, 12);

    let result = scanner_set.unqiue_beacons().len();
    println!("{}", result);
}

fn part_two(file: &str) {
    let contents = fs::read_to_string(file).unwrap();
    let scanners = parse_input(&contents);
    let scanner_set = ScannerSet::assemble(scanners, 12);

    let result = scanner_set.max_manhattan();
    println!("{}", result);
}

fn parse_input(input: &str) -> Vec<Scanner> {
    let mut scanners = Vec::new();
    let mut current_beacons = Vec::new();

    for line in input.lines() {
        if line.starts_with("---") {
            current_beacons = Vec::new();
        } else if !line.is_empty() {
            let mut nums = line.split(',').map(|num| num.parse::<i32>().unwrap());
            current_beacons.push(Beacon {
                x: nums.next().unwrap(),
                y: nums.next().unwrap(),
                z: nums.next().unwrap(),
            })
        } else {
            scanners.push(Scanner::from(&current_beacons));
        }
    }
    if current_beacons.len() > 0 {
        scanners.push(Scanner::from(&current_beacons));
    }

    scanners
}
