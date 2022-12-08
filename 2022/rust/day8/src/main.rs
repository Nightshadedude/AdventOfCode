use std::fs;

#[derive(Debug)]
struct Forest {
    grid: Vec<Vec<usize>>,
}

impl Forest {
    fn new(s: String) -> Self {
        let parsed = s.lines()
            .map(|l| l.chars()
            .map(|c| c.to_digit(10).expect("why was I lied to?????") as usize)
            .collect::<Vec<_>>()
        ).collect::<Vec<Vec<_>>>();
    
        Self {
            grid: parsed,
        }
    }

    fn print(&self) {
        for v in self.grid.iter(){
            for u in v.iter() {
                print!("{}", u);
                print!(" ");
            }
            println!(" ");
        }
    }
    
    fn part_1(&self) -> usize { //yay brute force
        let h = self.grid.len();
        let w = self.grid[0].len();
        let mut total = 2*(h + w) - 4; //perimiter is all visible
        for ii in 1..=h-2{
            for jj in 1..=w-2{
                let mut vis = 0;
                let t = self.grid[ii][jj];
                // if self.grid[ii+1][jj] < t { vis = vis + 1; }
                // if self.grid[ii-1][jj] < t { vis = vis + 1; }
                // if self.grid[ii][jj+1] < t { vis = vis + 1; }
                // if self.grid[ii][jj-1] < t { vis = vis + 1; }

                //scan up
                let mut max_scan = 0;
                for up in 0..=ii-1{
                    if self.grid[up][jj] > max_scan { max_scan = self.grid[up][jj]; }
                }
                if t > max_scan { vis = vis + 1; }
                //scan down
                max_scan = 0;
                for down in ii+1..h{
                    if self.grid[down][jj] > max_scan { max_scan = self.grid[down][jj]; }
                }
                if t > max_scan { vis = vis + 1; }
                //scan left
                max_scan = 0;
                for left in 0..=jj-1{
                    if self.grid[ii][left] > max_scan { max_scan = self.grid[ii][left]; }
                }
                if t > max_scan { vis = vis + 1; }
                //scan right
                max_scan = 0;
                for right in jj+1..w{
                    if self.grid[ii][right] > max_scan { max_scan = self.grid[ii][right]; }
                }
                if t > max_scan { vis = vis + 1; }
                if vis > 0 {
                    total = total + 1;                    
                }
            }
        }
        total
    }
}

fn read_file(name: &str) -> String {
    return fs::read_to_string(name)
    .expect("Should have been able to read the file");
}




fn main() {
    let input = read_file("input");
    let f = Forest::new(input);
    println!("{}", f.part_1()); 
}
