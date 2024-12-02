use std::fs;
use std::collections::HashMap;

fn read_file(name: &str) -> String {
    return fs::read_to_string(name)
    .expect("Should have been able to read the file");
}

fn parse_input(s: &str) -> (Vec<usize>, Vec<usize>, HashMap<usize,usize>) {
    let lines = s.lines();
    let mut ret = vec![];
    let mut ret1 = vec![];
    let mut ret2 = HashMap::new();
    for line in lines {
        let temp = line.split("   ").map(|i| i.parse::<usize>().unwrap()).collect::<Vec<_>>();
        ret.push(temp[0]);
        ret1.push(temp[1]); 
        match ret2.get_mut(&temp[1]) {
            Some(val) => {*val = &*val+1},
            None => {
                _ = ret2.insert(temp[1],1);
            },
        }
    }
    (ret,ret1,ret2)
}

fn part1(input: (Vec<usize>, Vec<usize>, HashMap<usize,usize>)) -> usize {
    let (mut v1, mut v2, _) = input;
    v1.sort();
    v2.sort();
    let mut dist = 0;
    for (ii, val) in v1.iter().enumerate() {
        let val2 = v2[ii];
        if *val > val2 {
            dist = dist + (val-val2);
        } else {
            dist = dist + (val2-val);
        }
    }
    dist
}

fn part2(input: (Vec<usize>, Vec<usize>, HashMap<usize,usize>)) -> usize {
    let (v1, _, h1) = input;
    let mut dist = 0;
    for (ii, val) in v1.iter().enumerate() {
        let count = h1.get(val).unwrap_or(&0);
        dist = dist + (val * count);
    }
    dist
}

fn main() {
    println!("Hello, world!");
    let parsed = parse_input(&read_file("input"));
    println!("{:?}", part1(parsed.clone()));
    println!("{:?}", part2(parsed.clone()));
}

