use clap::{App, Arg};
// use regex::Regex;
use std::fs;

use seven_segment_search::{SegmentDecoder};

fn main() {
    let part_validate = |s| {
        if s == "1" || s == "2" {
            return Ok(());
        }
        return Err(String::from("part must be either 1 or 2"));
    };

    let matches = App::new("Seven Segment Search")
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

    // count 1, 4, 7, 8 in output
    // count outputs with length 2, 3, 4, or 7
    let unique_count: u32 = contents.lines()
        .map(|line| line.split('|').nth(1).unwrap().trim())
        .map(|output| output.split_whitespace().filter_map(|digit| {
            match digit.len() {
                2 | 3 | 4 | 7 => Some(1),
                _ => None
            }
        }).sum::<u32>())
        .sum();

    println!("{}", unique_count);
}

fn part_two(file: &str) {
    let contents = fs::read_to_string(file).unwrap();

    let result: u32 = contents.lines().map(|line| {
        let mut inputs = line.split('|');
        let uniques: Vec<&str> = inputs.next().unwrap().trim().split_whitespace().collect();
        let outputs: Vec<&str> = inputs.next().unwrap().trim().split_whitespace().collect();

        let decoder = SegmentDecoder::from(&uniques);
        let result: u32 = outputs.iter().rev()
            .enumerate().map(|(i, out)| decoder.decode(out).unwrap() as u32 * (10 as u32).pow(i as u32)).sum();

        result
    }).sum();

    println!("{}", result);
}
