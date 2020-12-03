use std::fs;

pub struct Policy {
    pos1: u16,
    pos2: u16,
    ch: char,
    pass: String,
}

impl Policy {
    fn new(unparsed: &str) -> Self {
        let partial_parse = unparsed.split_whitespace().collect::<Vec<&str>>();
        let temp = partial_parse[0].split("-").collect::<Vec<&str>>();
        Policy {
            pos1: temp[0].parse::<u16>().unwrap_or(0),
            pos2: temp[1].parse::<u16>().unwrap_or(0),
            ch: partial_parse[1].chars().next().unwrap_or(' '),
            pass: String::from(partial_parse[2]),
        }
    }

    fn is_valid(&self) -> bool {
        let p1 = self.pass.chars().nth(self.pos1 as usize-1).unwrap_or(' ') == self.ch;
        let p2 = self.pass.chars().nth(self.pos2 as usize-1).unwrap_or(' ') == self.ch;

        return p1 ^ p2;
    }
}

fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename).expect("failed to read");
    let strs = contents.split_terminator("\n").collect::<Vec<&str>>();
    let pol_check = strs.iter().map(|s| Policy::new(s)).collect::<Vec<Policy>>();
    println!(
        "{}",
        pol_check
            .iter()
            .map(|pol| Policy::is_valid(pol) as u16)
            .sum::<u16>()
    );
}
