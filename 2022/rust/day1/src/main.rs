use std::fs;

fn read_file(name: &str) -> String {
    return fs::read_to_string(name)
    .expect("Should have been able to read the file");
}

fn main() {
    let input = read_file("input");
    let elves = input.split("\n\n").collect::<Vec<_>>();
    let mut calories = vec![];
    for elf in elves.iter(){
        let temp: i32 = elf.split("\n").collect::<Vec<_>>().iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>().iter().sum();
        calories.push(temp)
    }
    println!("{:?}", calories.iter().max());

    calories.sort();
    calories.reverse();

    println!("{:?}", &calories[0..=2].iter().sum::<i32>());
}
