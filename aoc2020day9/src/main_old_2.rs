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

    fn find_invalid(&mut self) -> Option<u64> {
        let mut rolling_window = self.payload
            .windows(PREAMBLE_LENGTH-1)
            .peekable();
        while rolling_window.peek().is_some() {
            let mut valid = false;
            let rw = rolling_window.next().unwrap();
            if rolling_window.peek().is_some() {
                let peekrw = rolling_window.peek().unwrap();
                for first in 0..rw.len()-1 {
                    for second in 1..rw.len() {
                        if rw[first] + rw[second] == peekrw[0] {
                            valid = true;
                        }
                    }
                }
                if !valid {return Some(peekrw[0]);}
             }
        }
        None
    }
}

fn main() {

    let filename = "input";
    let contents = fs::read_to_string(filename).expect("failed to read");
    let mut msg = EncodedMessage::new(&contents);
    println!("invalid: {:#?}", msg.find_invalid());
} 
