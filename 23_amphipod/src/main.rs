use clap::{App, Arg};
// use regex::Regex;
use std::fs;

use amphipod::{Amphipod, Burrow};

fn main() {
    let part_validate = |s| {
        if s == "1" || s == "2" {
            return Ok(());
        }
        return Err(String::from("part must be either 1 or 2"));
    };

    let matches = App::new("Amphipod")
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

    let rooms: Vec<_> = parse_input(&contents);
    let burrow = Burrow::new(&rooms);

    let (_states, energy) = burrow.min_energy().unwrap();
    
    // println!("steps: {}", _states.len());

    // for (i, state) in _states.into_iter().enumerate() {
    //     println!("\nStep {}:\n{}", i, state);
    // }

    println!("{}", energy);
}

fn part_two(file: &str) {
    let contents = fs::read_to_string(file).unwrap();
    
    let rooms = parse_input(&contents);
    let extensions = vec![
        vec!['D', 'D',],
        vec!['C', 'B',],
        vec!['B', 'A',],
        vec!['A', 'C',],
    ];
    let rooms: Vec<_> = rooms.into_iter().zip(extensions.into_iter())
        .map(|(r, e)| vec![r[0], Amphipod::from_char(e[0]).unwrap(), Amphipod::from_char(e[1]).unwrap(), r[1]])
        .collect();

    let burrow = Burrow::new(&rooms);

    let (_states, energy) = burrow.min_energy().unwrap();
    
    // println!("steps: {}", _states.len());

    // for (i, state) in _states.into_iter().enumerate() {
    //     println!("\nStep {}:\n{}", i, state);
    // }

    println!("{}", energy);
}

fn parse_input(input: &str) -> Vec<Vec<Amphipod>> {
    // just get all the letters
    let amphipods: Vec<_> = input.chars()
        .filter_map(|c| Amphipod::from_char(c))
        .collect();
    let rooms = (0..4)
        .map(|i| vec![amphipods[i], amphipods[i + 4]])
        .collect();
    rooms
}
