use std::fs;

#[derive(Debug)]
struct Rucksack {
    contents: String,
    priority: String,
    value: usize
}

#[derive(Debug)]
struct Rucksacks {
    sacks: Vec<Rucksack>,
}

impl Rucksack {
    fn new(s: &str) -> Self {
        let priority = Self::get_priority(s.to_string());
        let value = Self::get_value(&priority);
        Self {
            contents: s.to_string(),
            priority: priority,
            value: value,
        }
    }

    fn get_priority(s: String) -> String {
        let len = s.len();
        let mut priority = '\0';
        let (c1, c2) = s.split_at(len/2);
        for c in c1.chars(){
            for cc in c2.chars(){
                if c == cc{
                    priority = c;
                    break;
                }
            }
            if priority == c {break;}
        }
        return priority.to_string()
    }

    fn get_value(s: &str) -> usize {
        let val = s.chars().next().unwrap() as usize;
        let mut ret = 0;
        if val >= 65 && val < 91 { ret =  val - 38; }
        else if val >= 97 { ret =  val - 96; }
        else { ret =  0; }
        ret
    }
}

impl Rucksacks {
    fn new(v: Vec<Rucksack>) -> Self {
        Self{
            sacks: v,
        }
    }

    fn chunk(&self) -> usize {
        self.sacks.chunks(3).map(|c| Rucksack::get_value(&Self::get_priority(c))).sum::<usize>()
    }

    fn get_priority(c: &[Rucksack]) -> String {
        let c0 = &c[0].contents;
        let c1 = &c[1].contents;
        let c2 = &c[2].contents;
        let mut priority = '\0';
        for c in c0.chars(){
            for cc in c1.chars(){
                if c == cc {
                    for ccc in c2.chars(){
                        if c == ccc{
                                priority = c;
                                break;
                        }
                    }
                }
                if priority == c {break;}
            }
            if priority == c {break;}
        }
        return priority.to_string();
    }
}


fn read_file(name: &str) -> String {
    return fs::read_to_string(name)
    .expect("Should have been able to read the file");
}

fn main() {
    let input = read_file("input");
    let part_1 = input.lines().map(|s| Rucksack::new(s).value).sum::<usize>();
    println!("{}", part_1);

    let sacks = Rucksacks::new(input.lines().map(|s| Rucksack::new(s)).collect::<Vec<Rucksack>>());
    let part_2 = sacks.chunk();
    println!("{}", part_2);   
}