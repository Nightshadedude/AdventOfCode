use std::fs;
use std::str::FromStr;

#[derive(Debug)]
enum Dir {
    Forward,
    Backward,
    Up,
    Down,
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(input: &str) -> Result<Dir, Self::Err> {
        match input {
            "forward" => Ok(Dir::Forward),
            "backward" => Ok(Dir::Backward),
            "up"  => Ok(Dir::Up),
            "down" => Ok(Dir::Down),
            _      => Err(()),
        }
    }
}

#[derive(Debug)]
struct Movement {
    dir: Dir,
    dist: i32,
}

impl Movement {
    fn parse_line(line: &str) -> Self{
        let spl: Vec<&str> = line.split(" ").collect();
        Self{
            dir: Dir::from_str(spl[0]).expect("oops"),
            dist: spl[1].parse::<i32>().expect("another oops"),
        }
    }
}

fn calc(input: Vec<Movement>) -> i32 {
    let mut horiz = 0i32;
    let mut depth = 0i32;
    let mut aim = 0i32;
    for m in input {
        match m.dir {
            Dir::Forward => {horiz += m.dist;
                depth += (aim * m.dist);
            },
            Dir::Backward => {panic!("back")},
            Dir::Up => {aim -= m.dist},
            Dir::Down =>{aim += m.dist},
            _ => {panic!("uhh")},
        }
    }
    horiz * depth
}

fn main() {
    let file = "input";
    let data: Vec<Movement> = fs::read_to_string(file).unwrap().lines().map(|x| x.to_string()).collect::<Vec<String>>().iter().map(|s| Movement::parse_line(s)).collect();
    println!("{:?}", calc(data));
}
