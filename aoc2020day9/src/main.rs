use std::fs;

const PREAMBLE_LENGTH: usize = 25;

struct EncodedMessage {
    preamble: Vec<u64>,
    payload: Vec<u64>,
}

impl EncodedMessage {
    fn new(s: &str) -> Self {
        let spl = s.split_terminator("\n")
            .map(|i| i.parse::<u64>()
                 .expect("unable to parse"))
            .collect::<Vec<u64>>();
        
        if spl.len() < PREAMBLE_LENGTH { panic!("input too short"); };
        
        let mut preamble: Vec<u64> = vec![]; 
        for i in 0..PREAMBLE_LENGTH {
            preamble.push(spl[i]);
        }

        let mut payload: Vec<u64> = vec![];
        for i in PREAMBLE_LENGTH..spl.len() {
            payload.push(spl[i]);
        }

        EncodedMessage {
            preamble,
            payload,
        }
    }
    
    fn find_invalid(&mut self) -> Option<u64> {
        let rolling_window = &mut self.preamble;
        for num in &self.payload {
            let mut valid = false;
            for first in 0..rolling_window.len()-1 {
                for second in first+1..rolling_window.len() {
                    if rolling_window[first] + rolling_window[second] == *num {
                        valid = true;
                    }
                }
            }

            if !valid { return Some(*num); }
            
            rolling_window.drain(0..1);
            rolling_window.push(*num);
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
