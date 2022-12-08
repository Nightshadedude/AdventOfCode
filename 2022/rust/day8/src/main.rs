use std::fs;
use std::cmp;

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

    fn part_2(&self) -> usize { //yay brute force
        let h = self.grid.len();
        let w = self.grid[0].len();
        let mut x = 0;
        let mut y = 0;
        let mut max_score = 0; 
        for ii in 0..h{
            for jj in 0..w{
                let mut up_score = 0;
                let mut down_score = 0;
                let mut left_score = 0;
                let mut right_score = 0;
                let t = self.grid[ii][jj];
                //scan up
                let mut max_scan = 0;
                let u = (0..ii).rev();
                for up in (0..ii).rev(){
                    up_score = up_score + 1;
                    if max_scan <= self.grid[up][jj] {
                        max_scan = self.grid[up][jj];
                        if max_scan >= t {
                            break;
                        }
                    }
                }
                //scan down
                max_scan = 0;
                let mut range = ii;
                if ii != h { range = ii+1 }
                let d = (range..h);
                for down in (range..h){
                    down_score = down_score + 1;
                    if max_scan <= self.grid[down][jj] {
                        max_scan = self.grid[down][jj];
                        if max_scan >= t {
                            break;
                        }
                    }
                }
                //scan left
                max_scan = 0;
                let l = (0..jj).rev();
                for left in (0..jj).rev(){
                    left_score = left_score + 1;
                    if max_scan <= self.grid[ii][left] {
                        max_scan = self.grid[ii][left];
                        if max_scan >= t {
                            break;
                        }
                    }
                }
                
                //scan right
                max_scan = 0;
                let mut range = jj;
                if jj != w { range = jj+1 }
                let r = (range..w);
                for right in (range..w){
                    right_score = right_score + 1;
                    if max_scan <= self.grid[ii][right] {
                        max_scan = self.grid[ii][right];
                        if max_scan >= t {
                            break;
                        }
                    }
                }

                let score = cmp::max(0, up_score) * cmp::max(0, down_score) * cmp::max(0, left_score) * cmp::max(0, right_score);

                if score > max_score {
                    max_score = score;
                }
            }            
        }
        max_score
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
    println!("{}", f.part_2());
}
