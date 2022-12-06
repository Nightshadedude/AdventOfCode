use std::fs;
use std::collections::HashSet;

fn read_file(name: &str) -> String {
    return fs::read_to_string(name)
    .expect("Should have been able to read the file");
}

struct Comm {
    characters: Vec<String>,
}

impl Comm {
    fn new(v: Vec<String>) -> Self {
        Self {
            characters: v,
        }
    }

    fn report(&mut self) -> usize {
        let window_size = 4;
        let mut chars = 0usize;
        for (ii,w) in self.characters.windows(window_size).enumerate() {
            let mut set = HashSet::new();
            for s in w.clone().iter() {
                set.insert(s);
            }
            if set.len() == window_size {
                chars = ii;
                break;
            }
        }
        chars + window_size
    }
}

fn main() {
    let mut input = read_file("input");
    input.trim();
    let split = input.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    let mut comm = Comm::new(split);
    println!("{}", comm.report());
}
