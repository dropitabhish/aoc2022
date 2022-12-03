// https://adventofcode.com/2022/day/2

use std::{collections::HashMap, fs::read_to_string, process::exit};

#[derive(Clone, Copy)]
enum RPSType {
    Rock,
    Paper,
    Scissors,
}
impl RPSType {
    // First part
    fn comp(&self, other: &RPSType) -> i32 {
        match self {
            RPSType::Rock => match other {
                RPSType::Rock => 3,
                RPSType::Paper => 0,
                RPSType::Scissors => 6,
            },
            RPSType::Paper => match other {
                RPSType::Rock => 6,
                RPSType::Paper => 3,
                RPSType::Scissors => 0,
            },
            RPSType::Scissors => match other {
                RPSType::Rock => 0,
                RPSType::Paper => 6,
                RPSType::Scissors => 3,
            },
        }
    }

    // Second part
    fn type_from_outcome(&self, alphabet: &str) -> RPSType {
        match alphabet {
            "X" => match self {
                RPSType::Rock => RPSType::Scissors,
                RPSType::Paper => RPSType::Rock,
                RPSType::Scissors => RPSType::Paper,
            },
            "Y" => match self {
                RPSType::Rock => RPSType::Rock,
                RPSType::Paper => RPSType::Paper,
                RPSType::Scissors => RPSType::Scissors,
            },
            "Z" => match self {
                RPSType::Rock => RPSType::Paper,
                RPSType::Paper => RPSType::Scissors,
                RPSType::Scissors => RPSType::Rock,
            },
            _ => panic!("Invalid alphabet"),
        }
    }
}

fn main() {
    let read_input = read_to_string("src/input.txt");
    let read_input = read_input.unwrap_or_else(|_| {
        println!("{:?}", "File not found/ Unable to read".to_string());
        exit(0)
    });

    let rps_value = HashMap::from([
        ("A", RPSType::Rock),
        ("B", RPSType::Paper),
        ("C", RPSType::Scissors),
        ("X", RPSType::Rock),
        ("Y", RPSType::Paper),
        ("Z", RPSType::Scissors),
    ]);

    let input = read_input.trim().split('\n');

    let mut total = 0;
    let mut total2 = 0;

    for inp in input {
        let mut inp_char = inp.split(' ');
        let first = inp_char.next().unwrap();
        let second = inp_char.next().unwrap();

        let first_type = rps_value[first];
        let second_type = rps_value[second];

        let isecond = match second_type {
            RPSType::Rock => 1,
            RPSType::Paper => 2,
            RPSType::Scissors => 3,
        };

        let part2_type = match second {
            "X" => first_type.type_from_outcome("X"),
            "Y" => first_type.type_from_outcome("Y"),
            "Z" => first_type.type_from_outcome("Z"),
            _ => unreachable!(),
        };

        let ipart2 = match part2_type {
            RPSType::Rock => 1,
            RPSType::Paper => 2,
            RPSType::Scissors => 3,
        };

        total += isecond + second_type.comp(&first_type);
        total2 += ipart2 + part2_type.comp(&first_type);
    }

    println!("{:?}", total);
    println!("{:?}", total2);
}
