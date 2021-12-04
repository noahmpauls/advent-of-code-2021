use clap::{App, Arg};
// use regex::Regex;
use std::fs;

use binary_diagnostic::{Counter, find_oxygen, find_co2};

fn main() {
    let part_validate = |s| {
        if s == "1" || s == "2" {
            return Ok(());
        }
        return Err(String::from("part must be either 1 or 2"));
    };

    let matches = App::new("Binary Diagnostic")
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
    let bit_count = contents.lines().next().unwrap().len();
    let mut counters: Vec<Counter<char>> = (0..bit_count).map(|_| Counter::new()).collect();

    // count bit occurrences at each index
    contents.lines().for_each(|line| {
        line.chars().enumerate().for_each(|(i, bit)| {
            counters[i].add(bit);
        });
    });

    // create gamma string from occurrence count
    let gamma: String = counters.iter().map(|counter| {
        if counter.count('0') > counter.count('1') {
            '0'
        } else {
            '1'
        }
    }).collect();

    // parse gamma string and create epsilon
    let gamma = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = (!gamma) & (u32::MAX >> (32 - bit_count));

    // println!("gamma: {:b}, epsilon: {:b}", gamma, epsilon);
    println!("{}", gamma * epsilon);
}

fn part_two(file: &str) {
    let contents = fs::read_to_string(file).unwrap();
    let bit_count = contents.lines().next().unwrap().len();
    let nums = contents.lines().map(|line| u16::from_str_radix(line, 2).unwrap()).collect();
    
    let oxygen = find_oxygen(&nums, bit_count);
    let co2 = find_co2(&nums, bit_count);

    // println!("oxygen: {:b}, co2: {:b}", oxygen, co2);
    println!("{}", oxygen * co2);
}
