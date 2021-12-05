use clap::{App, Arg};
use regex::Regex;
use std::fs;

use giant_squid::{BingoBoard, MarkResult};

fn main() {
    let part_validate = |s| {
        if s == "1" || s == "2" {
            return Ok(());
        }
        return Err(String::from("part must be either 1 or 2"));
    };

    let matches = App::new("Giant Squid")
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

    let regex = Regex::new(r"(?m)^(?:(?:(?:\s+)?[0-9]+(?:\s+)?){5}){5}$").unwrap();
    let mut boards: Vec<BingoBoard> = regex.find_iter(&contents).map(|board| {
        let nums = parse_nums(board.as_str());
        BingoBoard::new(&nums)
    }).collect();

    let to_mark: Vec<u32> = contents.lines().next().unwrap()
        .split(',').map(|num| num.parse::<u32>().unwrap()).collect();

    'outer: for mark in to_mark {
        for board in boards.iter_mut() {
            if MarkResult::Bingo == board.mark(mark) {
                let unmarked_sum: u32 = board.unmarked().iter().sum();
                let result = unmarked_sum * mark;
                println!("{}", result);
                break 'outer;
            }
        }
    }
}

fn part_two(file: &str) {
    let contents = fs::read_to_string(file).unwrap();

    let regex = Regex::new(r"(?m)^(?:(?:(?:\s+)?[0-9]+(?:\s+)?){5}){5}$").unwrap();
    let mut boards: Vec<BingoBoard> = regex.find_iter(&contents).map(|board| {
        let nums = parse_nums(board.as_str());
        BingoBoard::new(&nums)
    }).collect();

    let to_mark: Vec<u32> = contents.lines().next().unwrap()
        .split(',').map(|num| num.parse::<u32>().unwrap()).collect();

    let mut last_bingo_unmarked = None;
    let mut last_bingo_num = None;

    for mark in to_mark {
        for board in boards.iter_mut() {
            if !board.bingo() && MarkResult::Bingo == board.mark(mark) {
                last_bingo_unmarked = Some(board.unmarked());
                last_bingo_num = Some(mark);
            }
        }
    }

    let unmarked_sum: u32 = last_bingo_unmarked.unwrap().iter().sum();
    let result: u32 = unmarked_sum * last_bingo_num.unwrap();
    println!("{}", result);
}

fn parse_nums(board: &str) -> Vec<u32> {
    let mut nums = Vec::new();

    for line in board.lines() {
        let thing: Vec<u32> = line
            .split_whitespace()
            .filter_map(|num| {
                if let Ok(num) = num.parse::<u32>() {
                    Some(num)
                } else { None }
            })
            .collect();
        nums = [nums, thing].concat();
    }
    nums
}