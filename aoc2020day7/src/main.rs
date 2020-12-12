use std::{collections::HashSet, fs};

struct Node<T>
where
    T: PartialEq,
{
    index: usize,
    value: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(index: usize, value: T) -> Self {
        Self{
            index,
            value,
            parent: None,
            children: vec![],
        }
    }
}

#[derive(Default)]
struct ArenaTree<T>
where
    T: PartialEq,
{
    arena: Vec<Node<T>>,
}

impl <T> ArenaTree<T>
where
    T: PartialEq,
{
    fn node(&mut self, value: T) -> usize {
        for node in &self.arena {
            if node.value == value {
                return node.index;
            }
        }

        let index = self.arena.len();
        self.arena.push(Node::new(index, value));
        index
    }

    fn size(&self) -> usize {
        self.arena.len()
    }

    fn edges(&self) -> usize {
        self.arena.iter()
            .fold(0, |a, n| a + n.children.len())
    }

    fn depth(&self, idx: usize) -> usize {
        match self.arena[idx].parent {
            Some(id) => self.depth(id) + 1,
            None => 0,
        }
    }

    fn depth_of_target(&self, index: usize, target: &T) -> Option<usize> {
        if target == &self.arena[index].value {
            return Some(0);
        }

        for p in &self.arena[index].children {
            if let Some(x) = self.depth_of_target(*p, &target) {
                return Some( x + 1);
            }
        }

        None
    }

    fn distance_between(&mut self, from: T, target: T) -> usize {
        let start_node = self.node(from);
        let mut ret = 0;
        let mut traversal = &self.arena[start_node];
        while let Some(inner) = traversal.parent {
            if let Some(x) = self.depth_of_target(inner, &target) {
                ret += x;
                break;
            }
            traversal = &self.arena[inner];
            ret += 1;
        }
        if ret > 0 {
            ret - 1
        } else {
            ret
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct Bag{
    adj: String,
    color: String,
}

impl Bag {
    fn to_string(&self) -> String {
        format!("{} {}", self.adj, self.color)
    }

    fn s_node_to_bag(s: &str) -> Self {
        let spl = s.split(" ").collect::<Vec<&str>>();
        Bag {
            adj: spl[0].to_string(),
            color: spl[0].to_string(),
        }
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
    let mut tree: ArenaTree<Bag> = ArenaTree::default();
    let mut all_nodes: HashSet<String> = HashSet::new();
    let filename = "input";
    let contents = fs::read_to_string(filename).expect("failed to read");
    let line_of_data = contents.split_terminator("\n").collect::<Vec<&str>>();
    for line in line_of_data {
        println!("Parsing: {}", line);
        let temp_parent = tree.node(parse_bag(line).0);
        all_nodes.insert(parse_bag(line).0.to_string());
        println!("--Parent: {}", parse_bag(line).0.to_string());
        match parse_bag(line).1 {
            Some(bags) => {
                for bag in bags {
                    let temp_child = tree.node(bag.clone());
                    all_nodes.insert(bag.to_string());
                    println!("----Child: {}", bag.to_string());
                    tree.arena[temp_parent].children.push(temp_child);
                    tree.arena[temp_child].parent = Some(temp_parent);
                }
            },
            None => (),
        }
    }
    
    let mut total = 0;
    for s_node in all_nodes {
        let bag_to_find = Bag {
            adj: "shiny".to_string(),
            color: "gold".to_string(),
        };
        let bag_to_check = Bag::s_node_to_bag(&s_node);
        match tree.distance_between(bag_to_find, bag_to_check) {
            0 => (),
            _ => total += 1,
        }
    }

    println!("{}", total);
}
