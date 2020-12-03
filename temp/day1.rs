  use std::env;
  use std::fs;

  fn main() {
      let filename = "input";
      let contents = fs::read_to_string(filename) Result<String, Error>
          .expect("failed to read");
      let strs = contents.split_terminator("\n").collect::<Vec<&str>>();
      let mut nums = strs.iter() Iter<&str>
          .map(|s| s.parse::<u32>().unwrap_or(0)) impl Iterator<Item = u32>
          .collect::<Vec<u32>>();
      while nums.last().is_some() {
          let temp = nums.pop().unwrap();
          for i in 0..nums.len() {
              for j in 1..nums.len() {
                  if nums[i] + nums[j] + temp == 2020 && i != j {
                      println!("Answer: {},{},{} = {}", nums[i], nums[j], temp, nums[  i]*nums[j]*temp);
                  }
              }
          }
      }
  } 
