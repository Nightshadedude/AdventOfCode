use std::fs;
use std::collections::HashSet;

struct Group {
    answers: String
}

impl Group {
    pub fn new(g: String) -> Self {
        Group{
            answers: g,
        }
    }

    pub fn count(&self) -> usize {
        let mut temp = HashSet::new();
        self.answers.split_terminator("\n").for_each(|a| a.chars().for_each(|c| {temp.insert(c);}));
        temp.len()
    }
}
fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename)
        .expect("failed to read")
        .split_terminator("\n\n")
        .map(|g| Group::new(g.to_string()).count())
        .sum::<usize>();
    println!("{}", contents);
}
