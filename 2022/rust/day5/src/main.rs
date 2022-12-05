use std::fs;

fn read_file(name: &str) -> String {
    return fs::read_to_string(name)
    .expect("Should have been able to read the file");
}

struct Crates {
    crates: Vec<Vec<String>>,
}

impl Crates {
    fn new(s: &str) -> Self {
        let mut crates: Vec<Vec<String>> = vec![vec![]];
        let mut lines = s.lines().collect::<Vec<_>>();
        lines.reverse();
        for (ii, line) in lines.iter().enumerate(){
            if ii == 0 {
                let len =line.chars().collect::<Vec<char>>().len() / 4;
                for _ in 0..len+1{
                    let temp_vec = vec![];
                    crates.push(temp_vec);
                }
            }
            else {
                let char_vec: Vec<char> = line.chars().collect();
                let chunk = char_vec.chunks(4);
                for (jj, c) in chunk.enumerate() {
                    let mut temp_str:String = c.into_iter().collect();
                    temp_str = temp_str.trim().to_string();
                    if temp_str == "".to_string() {}
                    else {crates[jj+1].push(temp_str);}
                }
            }
        }
        Self {
            crates: crates,
        }
    }
}
#[derive(Debug, Copy, Clone)]
struct Move {
    num_moved: usize,
    from_stack: usize,
    to_stack: usize,
}

impl Move {
    fn new(s: &str) -> Self {
        let split = s.split(" ").collect::<Vec<_>>();
        Self {
            num_moved: split[1].parse::<usize>().unwrap(),
            from_stack: split[3].parse::<usize>().unwrap(),
            to_stack: split[5].parse::<usize>().unwrap(),
        }
    }
}

struct Cargo {
    crates: Crates,
    moves: Vec<Move>,
}

impl Cargo {
    fn new(c: Crates, v: Vec<Move>) -> Self {
        Self {
            crates: c,
            moves: v,
        }
    }

    fn crane_magic(&mut self){
        let mut ret_vec: Vec<String> = vec![];
        for m in self.moves.iter(){
        for _ in 0..m.num_moved{
            let popped = self.crates.crates[m.from_stack].pop().unwrap();
            self.crates.crates[m.to_stack].push(popped);
        }
        }
        println!("{:#?}", self.crates.crates);
    }

    fn crane_magic_9001(&mut self){
        let mut ret_vec: Vec<String> = vec![];
        for m in self.moves.iter(){
            let mut temp_vec: Vec<String> = vec![];
            for _ in 0..m.num_moved{
                 temp_vec.push(self.crates.crates[m.from_stack].pop().unwrap());
            }
            for _ in 0..temp_vec.len(){
                self.crates.crates[m.to_stack].push(temp_vec.pop().unwrap());
            }

        }
        println!("{:#?}", self.crates.crates);
    }

}

fn main() {
    let input = read_file("input");
    let input = input.split("\n\n").collect::<Vec<&str>>();
    let crates = Crates::new(input[0]);
    let moves = input[1].lines().map(|s| Move::new(s)).collect::<Vec<Move>>();
    //Cargo::new(crates, moves).crane_magic();
    Cargo::new(crates, moves).crane_magic_9001();
}
