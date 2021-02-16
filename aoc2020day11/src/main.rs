use std::fs;

#[derive(Debug, PartialEq)]
struct SeatMap{
    layout: Vec<Vec<char>>,
}

impl SeatMap{
    fn new(string: String) -> SeatMap {
        let spl = string.split_terminator("\n").collect::<Vec<&str>>();
        let mut layout = vec![];
        for row in spl.iter() {
            let temp = row.chars().collect::<Vec<char>>();
            layout.push(temp);
        }

        SeatMap{
            layout,
        }
    }

    fn check_dirs(&self,
                x: usize,
                y: usize,
                distance_check: Option<usize>) ->
        u16 {
            let mut count = 0;
            let dirs = vec![(-1,-1), (-1,0), (-1,1),
                            (0,-1),/*(0,0),*/(0,1),
                            (1,-1), (1,0), (1,1)];
            for (x_shift, y_shift) in dirs {
                let mut x_check = x as i16 + x_shift;
                let mut y_check = y as i16 + y_shift;
                let mut found = false;
                let max_dist = match distance_check {
                    Some(x) => x,
                    None => self.layout.len(),
                };
                let mut curr_dist = 0;
                while 0 <= x_check && x_check < self.layout.len() as i16 &&
                    0 <= y_check && y_check < self.layout[0].len() as i16 &&
                    !found && curr_dist < max_dist {
                        match self.layout[x_check as usize][y_check as usize]{
                            '#' => {
                                count += 1;
                                found = true;
                            },
                            'L'=> {
                                found = true;
                            },
                            _ => {
                                x_check += x_shift;
                                y_check += y_shift;
                            },
                        }
                        curr_dist += 1;
                    }
            }
            count
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
                        let temp = self.check_dirs(x, y, None);
                        if temp > 0 {
                            row.push('L');
                        }
                        else {
                            row.push('#');
                        }
                    },
                    '#' => {
                        let temp = self.check_dirs(x, y, None);
                        if temp >= 5 {
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
