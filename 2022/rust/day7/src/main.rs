//I failed miserably
//using https://nickymeuleman.netlify.app/garden/aoc2022-day07 to move on to day 8 then will redo as time allows

use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

fn main() {
    println!("{}", part_1());
    println!("{}", part_2());
}

pub fn part_1() -> u32 {
    let input = fs::read_to_string("input").unwrap();
    let mut sizes = HashMap::new();
    let mut affected = Vec::new();

    for line in input.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let parts: Vec<_> = line.split_whitespace().collect();
        match parts[..] {
            ["$", "cd", ".."] => {
                affected.pop();
            }
            ["$", "cd", name] => {
                affected.push(name);
            }
            [size, _name] => {
                let size: u32 = size.parse().unwrap();
                for idx in 0..affected.len() {
                    let path = PathBuf::from_iter(&affected[..=idx]);
                    *sizes.entry(path).or_insert(0) += size;
                }
            }
            _ => {}
        };
    }

    sizes
        .into_values()
        .filter(|size| *size <= 100_000)
        .sum()
}

pub fn part_2() -> u32 {
    let input = fs::read_to_string("input").unwrap();

    let mut sizes = HashMap::new();
    let mut affected = Vec::new();

    for line in input.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let parts: Vec<_> = line.split_whitespace().collect();
        match parts[..] {
            ["$", "cd", ".."] => {
                affected.pop();
            }
            ["$", "cd", name] => {
                affected.push(name);
            }
            [size, _name] => {
                let size: u32 = size.parse().unwrap();
                for idx in 0..affected.len() {
                    let path = PathBuf::from_iter(&affected[..=idx]);
                    *sizes.entry(path).or_insert(0) += size;
                }
            }
            _ => {}
        };
    }

    let disk = 70_000_000;
    let needed = 30_000_000;
    let root = sizes.get(&PathBuf::from("/")).unwrap();
    let available = disk - root;

    sizes
        .into_values()
        .filter(|size| available + size >= needed)
        .min()
        .unwrap()
}