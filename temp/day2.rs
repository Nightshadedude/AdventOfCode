 use std::env;
 use std::fs;

pub struct policy {
      min: u16,
      max: u16,
      ch: char,
      pass: String,
  }

  impl policy {
      fn new(unparsed: &str) -> Self {
          let partial_parse = unparsed.split_whitespace().collect::<Vec<&str>>();
          let temp = partial_parse[0].split("-").collect::<Vec<&str>>();
          policy {
              min: temp[0].parse::<u16>().unwrap_or(0),
              max: temp[1].parse::<u16>().unwrap_or(0),
              ch: partial_parse[1].chars().next().unwrap_or(' '),
              pass: String::from(partial_parse[2]),
          }
      }

      fn is_valid(&self) -> bool {
          let count = self.pass.matches(self.ch).count() as u16;
          let rng = self.min..=self.max;
          return rng.contains(&count);
      }
  }
  
  fn main() {
      let filename = "input";
      let contents = fs::read_to_string(filename) Result<String, Error>
          .expect("failed to read");
      let strs = contents.split_terminator("\n").collect::<Vec<&str>>();
      let pol_check = strs.iter().map(|s| policy::new(s)).collect::<Vec<policy>>();
      println!("{}", pol_check.iter().map(|pol| policy::is_valid(pol) as u16).sum::<u  16>());
  }
