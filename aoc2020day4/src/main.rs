use std::fs;

pub struct Passport {
    birth: String,
    issue: String,
    expiry: String,
    height: String,
    hair: String,
    eye: String,
    pass_id: String,
    country_id: String,
}

impl Passport {
    pub fn new(pass: &str) -> Self {
        let mut birth: String = "".to_string();
        let mut issue: String = "".to_string();
        let mut expiry: String = "".to_string();
        let mut height: String = "".to_string();
        let mut hair: String = "".to_string();
        let mut eye: String = "".to_string();
        let mut pass_id: String = "".to_string();
        let mut country_id: String = "".to_string();
        
        let partial_parse = pass.split_whitespace().collect::<Vec<&str>>();
        for kv in partial_parse {
            let key_val = kv.split(':').collect::<Vec<&str>>();
            match key_val[0] {
                "byr" if key_val.len() == 2 => birth = key_val[1].to_string(),
                "iyr" if key_val.len() == 2 => issue = key_val[1].to_string(),
                "eyr" if key_val.len() == 2 => expiry = key_val[1].to_string(),
                "hgt" if key_val.len() == 2 => height = key_val[1].to_string(),
                "hcl" if key_val.len() == 2 => hair = key_val[1].to_string(),
                "ecl" if key_val.len() == 2 => eye = key_val[1].to_string(),
                "pid" if key_val.len() == 2 => pass_id = key_val[1].to_string(),
                "cid" if key_val.len() == 2 => country_id = key_val[1].to_string(),
                _ => println!("unable to parse kv: {:#?}", key_val),
            }
        }

        Passport {
            birth,
            issue,
            expiry,
            height,
            hair,
            eye,
            pass_id,
            country_id,
        }
    }

    pub fn is_valid(&self) -> u16 {
        if self.birth.chars().count() > 0usize &&
            self.issue.chars().count() > 0usize &&
            self.expiry.chars().count() > 0usize &&
            self.height.chars().count() > 0usize &&
            self.hair.chars().count() > 0usize &&
            self.eye.chars().count() > 0usize &&
            self.pass_id.chars().count() > 0usize {
                return 1u16;
            } else {
                return 0u16;
            }
    }
}

fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename).expect("failed to read");
    let passports = contents.split("\n\n")
        .map(|s| Passport::new(s).is_valid())
        .sum::<u16>();
    println!("{}", passports);
}
