use std::fs;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Shuttle {
    time: u32,
    buses: Vec<u32>,
}

impl Shuttle {
    fn parse(input: &str) -> Self {
        let pre_parse = input.split_terminator("\n").collect::<Vec<&str>>();
        let time = pre_parse[0].parse::<u32>().unwrap();
        let buses = pre_parse[1].split(",")
            .map(|s| s.parse::<u32>().unwrap_or(0))
            .filter(|n| n > &0)
            .collect::<Vec<u32>>();
        Shuttle {
            time,
            buses,
        }
    }

    fn next_departure(&self) -> (u32, u32) {
        let mut try_time = self.time;
        let mut depart_time = 0;
        let mut depart_bus = 0;
        while depart_time == 0 {
            for bus in &self.buses {
                if try_time % bus == 0 {
                    depart_time = try_time;
                    depart_bus = *bus;
                }
            }
            try_time += 1;
        }
        (try_time-1, depart_bus)
    }

    fn part_1(&self) {
        let (depart_time, depart_bus) = self.next_departure();
        let initial_time = self.time;
        let p1 = (depart_time - initial_time) * depart_bus;
        println!("Part 1: {}", p1);
    }
}


fn main() {
    let filename = "input";
    let file_contents = fs::read_to_string(filename).expect("failed to read");
    Shuttle::parse(&file_contents).part_1();
}

#[test]
fn test_parse() {
    let test_str = "939\n7,13,x,x,59,x,31,19";
    let test_shuttle = Shuttle::parse(test_str);
    let comp_shuttle = Shuttle {
        time: 939u32,
        buses: vec![7u32,13u32,59u32,31u32,19u32],
    };
    assert_eq!(test_shuttle, comp_shuttle);
}

#[test]
fn test_departure() {
    let test_str = "939\n7,13,x,x,59,x,31,19";
    let test_shuttle = Shuttle::parse(test_str);
    assert_eq!(test_shuttle.next_departure(), (944u32, 59u32));
}


