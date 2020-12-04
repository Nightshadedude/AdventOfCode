use std::fs;

pub struct Slope {
    pattern: Vec<String>,
    right: usize,
    down: usize,
}

impl Slope {
    pub fn new(pat: Vec<String>, rt: usize, dn: usize) -> Self {
        Slope {
            pattern: pat,
            right: rt,
            down: dn,
        }
    }

    pub fn calc_impact(&self) -> u16 {
        let max_height = self.pattern.len();
        let max_width = self.pattern[0].chars().count();
        let mut width = 0usize;
        let mut height = 0usize;
        let mut acc = 0;
        while height < max_height - self.down {
            width += self.right;
            if width >= max_width {
                width = width % max_width
            };
            height += self.down;
            let ch = self.pattern[height].chars().nth(width).unwrap_or(' ');
            match ch {
                '.' => acc += 0,
                '#' => acc += 1,
                ' ' => println!("loop broke: h {},w {}", height, width),
                _ => println!("something else broke: {}", ch),
            }
        }
        return acc;
    }
}

fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename).expect("failed to read");
    let slope = Slope::new(
        contents
            .split_terminator('\n')
            .map(|s| String::from(s))
            .collect::<Vec<String>>(),
        3usize,
        1usize,
    );
    println!("{}", slope.calc_impact());
}
