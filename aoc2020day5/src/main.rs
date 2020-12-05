use std::fs;

struct BoardingPass {
    row: u16,
    row_multipler: u16,
    column: u16,
}

impl BoardingPass {
    fn new(code: &str) -> Self {
        let mut code = code.to_string();
        let split = code.split_off(7);
        BoardingPass {
            row: BoardingPass::calc_row(&code),
            row_multipler: 8,
            column: BoardingPass::calc_col(&split),
        }
    }

    fn calc_seat(&self) -> u16 {
        self.row * self.row_multipler + self.column
    }

    fn calc(code: &str, upper: u16) -> u16 {
        let mut lower_range = 0;
        let mut upper_range = upper;
        code.chars().for_each(|c| match c {
            'F' => {
                let delta = (upper_range - lower_range) / 2;
                upper_range = lower_range + delta;
            }
            'L' => {
                let delta = (upper_range - lower_range) / 2;
                upper_range = lower_range + delta;
            }
            'B' => {
                let delta = (upper_range - lower_range) / 2;
                lower_range = upper_range - delta;
            }
            'R' => {
                let delta = (upper_range - lower_range) / 2;
                lower_range = upper_range - delta;
            }
            _ => {}
        });
        //        match code.chars().nth(code.chars().count() - 1).unwrap_or(' ') {
        //            'F' => lower_range,
        //            'L' => lower_range,
        //            'B' => upper_range,
        //            'R' => upper_range,
        //            _ => 0,
        //        }
        lower_range
    }

    fn calc_col(code: &str) -> u16 {
        BoardingPass::calc(code, 7)
    }

    fn calc_row(code: &str) -> u16 {
        BoardingPass::calc(code, 127)
    }
}

fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename).expect("failed to read");
    let mut passes = contents
        .split_terminator("\n")
        .map(|s| BoardingPass::new(s).calc_seat())
        .collect::<Vec<u16>>();
    passes.sort();
    for window in passes.windows(2) {
        if window[1] - window[0] != 1 {
            println!("{:#?}", window[1] - 1);
        };
    }
}
