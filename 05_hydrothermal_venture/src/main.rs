use clap::{App, Arg};
use regex::Regex;
use std::fs;

use common::{Counter};
use hydrothermal_venture::{Coord,CompassLineSegment};

fn main() {
    let part_validate = |s| {
        if s == "1" || s == "2" {
            return Ok(());
        }
        return Err(String::from("part must be either 1 or 2"));
    };

    let matches = App::new("Hydrothermal Venture")
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

    // collect only horizontal/vertical lines
    let regex = Regex::new(r"[0-9]+").unwrap();
    let segments: Vec<CompassLineSegment> = contents.lines().filter_map(|line| {
        let nums: Vec<i32> = regex.find_iter(line)
            .map(|num| num.as_str().parse::<i32>().unwrap())
            .collect();
        assert_eq!(4, nums.len());
        let (c1, c2) = (Coord::new(nums[0], nums[1]), Coord::new(nums[2], nums[3]));
        let segment = CompassLineSegment::new(c1, c2);
        if segment.is_up_down() {
            Some(segment)
        } else {
            None
        }
    }).collect();

    let mut counter = Counter::new();
    for seg in &segments {
        for coord in seg.integer_coords() {
            counter.add(coord);
        }
    }

    let result = counter.with_count_ge(2).len();
    println!("{}", result);
}

fn part_two(file: &str) {
    let contents = fs::read_to_string(file).unwrap();

    // collect all lines
    let regex = Regex::new(r"[0-9]+").unwrap();
    let segments: Vec<CompassLineSegment> = contents.lines().map(|line| {
        let nums: Vec<i32> = regex.find_iter(line)
            .map(|num| num.as_str().parse::<i32>().unwrap())
            .collect();
        assert_eq!(4, nums.len());
        let (c1, c2) = (Coord::new(nums[0], nums[1]), Coord::new(nums[2], nums[3]));
        CompassLineSegment::new(c1, c2)
    }).collect();

    let mut counter = Counter::new();
    for seg in &segments {
        for coord in seg.integer_coords() {
            counter.add(coord);
        }
    }

    let result = counter.with_count_ge(2).len();
    println!("{}", result);
}
