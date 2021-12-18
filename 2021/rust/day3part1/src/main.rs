use std::fs;

fn main() {
    let file = "input";
    let data= fs::read_to_string(file).unwrap().lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut gamma = "".to_string();
    let mut epsilon = "".to_string();
    for ii in 0..data[0].len() {
        let mut sum = 0;
        for jj in 0..data.len() {
            sum += data[jj][ii].to_digit(10).unwrap_or(0);
        }
        if sum < (data.len() as u32)/2 { 
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        } 
    }
    let result = usize::from_str_radix(&gamma, 2).unwrap() * usize::from_str_radix(&epsilon, 2).unwrap();
    println!("{}", result);
}
