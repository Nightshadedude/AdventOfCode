use std::{collections::HashMap, fs};

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
        let num_answers = self.answers.split_terminator("\n").collect::<Vec<&str>>().len();
        let mut count = HashMap::new();
        self.answers.chars().for_each(|c| {count.insert(c, self.answers.matches(c).count());});
        count.remove(&'\n');
        let mut total = 0usize;
        for (_,v) in &count {
            if *v == num_answers { total += 1; }
        }
//        println!("str: {}\nhashmap: {:#?}\nnum: {}\ntotal: {}\n\n", self.answers, &count, num_answers, total);
        total
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
