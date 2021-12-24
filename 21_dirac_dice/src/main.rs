use clap::{App, Arg};
use regex::Regex;
use std::fs;

use dirac_dice::{play_deterministic, play_dirac};

fn main() {
    let part_validate = |s| {
        if s == "1" || s == "2" {
            return Ok(());
        }
        return Err(String::from("part must be either 1 or 2"));
    };

    let matches = App::new("Dirac Dice")
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
    let players = parse_input(&contents);
    println!("{}", play_deterministic(players));
}

fn part_two(file: &str) {
    let contents = fs::read_to_string(file).unwrap();
    let players = parse_input(&contents);
    
    let wins = play_dirac(players[0], players[1]);
    let result = wins.iter().map(|(_player, wins)| wins).max().unwrap();
    println!("{}", result);
}

fn parse_input(input: &str) -> Vec<u32> {
    let regex = Regex::new(r"Player [0-9]+ starting position: (?P<position>[0-9]+)").unwrap();
    regex.captures_iter(input).map(|c| {
        u32::from_str_radix(c.name("position").unwrap().as_str(), 10).unwrap()
    }).collect()
}
