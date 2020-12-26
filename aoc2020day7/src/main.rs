use indextree::Arena;
use std::fs;
use std::collections::HashSet;

struct Bag{
//    parent: Option<Box<Bag>,
    adj: String,
    color: String,
//    children: Option<Vec<Box<Bag>>>,
}

impl Bag {
    fn to_string(&self) -> String {
        format!("{} {}", self.adj, self.color)
    }

    fn str_to_bag(s: &str) -> Self {
        let spl = s.split(" ").collect::<Vec<&str>>();
        Bag {
            adj: spl[0].to_string(),
            color: spl[0].to_string(),
        }
    }

    fn bag_check(&self, other_bag: &Bag) -> bool {
        self.color == other_bag.color &&
        self.adj == other_bag.adj
    }
}

fn parse_bag(s: &str) -> (Bag, Option<Vec<Bag>>) {
    let parent_child = s.split(" bags contain ").collect::<Vec<&str>>();
    let parent = parent_child[0].split(" ").collect::<Vec<&str>>();
    let p_bag = Bag {
        adj: parent[0].to_string(),
        color: parent[1].to_string(),
    };
    let children = parent_child[1].split(", ").collect::<Vec<&str>>();
    let mut c_bag: Vec<Bag> = vec![];
    for s in children {
        let temp = s.split(" ").collect::<Vec<&str>>();
        match temp[0] {
            "no" => (),
            _ if temp.len() > 3 => {
                c_bag.push(Bag{
                    adj: temp[1].to_string(),
                    color: temp[2].into(),
                });
            },
            _ => (),
        }
    }
    
    match c_bag.len(){
        0 => (p_bag, None),
        _ => (p_bag, Some(c_bag)),
    }
}


fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename).expect("failed to read");
    let line_of_data = contents.split_terminator("\n").collect::<Vec<&str>>();
    let mut bags: HashSet<(Bag, Option<Vec<Bag>>)> = HashSet::new();
    for line in line_of_data {
        println!("Parsing: {}", line);
        bags.insert(parse_bag(line));
    }

    let arena = &mut Arena::new();

    for b in bags {
        let p_node = arena.new_node(b.0);
        match b.1 {
            Some(c_bag) => {
                for cb in c_bag { 
                    p_node.append(arena.new_node(cb), arena);
                }
            },
            None => {
                println!("Parent node is empty");
            },
        }
    }
    println!("{:#?}", arena);
}
