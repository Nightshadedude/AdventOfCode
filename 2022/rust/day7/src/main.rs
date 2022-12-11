use std::fs;

#[derive(Debug)]
enum StructType {
    Dir,
    File
}

struct ParseHelper {
    line: usize,
    level: usize,
    s: String,
}

impl ParseHelper {
    fn new(line: usize, level: usize, s: &str) {
        
    }

    fn feeder(s: &str) {
        let mut level = 0;
        let v = s.split("\n").collect::<Vec<_>>();
        let mut vph = vec![];
        for (ii,l) in v.iter().enumerate() {
            
        }
    }
}

#[derive(Debug)]
pub struct BlindTree {
    name: String,
    stype: StructType,
    size: usize,
    contents: Vec<BlindTree>,
}

impl BlindTree {
    fn new(name: &str, st: StructType, size: usize) -> Self {
        Self {
            name: name.to_string(),
            stype: st,
            size: size,
            contents: vec![],
        }
    }

    fn add_contents(&mut self, ft: BlindTree) {
        match self.stype {
            StructType::File => panic!("Unable to add contents to File"),
            StructType::Dir => self.contents.push(ft),
        }
    }

}


fn read_file(name: &str) -> String {
    return fs::read_to_string(name)
    .expect("Should have been able to read the file");

}



fn main() {
    let mut input = read_file("input");
    let mut root = BlindTree::new("root", StructType::Dir, 0);
    println!("{:?}", &root);
    let child = BlindTree::new("child", StructType::Dir, 0);
    let mut child2 = BlindTree::new("child2", StructType::Dir, 0);
    let child3 = BlindTree::new("child3", StructType::Dir, 0);
    root.add_contents(child);
    child2.add_contents(child3);
    println!("{:?}", &child2);
    root.add_contents(child2);
    println!("{:?}", &root);
    // let inputs = input.split("\n$").collect::<Vec<_>>().iter().map(|s| parse_input(s)).collect::<Vec<_>>();
    // println!("{:?}", inputs); 
}
