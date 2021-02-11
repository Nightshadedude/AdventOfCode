use std::fs;

#[derive(Debug, PartialEq)]
struct SeatMap{
    layout: Vec<Vec<char>>,
    //next_layout: Vec<Vec<char>>,
}

impl SeatMap{
    fn new(string: String) -> SeatMap {
        let mut spl = string.split_terminator("\n").collect::<Vec<&str>>();
        let mut layout = vec![];
        let blank = (0..spl[0].len()).map(|_| ".").collect::<String>();
        spl.insert(0, &blank);
        spl.push(&blank);
        for row in spl.iter() {
            let mut temp = row.chars().collect::<Vec<char>>();
            temp.insert(0, '.');
            temp.push('.');
            layout.push(temp);
        }

        SeatMap{
            layout,
        }
    }

    fn validate_seats(&mut self) {
        let x_max = self.layout.len();
        let y_max = self.layout[0].len();
        let mut check: Vec<Vec<char>> = Vec::new();
        for x in 0..x_max {
            let mut row: Vec<char> = vec![];
            for y in 0..y_max {
                let seat = self.layout[x][y];
                match seat {
                    'L' => {
                        let mut temp = 0;
                        for x_offset in 0..=2 {
                            for y_offset in 0..=2 {
                                if x_offset == 1 && y_offset == 1 {}
                                else {
                                    match self
                                        .layout[x+x_offset-1][y + y_offset-1]
                                        {
                                            '#' => {temp += 1;},
                                            _ => (),
                                        }
                                }
                            }
                        }
                        if temp > 0 {
                            row.push('L');
                        }
                        else {
                            row.push('#');
                        }
                    },
                    '#' => {
                        let mut temp = 0;
                        for x_offset in 0..=2 {
                            for y_offset in 0..=2 {
                                if x_offset == 1 && y_offset == 1 {}
                                else {
                                    match self
                                        .layout[x+x_offset-1][y + y_offset-1]
                                        {
                                            '#' => {temp += 1;},
                                            _ => (),
                                        }
                                }
                            }
                        }
                        if temp >= 4 {
                            row.push('L');
                        }
                        else {
                            row.push('#');
                        }

                    },
                    '.' => {
                        row.push('.');
                    },
                    _ => {
                        println!("Unsure how we got here, X: {}, Y: {}, Ch: {}",
                        x,
                        y,
                        self.layout[x][y]);
                    },
                }
            }
            check.push(row);
        }
        self.layout = check;
    }

    fn let_er_rip(&mut self) -> u64 {
        loop {
            let prev = self.layout.clone();
            self.validate_seats();
            let next = self.layout.clone();
            if prev == next {
                break;
            }
        }
        self.count('#')
    }

    fn count(&self, ch: char) -> u64 {
        let mut count = 0;
        for row in self.layout.iter() {
            for layout_ch in row.iter() {
                if layout_ch == &ch {
                    count += 1;
                }
            }
        }
        count
    }
}

fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename).expect("failed to read");
    let mut seats = SeatMap::new(contents);
    println!("{}", seats.let_er_rip());
}
