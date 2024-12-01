use std::fs;

fn read_file(name: &str) -> String {
    return fs::read_to_string(name)
    .expect("Should have been able to read the file");
}

fn parse_input(s: &str) -> (Vec<usize>, Vec<usize>) {
    let lines = s.lines();
    let mut ret = vec![];
    let mut ret1 = vec![];
    for line in lines {
        let temp = line.split(" ").filter(|x| *x != "").map(|i| i.parse::<usize>().unwrap()).collect::<Vec<_>>();
        ret.push(temp[0]);
        ret1.push(temp[1]);        
    }
    (ret,ret1)
}

fn part1(input: (Vec<usize>, Vec<usize>)) -> usize {
    let (mut v1, mut v2) = input;
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

fn part2(input: (Vec<usize>, Vec<usize>)) -> usize {
    let (mut v1, mut v2) = input;
    let mut dist = 0;
    for (ii, val) in v1.iter().enumerate() {
        let count = v2.iter().filter(|i| *i == val).count();
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
