use std::fs;
use std::str::FromStr;

fn read_file(name: &str) -> String {
    return fs::read_to_string(name)
    .expect("Should have been able to read the file");
}

#[derive(Debug)]
enum Throw {
    Rock,
    Paper,
    Scissors,
    Error
}


fn throw_value(throw: &Throw) -> i32 {
    match throw {
        &Throw::Rock => 1,
        &Throw::Paper => 2,
        &Throw::Scissors => 3,
        _ => 0,
    }
}

fn str_to_throw(input: &str) -> Throw {
    match input {
        "A" => Throw::Rock,
        "B" => Throw::Paper,
        "C" => Throw::Scissors,
        "X" => Throw::Rock,
        "Y" => Throw::Paper,
        "Z" => Throw::Scissors,
        _ => Throw::Error,
    }
}


fn compare(throws: Vec<Throw>) -> i32 {
    let first = &throws[1];
    let second = &throws[0];
    match first {
        &Throw::Rock => {
            match second {
                &Throw::Rock => 3,
                &Throw::Paper => 0,
                &Throw::Scissors => 6,
                _ => 0,
            }
        },
        &Throw::Paper => {
            match second {
                &Throw::Rock => 6,
                &Throw::Paper => 3,
                &Throw::Scissors => 0,
                _ => 0,
            }
        },
        &Throw::Scissors => {
            match second {
                &Throw::Rock => 0,
                &Throw::Paper => 6,
                &Throw::Scissors => 3,
                _ => 0,
            }
        },
        _ => { 0 },
    }
}

fn clean(input: &str) -> i32 {
    let split = input.split(" ").map(|s| str_to_throw(s)).collect::<Vec<Throw>>();
    let mut sum = throw_value(&split[1]);
    sum += compare(split);
    sum
}

fn main() {
    let input = read_file("input");
    let throws = input.lines().map(|l| clean(l)).collect::<Vec<i32>>();
    println!("Part 1: {:?}", throws.iter().sum::<i32>());
}