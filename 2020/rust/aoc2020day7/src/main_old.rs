use std::fs;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Bag<'a> {
    parent: Option<Box<&'static Bag<'a>>>,
    adj: &'a str,
    color: &'a str,
    children: Option<Vec<Box<Bag<'a>>>>,
}

impl Bag <'_> {
    pub fn new<'a>() -> Self {
        Bag {
            parent: None,
            adj: "",
            color: "",
            children: None,
        }
    }
}

fn parse_bag(s: &str) -> Bag {
    let parent_child = s.split(" bags contain ").collect::<Vec<&str>>();
    let parent = parent_child[0].split(" ").collect::<Vec<&str>>();
    let mut p_bag = Bag::new();
    p_bag.adj = parent[0];
    p_bag.color = parent[1];

    let children = parent_child[1].split(", ").collect::<Vec<&str>>();
    let mut c_bag: Vec<Box<Bag>> = vec![];
    for s in children {
        let temp = s.split(" ").collect::<Vec<&str>>();
        match temp[0] {
            "no" => (),
            _ if temp.len() > 3 => {
                let mut temp_bag = Bag::new();
                temp_bag.parent = Some(Box::new(p_bag));
                temp_bag.adj = temp[1];
                temp_bag.color = temp[2];
                c_bag.push(Box::new(temp_bag));
            },
            _ => (),
        }
    }
    p_bag
}


fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename).expect("failed to read");
    let line_of_data = contents.split_terminator("\n").collect::<Vec<&str>>();
    let mut bags: HashSet<Bag> = HashSet::new();
    for line in line_of_data {
        println!("Parsing: {}", line);
        bags.insert(parse_bag(line));
    }
    for b in bags {
        println!("{:#?}", b);
    }
}
