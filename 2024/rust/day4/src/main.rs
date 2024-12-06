use std::fs;

fn read_file(name: &str) -> String {
    return fs::read_to_string(name)
    .expect("Should have been able to read the file");
}

fn parse_input_part1(s: &str) -> (Vec<Vec<char>>, usize, usize) {
    let lines = s.lines();
    let char_grid = lines.map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let h = char_grid.len();
    let w = char_grid[0].len();
    (char_grid,h,w)
}

fn part1(char_grid: Vec<Vec<char>>, h: usize, w: usize) -> usize{
    let mut count = 0;
    for ii in 0..h {
        for jj in 0..w {
            if char_grid[ii][jj] == 'X' {
                count += check_around_part1(&char_grid, h, w, ii, jj);
            }
        }
    }
    count
}

fn check_around_part1(char_grid: &Vec<Vec<char>>, h: usize, w: usize, ii: usize, jj: usize) -> usize {
    //up / down
    let mut count = 0;
    let xmas = ['X','M','A','S'];
    //left/right
    let dirs: Vec<(isize,isize)> = vec![
        (-1,0), //up
        (1,0), //down
        (0,-1), //left
        (0,1), //right
        (-1,-1), //ul
        (-1,1), //ur
        (1,-1), //dl
        (1,1), //dr
    ];
    for dir in dirs {
        let mut found = 0;
        for dist in 1isize..=3isize {
            let i = ii as isize +(dir.0*dist);
            let j = jj as isize+(dir.1*dist);
            if i < 0 || i >= h as isize || j < 0 || j >= w as isize {()}
            else if char_grid[i as usize][j as usize] == xmas[dist as usize] {
                found += 1;
            }
        }
        if found == 3 { count += 1 }
    }
    count
}


fn part2(char_grid: Vec<Vec<char>>, h: usize, w: usize) -> usize{
    let mut count = 0;
    for ii in 0..h {
        for jj in 0..w {
            if char_grid[ii][jj] == 'A' {
                count += check_around_part2(&char_grid, h, w, ii, jj);
            }
        }
    }
    count
}

fn check_around_part2(char_grid: &Vec<Vec<char>>, h: usize, w: usize, ii: usize, jj: usize) -> usize {
    //up / down
    let mut count = 0;
    //left/right
    let dirs: Vec<(isize,isize)> = vec![
        (-1,-1), //ul
        (-1,1), //ur
        (1,-1), //dl
        (1,1), //dr
    ];
    let mut found = vec![];
    for dir in dirs {
        let i = ii as isize + dir.0;
        let j = jj as isize + dir.1;
        if i < 0 || i >= h as isize || j < 0 || j >= w as isize {()}
        else if char_grid[i as usize][j as usize] == 'S' || char_grid[i as usize][j as usize] == 'M' {
            found.push(char_grid[i as usize][j as usize])
        }
    }
    match &found[..] {
        ['M','M','S','S'] => count += 1,
        ['S','S','M','M'] => count += 1,
        ['M','S','M','S'] => count += 1,
        ['S','M','S','M'] => count += 1,
        _ => (),
    }
    count
}


fn main() {
    println!("Hello, world!");
    let parsed = parse_input_part1(&read_file("input"));
    println!("part1: {:?}", part1(parsed.0.clone(), parsed.1, parsed.2));
    println!("part2: {:?}", part2(parsed.0, parsed.1, parsed.2));

}

