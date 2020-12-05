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
        let birth = match self.birth.parse::<u16>() {
            Ok(i) => {
                if i >= 1920 && i <=2002 { true }
                else { false }
            },
            Err(_) => { false },
        };

        let issue = match self.issue.parse::<u16>() {
            Ok(i) => {
                if i >= 2010 && i <=2020 { true }
                else { false }
            },
            Err(_) => { false },
        };

        let expiry = match self.expiry.parse::<u16>() {
            Ok(i) => {
                if i >= 2020 && i <=2030 { true }
                else { false }
            },
            Err(_) => { false },
        };

        let mut digit = self.height.clone();
        let inch_check = digit.find("in").unwrap_or(digit.len());
        let inch = digit.split_off(inch_check);
        let cm_check = digit.find("cm").unwrap_or(digit.len());
        let cm = digit.split_off(cm_check);
        let height = match digit.parse::<u16>() {
            Ok(i) if inch == "in".to_string() => {
                match i {
                    59..=76 => true,
                    _ => false,
                }
            },
            Ok(i) if cm == "cm".to_string() => {
                 match i {
                     150..=193 => true,
                     _ => false,
                 }
            },
            _ => false,
        };

        
        let hair = match self.hair.chars().count() {
            7 => {
                match self.hair.chars().nth(0).unwrap() {
                    '#' => {
                        match i64::from_str_radix(self.hair.trim_start_matches('#'), 16) {
                            Ok(i) if i <= 16777215 => { true },
                            _ => false,
                        }
                    },
                    _ => false,
                }
            },
            _ => false,
        };

        let eye = match self.eye.as_str() {
            "amb" => true,
            "blu" => true,
            "brn" => true,
            "gry" => true,
            "grn" => true,
            "hzl" => true,
            "oth" => true,
            _ => false,
        };

        let pass_id = match self.pass_id.chars().count() {
            9 => {
                match self.pass_id.parse::<u32>() {
                    Ok(_) => true,
                    Err (_) => false,
                }
            },
            _ => false,
        };

        (birth && issue && expiry && height && hair && eye && pass_id ) as u16
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
