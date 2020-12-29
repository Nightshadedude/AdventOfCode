use std::fs;

enum OpCode {
    Acc,
    Jmp,
    Nop,
    NA,
}

struct Operation{
    oc: OpCode,
    val: i32,
    consumed: bool,
}

impl Operation {
    pub fn new(s: &str)  -> Self {
        let spl: Vec<&str> = s.split(" ").collect();
        let parse_res = spl[1].parse::<i32>().expect("unable to parse num");
        let opcode = match spl[0]{
            "acc" => OpCode::Acc,
            "jmp" => OpCode::Jmp,
            "nop" => OpCode::Nop,
            _ => OpCode::NA,
        };
        Operation {
            oc: opcode,
            val: parse_res,
            consumed: false,
        }
    }
}

fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename).expect("failed to read");
    let mut ops = contents.split_terminator("\n")
        .map(|s| Operation::new(s))
        .collect::<Vec<Operation>>(); 
    let mut last_checked = 0i32;
    while ops.iter().nth(last_checked as usize).is_some() {
        match ops.iter_mut().nth(last_checked as usize).unwrap().oc {
            OpCode::Jmp => ops.iter_mut().nth(last_checked as usize).unwrap().oc = OpCode::Nop,
            OpCode::Nop => ops.iter_mut().nth(last_checked as usize).unwrap().oc = OpCode::Jmp,
            _ => (),
        };
        let mut pos = 0i32;
        let mut acc = 0;
        loop {
            match ops.iter_mut().nth(pos as usize) {
                Some(op) => match op.oc {
                    OpCode::Jmp if !op.consumed => {
                        pos += op.val;
                        op.consumed = true;
                    },
                    OpCode::Acc if !op.consumed => {
                        acc += op.val;
                        pos += 1;
                        op.consumed = true;
                    },
                    OpCode::Nop if !op.consumed => {
                        pos += 1;
                        op.consumed = true;
                    },
                    _ => {
                        break;
                    },
                },
                None => {
                    println!("successful exit: {}, acc {}", pos, acc);
                    break;
                },
            }
        };

        match ops.iter_mut().nth(last_checked as usize).unwrap().oc {
            OpCode::Jmp => ops.iter_mut().nth(last_checked as usize).unwrap().oc = OpCode::Nop,
            OpCode::Nop => ops.iter_mut().nth(last_checked as usize).unwrap().oc = OpCode::Jmp,
            _ => (),
        };
        last_checked += 1;
        ops.iter_mut().for_each(|op| op.consumed = false);
    }
} 
