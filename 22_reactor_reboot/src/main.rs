use clap::{App, Arg};
use regex::Regex;
use std::fs;

use reactor_reboot::{SimpleCuboid,SimpleReactor,ComplexCuboid,ComplexReactor};

fn main() {
    let part_validate = |s| {
        if s == "1" || s == "2" {
            return Ok(());
        }
        return Err(String::from("part must be either 1 or 2"));
    };

    let matches = App::new("Reactor Reboot")
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
    let mut reactor = SimpleReactor::new();
    let instructions = parse_input_simple(&contents);

    for i in instructions {
        if i.1.is_initialization() {
            match i.0 {
                "on" => reactor.turn_on(i.1),
                "off" => reactor.turn_off(i.1),
                _ => (),
            }
        }
    }

    println!("{}", reactor.on_count());
}

fn part_two(file: &str) {
    let contents = fs::read_to_string(file).unwrap();
    let mut reactor = ComplexReactor::new();
    let instructions = parse_input_complex(&contents);

    for i in instructions {
        match i.0 {
            "on" => reactor.turn_on(i.1),
            "off" => reactor.turn_off(i.1),
            _ => (),
        }
    }

    println!("{}", reactor.on_count());
}

fn parse_input_simple(input: &str) -> Vec<(&str, SimpleCuboid)> {
    let regex = Regex::new(r"-?[0-9]+").unwrap();
    input.lines().map(|line| {
        let instruction = if line.starts_with("on") {
            "on"
        } else {
            "off"
        };
        let mut num_iter = regex.find_iter(line);
        let ranges: Vec<_> = (0..3).map(|_| {
            num_iter.next().unwrap().as_str().parse::<i32>().unwrap()..=num_iter.next().unwrap().as_str().parse::<i32>().unwrap()
        }).collect();
        (instruction, SimpleCuboid::new(ranges[0].clone(), ranges[1].clone(), ranges[2].clone()))
    }).collect()
}

fn parse_input_complex(input: &str) -> Vec<(&str, ComplexCuboid)> {
    let regex = Regex::new(r"-?[0-9]+").unwrap();
    input.lines().map(|line| {
        let instruction = if line.starts_with("on") {
            "on"
        } else {
            "off"
        };
        let mut num_iter = regex.find_iter(line);
        let ranges: Vec<_> = (0..3).map(|_| {
            num_iter.next().unwrap().as_str().parse::<i32>().unwrap()..=num_iter.next().unwrap().as_str().parse::<i32>().unwrap()
        }).collect();
        (instruction, ComplexCuboid::from_ranges(ranges[0].clone(), ranges[1].clone(), ranges[2].clone()))
    }).collect()
}
