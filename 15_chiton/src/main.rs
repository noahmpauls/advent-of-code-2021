use clap::{App, Arg};
// use regex::Regex;
use std::fs;

use chiton::{Risk, RiskGrid};

fn main() {
    let part_validate = |s| {
        if s == "1" || s == "2" {
            return Ok(());
        }
        return Err(String::from("part must be either 1 or 2"));
    };

    let matches = App::new("Chiton")
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
    
    let row_size = contents.lines().next().unwrap().chars().count();
    let risk_grid = RiskGrid::from_list(
        &contents.lines().flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as Risk)).collect::<Vec<Risk>>()[..],
        row_size
    );

    let total_risk: u32 = risk_grid.safest_path()
        .iter().map(|c| risk_grid.get(c.row, c.col).unwrap() as u32)
        .skip(1).sum();

    println!("{}", total_risk);
}

fn part_two(file: &str) {
    let contents = fs::read_to_string(file).unwrap();

    let starting_grid: Vec<Vec<Risk>> = contents
        .lines().map(|row| {
            row.chars().map(|c| c.to_digit(10).unwrap() as Risk).collect()
        }).collect();
    
    let factor = 5;
    let full_grid = multiply_grid(starting_grid, factor);
    
    let row_size = full_grid[0].len();
    let full_list: Vec<Risk> = full_grid.iter().flat_map(|row| row.iter().copied()).collect();
    let risk_grid = RiskGrid::from_list(&full_list, row_size);
    let total_risk: u32 = risk_grid.safest_path()
        .iter().map(|c| risk_grid.get(c.row, c.col).unwrap() as u32)
        .skip(1).sum();

    println!("{}", total_risk);
}

fn multiply_grid(grid: Vec<Vec<Risk>>, factor: usize) -> Vec<Vec<Risk>> {
    let starting_rows = grid.len();
    let mut full_grid: Vec<Vec<Risk>> = Vec::new();

    for r in 0..factor {
        if full_grid.len() < (r + 1) * starting_rows {
            (0..starting_rows).for_each(|_| full_grid.push(Vec::new()));
        }
        for c in 0..factor {
            let add = r + c;
            let base = r * starting_rows;
            for (i, row) in grid.iter().enumerate() {
                let index = base + i;
                for risk in row {
                    let mut risk = add_clamp(*risk as usize, add, 1, 9);
                    if risk == 0 { risk += 1 }
                    full_grid[index].push(risk as u8);
                }
            }
        }
    }

    full_grid
}

fn add_clamp(start: usize, add: usize, min: usize, max: usize) -> usize {
    let modulus = max - min + 1;
    let result = (((start - min) + add) % modulus) + min;
    result
}
