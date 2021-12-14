use clap::{App, Arg};
// use regex::Regex;
use std::fs;

use extended_polymerization::{PolyRule,PolyRuleSet,Counter};
use common::{char_windows};

fn main() {
    let part_validate = |s| {
        if s == "1" || s == "2" {
            return Ok(());
        }
        return Err(String::from("part must be either 1 or 2"));
    };

    let matches = App::new("Extended Polymerization")
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
    let template = contents.lines().next().unwrap();

    let polymerizer = PolyRuleSet::from(
        contents.lines().filter_map(|line| {
            if line.contains("->") {
                let mut rule = line.split("->");
                let rule = PolyRule::new(
                    rule.next().unwrap().trim(),
                    rule.next().unwrap().trim().chars().next().unwrap(),
                );
                Some(rule)
            } else {
                None
            }
        }).collect()
    );

    let mut polymer = String::from(template);
    for _ in 0..10 {
        polymer = polymerizer.polymerize(&polymer);
    }

    let mut counter = Counter::new();
    polymer.chars().for_each(|c| counter.add(c));

    let most_frequent = *counter.most_frequent().unwrap();
    let least_frequent = *counter.least_frequent().unwrap();

    println!("{}", counter.count(most_frequent).unwrap() - counter.count(least_frequent).unwrap());
}

fn part_two(file: &str) {
    let contents = fs::read_to_string(file).unwrap();
    let template = contents.lines().next().unwrap();

    let polymerizer = PolyRuleSet::from(
        contents.lines().filter_map(|line| {
            if line.contains("->") {
                let mut rule = line.split("->");
                let rule = PolyRule::new(
                    rule.next().unwrap().trim(),
                    rule.next().unwrap().trim().chars().next().unwrap(),
                );
                Some(rule)
            } else {
                None
            }
        }).collect()
    );

    let mut element_counter = Counter::new();
    template.chars().for_each(|c| element_counter.add(c));

    let mut pair_counter = Counter::new();
    char_windows(template, 2).for_each(|w| pair_counter.add(w));

    for _i in 0..40 {
        let mut new_counter = Counter::new();
        for (key, count) in pair_counter.iter() {
            polymerizer.polymer_pairs(key).unwrap().iter().for_each(|w| new_counter.add_count(*w, *count));
            element_counter.add_count(polymerizer.polymer(key).unwrap().chars().nth(1).unwrap(), *count);
        }
        pair_counter = new_counter;
    }

    let most_frequent = *element_counter.most_frequent().unwrap();
    let least_frequent = *element_counter.least_frequent().unwrap();

    println!("{}", element_counter.count(most_frequent).unwrap() - element_counter.count(least_frequent).unwrap());
}
