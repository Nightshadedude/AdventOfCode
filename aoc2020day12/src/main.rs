use std::fs;

/*
 * N: 1
 * E: 1
 * S: -1
 * W: -1
 */

struct Pathing<'a> {
     steps: Vec<(&'a str, i16)>,
}

struct Gps<'a> {
    facing: &'a str,
    x: i16,
    y: i16,
}

impl<'a> Pathing<'a> {
    fn new(s: &'a str) -> Self {
        let spl: Vec<&str> = s.split_terminator("\n").collect();
        let mut steps = vec![];
        for i in spl {
            let (inst, count) = i.split_at(1);
            steps.push((inst, count.parse::<i16>().unwrap_or(0)));
        }
        Pathing {
            steps
        }
    }

    fn walk_steps(&mut self) -> Gps {
        let mut current = Gps::new();
        for step in &self.steps{
            current.mv(*step);
        }
        current
    }
}

impl<'a> Gps<'a> {
    fn new() -> Self {
        Gps {
            facing: "E", //default is facing east
            x: 0,
            y: 0,
        }
    }

    fn mv(&mut self, inst: (&str, i16)) {
        match inst.0 {
            "N" => {
                self.x += inst.1;
            },
            "S" => {
                self.x -= inst.1;
            },
            "E" => {
                self.y += inst.1;
            },
            "W" => {
                self.y -= inst.1;
            },
            "L" => {
                self.facing = rot(self.facing, 360i16-inst.1);
            },
            "R" => {
                self.facing = rot(self.facing, inst.1);
            },
            "F" => {
                self.mv((self.facing, inst.1));
            },
            _ => {
                println!("Ignored bad instruction: {}, {}", inst.0, inst.1);
            },
        }
    }

    fn manhattan_distance(&self) {
        println!("dist: {}", self.x.abs()+self.y.abs());
    }
}

fn rot(dir: &str, rotation: i16) -> &str {
    let num_rotations = rotation / 90;
    let mut current_dir = dir;
    for _ in 0..num_rotations {
        current_dir = match current_dir {
            "N" => "E",
            "E" => "S",
            "S" => "W",
            "W" => "N",
            _ => "_",
        }
    }
    current_dir
}

fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename).expect("failed to read");
    let mut path = Pathing::new(&contents);
    path.walk_steps().manhattan_distance();
}
