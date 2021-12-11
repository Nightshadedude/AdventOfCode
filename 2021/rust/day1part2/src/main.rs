use std::fs;
fn main() {
    let file = "input";
    let data: Vec<u32> = fs::read_to_string(file).unwrap().lines().map(|x| x.parse().unwrap()).collect();
    let interim: Vec<u32> = data.windows(3).map(|x| x[0]+x[1]+x[2]).collect();
    println!("{}", interim.windows(2).filter(|x| x[1] > x[0]).count());
}
