use std::fs;

struct AdapterArray {
    adapters: Vec<u16>,
    voltage_diff: Vec<u16>, //voltage diff
}

impl AdapterArray {
    fn new(s: &str) -> Self {
        let mut spl = s.split_terminator("\n")
            .map(|i| i.parse::<u16>()
                 .expect("unable to parse"))
            .collect::<Vec<u16>>();
        spl.push(0);
        spl.sort();
        spl.push(spl.last().unwrap()+3);
        AdapterArray {
            adapters: spl,
            voltage_diff: vec![],
        }
    }

    fn calc_diff(&mut self) {
        println!("{:?}", self.adapters);
        for window in self.adapters.windows(2) {
            let diff = window[1] - window[0];
            self.voltage_diff.push(diff);
        }
        println!("Max: {}", self.adapters.last().unwrap()+3);
    }

    fn get_answer(&self) -> u64 {
        let jolt_1 = self.voltage_diff.iter().filter(|&n| *n==1).count();
        let jolt_3 = self.voltage_diff.iter().filter(|&n| *n==3).count();
        println!("1-{}, 3-{}", jolt_1, jolt_3);
        (jolt_1 * jolt_3) as u64
    }
}



fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename).expect("failed to read");
    let mut adapter_array = AdapterArray::new(&contents);
    adapter_array.calc_diff();
    println!("answer: {}", adapter_array.get_answer());
}
