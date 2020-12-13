use std::fs;
struct Bag{
    adj: String,
    color: String,
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

//blows the stack.  ooops
fn find_to_bag(to_search: &Vec<(Bag, Option<Vec<Bag>>)>, to_find: &Bag, mut start: usize) -> usize {
    let mut found = false;
    let len = to_search.len();
    let mut current = 0;
    while !found && current < len {
        let check = to_search[current].0.bag_check(to_find);
        if check {
            start += 1;
            found = true;
        } else {
            match &to_search[current].1 {
                None => (),
                Some(bags) => {
                    for c_bag in bags {
                        if c_bag.bag_check(to_find) {
                            start += find_to_bag(to_search, c_bag, start);
                        }
                    }
                },
            }
        }
        current += 1;
    }
    start
}


fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename).expect("failed to read");
    let line_of_data = contents.split_terminator("\n").collect::<Vec<&str>>();
    let mut bags: Vec<(Bag, Option<Vec<Bag>>)> = vec![];
    for line in line_of_data {
        println!("Parsing: {}", line);
        bags.push(parse_bag(line));
    }
    
    let to_find = Bag {
        adj: "shiny".to_string(),
        color: "gold".to_string(),
    };

    let total = find_to_bag(&bags, &to_find, 0);

    println!("{}", total);
}
