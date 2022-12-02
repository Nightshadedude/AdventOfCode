use std::fs;

fn read_file(name: &str) -> String {
    return fs::read_to_string(name)
    .expect("Should have been able to read the file");
}

#[derive(Debug)]
enum Throw {
    Rock,
    Paper,
    Scissors,
    Error,
}

#[derive(Debug)]
enum Outcome {
    Win,
    Draw,
    Loss,
    Error,
}

#[derive(Debug)]
struct Game {
    throw: Throw,
    outcome: Outcome,
}

impl Game {
    fn new(v: Vec<&str>)-> Self {
        let throw = match v[0] {
            "A" => Throw::Rock,
            "B" => Throw::Paper,
            "C" => Throw::Scissors,
            _ => Throw::Error,
        };
        let outcome = match v[1] {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => Outcome::Error
        };

        Self {
            throw,
            outcome,
        }
    }

    fn compare(&self) -> i32 {
        match &self.outcome {
            &Outcome::Win => 6,
            &Outcome::Draw => 3,
            &Outcome::Loss => 0,
            _ => 0,
        }
    }

    fn throw_value(&self) -> i32 {
        match (&self.outcome, &self.throw) {
            (&Outcome::Win, &Throw::Rock) => 2,
            (&Outcome::Win, &Throw::Paper) => 3,
            (&Outcome::Win, &Throw::Scissors) => 1,
            (&Outcome::Draw, &Throw::Rock) => 1,
            (&Outcome::Draw, &Throw::Paper) => 2,
            (&Outcome::Draw, &Throw::Scissors) => 3,
            (&Outcome::Loss, &Throw::Rock) => 3,
            (&Outcome::Loss, &Throw::Paper) => 1,
            (&Outcome::Loss, &Throw::Scissors) => 2,
            _ => 0,
        }
    }

    fn total(&self) -> i32 {
        self.compare() + self.throw_value()
    }
}


fn parse(input: &str) -> i32 {
    let split = input.split(" ").collect::<Vec<&str>>();
    Game::new(split).total()
}

fn main() {
    let input = read_file("input");
    let throws = input.lines().map(|l| parse(l)).collect::<Vec<i32>>();
    println!("{}", throws.iter().sum::<i32>())
}