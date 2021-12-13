use clap::{App, Arg};
use regex::Regex;
use std::fs;

use transparent_origami::{Dot,Axis,Fold,DottedPaper};

fn main() {
    let part_validate = |s| {
        if s == "1" || s == "2" {
            return Ok(());
        }
        return Err(String::from("part must be either 1 or 2"));
    };

    let matches = App::new("Transparent Origami")
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

    let dot_regex = Regex::new(r"[0-9]+,[0-9]+").unwrap();
    let dots = dot_regex.find_iter(&contents)
        .map(|m| {
            let mut nums = m.as_str().split(',').map(|n| n.parse::<i32>().unwrap());
            let x = nums.next().unwrap();
            let y = nums.next().unwrap();
            Dot { x, y }
        }).collect();

    let fold_regex = Regex::new(r"[xy]=[0-9]+").unwrap();
    let fold = fold_regex.find(&contents).unwrap();
    let fold = {
        let mut parts = fold.as_str().split('=');
        let axis = match parts.next().unwrap() {
            "x" => Axis::X,
            "y" => Axis::Y,
            _ => panic!("invalid axis"),
        };
        let line = parts.next().unwrap().parse::<i32>().unwrap();
        Fold { axis, line }
    };

    let mut paper = DottedPaper::new(&dots);
    paper.fold(&fold);

    println!("{}", paper.dot_count());
}

fn part_two(file: &str) {
    let contents = fs::read_to_string(file).unwrap();

    let dot_regex = Regex::new(r"[0-9]+,[0-9]+").unwrap();
    let dots = dot_regex.find_iter(&contents)
        .map(|m| {
            let mut nums = m.as_str().split(',').map(|n| n.parse::<i32>().unwrap());
            let x = nums.next().unwrap();
            let y = nums.next().unwrap();
            Dot { x, y }
        }).collect();

    let fold_regex = Regex::new(r"[xy]=[0-9]+").unwrap();
    let folds: Vec<Fold> = fold_regex.find_iter(&contents)
        .map(|m| {
            let mut parts = m.as_str().split('=');
            let axis = match parts.next().unwrap() {
                "x" => Axis::X,
                "y" => Axis::Y,
                _ => panic!("invalid axis"),
            };
            let line = parts.next().unwrap().parse::<i32>().unwrap();
            Fold { axis, line }
        }).collect();

    let mut paper = DottedPaper::new(&dots);
    for fold in folds {
        paper.fold(&fold);
    }

    println!("{}", paper);
}
