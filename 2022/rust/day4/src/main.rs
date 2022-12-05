use std::fs;

fn read_file(name: &str) -> String {
    return fs::read_to_string(name)
    .expect("Should have been able to read the file");
}

struct Pair{
    first_low: i32,
    first_high: i32,
    second_low: i32,
    second_high: i32,
}

impl Pair{
    fn new(s: &str) -> Self {
        let split = s.split(",").collect::<Vec<&str>>();
        let first = split[0].split("-").collect::<Vec<&str>>();
        let second = split[1].split("-").collect::<Vec<&str>>();
        Self {
            first_low: first[0].parse::<i32>().unwrap(),
            first_high: first[1].parse::<i32>().unwrap(),
            second_low: second[0].parse::<i32>().unwrap(),
            second_high: second[1].parse::<i32>().unwrap(),
        }
    }

    fn fully_contains(&self) -> i32 {
        if self.first_low >= self.second_low && self.first_high <= self.second_high {
            return 1;
        }
        else if self.second_low >= self.first_low && self.second_high <= self.first_high {
            return 1;
        }
        else {
            return 0;
        }
    }
}


fn main() {
    let input = read_file("input");
    let part_1 = input.lines().map(|s| Pair::new(s).fully_contains()).sum::<i32>();
    println!("{}", part_1);
}
