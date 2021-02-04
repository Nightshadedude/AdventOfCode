use std::fs;

const PREAMBLE_LENGTH: usize = 25;

struct EncodedMessage {
    payload: Vec<u64>,
}

impl EncodedMessage {
    fn new(s: &str) -> Self {
        let payload = s.split_terminator("\n")
            .map(|i| i.parse::<u64>()
                 .expect("unable to parse"))
            .collect::<Vec<u64>>();
        
        if payload.len() < PREAMBLE_LENGTH { panic!("input too short"); };

        EncodedMessage {
            payload,
        }
    }
    
    fn find_invalid(&mut self) -> Option<u64> {
        let mut next_elem = PREAMBLE_LENGTH + 1;
        for window in self.payload.windows(PREAMBLE_LENGTH) {
            if next_elem > self.payload.len() { break; }
            let mut valid = false;
            for first in 0..window.len()-1 {
                for second in first+1..window.len() {
                    if window[first] + window[second] ==  self.payload[next_elem]{
                        valid = true;
                    }
                }
            }
            
            if !valid {
                println!("window: {:?} matching: {} elem: {}", window, self.payload[next_elem], next_elem);
                return Some(self.payload[next_elem]);
            }
            next_elem += 1;
        }
        None
    }
/*
    fn largest_contiguous(&mut self) -> Vec<u64> {
        let mut ret_vec: Vec<u64> = vec![];
        let full_vec: Vec<u64> = self.preamble
            .iter()
            .cloned()
            .chain(self.payload
                   .iter()
                   .cloned())
            .collect();
        let mut temp_vec = vec![];
        let mut last_num = 0u64;
        full_vec.iter().for_each(|num| {
            if *num > last_num {
                temp_vec.push(*num);
                last_num = *num;
            } else if temp_vec.len() >= ret_vec.len() {
                println!("temp_vec: {:?}", temp_vec);
                ret_vec.clear();
                ret_vec.append(&mut temp_vec);
                last_num = *num;
            } else {
                temp_vec.clear();
                last_num = *num;
            }
        });

        ret_vec
    }
*/}

fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename).expect("failed to read");
    let mut msg = EncodedMessage::new(&contents);
    println!("invalid: {:#?}", msg.find_invalid());
//    let mut largest = msg.largest_contiguous();
//    largest.sort();
//    println!("weakness: {}", largest[0]+largest[largest.len()-1]);
} 
