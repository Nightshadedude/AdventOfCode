use std::fs;
fn main() {
    let file = "input";
    let data: Vec<u32> = fs::read_to_string(file).unwrap().lines().map(|x| x.parse().unwrap()).collect();
    println!("{}", data.windows(2).filter(|x| x[1] > x[0]).count());
}
