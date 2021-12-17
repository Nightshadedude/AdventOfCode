use std::fs;
use std::collections::HashMap;


struct AdapterArray {
    adapters: Vec<u64>,
    voltage_diff: Vec<u64>, //voltage diff
}

impl AdapterArray {
    fn new(s: &str) -> Self {
        let mut spl = s.split_terminator("\n")
            .map(|i| i.parse::<u64>()
                 .expect("unable to parse"))
            .collect::<Vec<u64>>();
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

fn part2(
    mtable: &mut HashMap<(u64, usize), u64>,
    pval: u64,
    idx: usize,
    nums: &[u64],
    ) -> u64 {
    if let Some(res) = mtable.get(&(pval, idx)) {
        return *res;
    }
    let slice = &nums[idx..nums.len()];
    if slice[0] - pval > 3 {
        return 0;
    }

    if slice.len() == 1 {
        return 1;
    }
    let w_next =
        part2(mtable, slice[0], idx + 1, nums);
    let wo_next = if slice[1] - pval > 3 {
        0
    } else {
        part2(mtable, pval, idx + 1, nums)
    };
    let res = w_next + wo_next;
    mtable.insert((pval, idx), res);
    res
}



fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename).expect("failed to read");
    let mut adapter_array = AdapterArray::new(&contents);
    adapter_array.calc_diff();
    println!("answer: {}", adapter_array.get_answer());
    let part_2 = part2(&mut HashMap::new(),
        adapter_array.adapters[0],
        1,
        &adapter_array.adapters);
    println!("{}", part_2);
}
