use std::fs;
use std::collections::HashMap;

fn read_file(name: &str) -> String {
    return fs::read_to_string(name)
    .expect("Should have been able to read the file");
}

fn parse_input_part1(s: &str) -> Vec<(usize,usize)> {
    let mut ret = vec![];
    let c: Vec<char> = s.chars().collect();
    for window in c.windows(12) {
        if window[0..4] == ['m','u','l','(']
        &&  window.contains(&',')
        &&  window.contains(&')') {
            let temp: String = window.into_iter().collect();
            let comma = temp.find(",");
            let paren = temp.find(")");
            if comma.is_none() || paren.is_none() { break; }
            let comma = comma.unwrap();
            let paren = paren.unwrap();
            if comma < paren {
                let left = temp[4..comma].parse::<usize>();
                let right = temp[comma+1..paren].parse::<usize>();
                if left.is_ok() && right.is_ok() {
                    ret.push((left.unwrap(),right.unwrap()))
                }     
            }
        }

    }
    ret
}

fn part1(v: Vec<(usize,usize)>) -> usize {
    v.iter().map(|(l,r)| l*r).sum()
}

fn parse_input_part2(s: &str) -> Vec<(usize,usize)> {
    let mut ret = vec![];
    let mut switch = true;
    let c: Vec<char> = s.chars().collect();
    for window in c.windows(12) {
        if window[0..7] == ['d','o','n','\'','t','(',')'] { switch = false; }
        if window[0..4] == ['d','o','(',')'] { switch = true; }
        if window[0..4] == ['m','u','l','(']
        &&  window.contains(&',')
        &&  window.contains(&')') {
            let temp: String = window.into_iter().collect();
            let comma = temp.find(",");
            let paren = temp.find(")");
            if comma.is_none() || paren.is_none() { break; }
            let comma = comma.unwrap();
            let paren = paren.unwrap();
            if comma < paren {
                let left = temp[4..comma].parse::<usize>();
                let right = temp[comma+1..paren].parse::<usize>();
                if left.is_ok() && right.is_ok() && switch {
                    ret.push((left.unwrap(),right.unwrap()))
                }     
            }
        }

    }
    ret
}



fn main() {
    println!("Hello, world!");
    let parsed = parse_input_part1(&read_file("input"));
    println!("parse: {:?}", part1(parsed));
    let parsed = parse_input_part2(&read_file("input"));
    println!("parse: {:?}", part1(parsed));
}

