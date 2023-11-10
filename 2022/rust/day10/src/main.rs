use std::fs;
use std::collections::VecDeque;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(isize),
}

#[derive(Debug)]
struct Program {
    ins: Vec<Instruction>,
    cycles: isize,
    register: isize,
    queue: VecDeque<isize>,
    sig_str: isize,
}

impl Program {
    fn parse(s: &str) -> Vec<Instruction> {
        let inp = s.lines().collect::<Vec<_>>();
        let mut instructions = vec![];
        for ii in inp.iter() {
            match &ii[0..=3] {
                "addx" => {
                    let num = ii.split(" ").collect::<Vec<_>>()[1].parse::<isize>().expect("failed parse");
                    instructions.push(Instruction::Addx(num));
                },
                "noop" => instructions.push(Instruction::Noop),
                _ => panic!("failed to parse: {}", ii),
            }
        }
        // println!("{:?}", instructions);
        instructions
    }

    fn new(s: &str) -> Self {
        Self{
            ins: Self::parse(s),
            cycles: 0,
            register: 0,
            queue: VecDeque::new(),
            sig_str: 0,
        }
    }

    fn execute(mut self) -> isize {
        self.register = 1;
        for ii in self.ins.iter() {
            match ii {
                Instruction::Addx(val) => {
                    self.queue.push_back(0isize);
                    self.queue.push_back(*val);
                },
                Instruction::Noop => {self.queue.push_back(0isize);},
                // _ => panic!("Unsupported instruction: {:?}", ii),
            }
        }
        // println!("{:?}", self.queue);
        while self.queue.len() > 0 {
            if self.cycles == self.register {
                print!("#");
            } else {
                print!(".");
            }
            if (self.cycles+1) % 40 == 0 {println!("");}
            self.cycles = self.cycles + 1;
            let num = self.queue.pop_front().unwrap();
            if (self.cycles+20)%40 == 0 {
                // println!("reg: {} || cyc: {} || sig: {}", self.register, self.cycles, self.register * self.cycles);
                // println!("", );
                self.sig_str = self.sig_str + (self.register * self.cycles); 
            }
            self.register =  self.register + num;
        }
        self.sig_str
    } 
}



fn read_file(name: &str) -> String {
    return fs::read_to_string(name)
    .expect("Should have been able to read the file");
}

fn main() {
    let test = read_file("input");
    println!("{}", Program::new(&test).execute());
}
