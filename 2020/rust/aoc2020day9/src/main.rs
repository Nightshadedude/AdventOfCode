use std::fs;

const PREAMBLE_LENGTH: usize = 25;

struct EncodedMessage {
    payload: Vec<u64>,
}

impl EncodedMessage {
    fn new(s: &str) -> Self {
        let spl = s.split_terminator("\n")
            .map(|i| i.parse::<u64>()
                 .expect("unable to parse"))
            .collect::<Vec<u64>>();

        if spl.len() < PREAMBLE_LENGTH { panic!("input too short"); };
        EncodedMessage {
            payload: spl,
        }
    }

    fn find_invalid(&self) -> Option<u64> {
        let mut rolling_window = self.payload.windows(PREAMBLE_LENGTH + 1).peekable();
        while rolling_window.peek().is_some() {
            let mut valid = false;
            let rw = rolling_window.next().unwrap();
            for first in 0..rw.len()-2 {
                for second in first+1..rw.len()-1 {
                    if rw[first] + rw[second] == rw[rw.len()-1] { valid = true; }
                }
            }
            if !valid { return Some(rw[rw.len()-1]); }
        }
        None
    }

    fn find_contiguous(&self, to_find: u64) -> u64 {
        for i in 0..self.payload.len() {
            let mut cont_slice = Vec::new();
            cont_slice.push(self.payload[i]);
            for j in i+1..self.payload.len() {
                if cont_slice.iter().sum::<u64>() == to_find {
                    cont_slice.sort();
                    return cont_slice[0]+cont_slice[cont_slice.len()-1];
                }
                if j > self.payload.len() {}
                cont_slice.push(self.payload[j]);
            }
        }
        0   
    }
}

fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename).expect("failed to read");
    let msg = EncodedMessage::new(&contents);
    let num = msg.find_invalid().unwrap_or(0);
    println!("{:?}", num);
    println!("{}", msg.find_contiguous(num));
} 
