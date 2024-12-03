
fn check_safe_p1(v: &Vec<usize>) -> bool {
    (count_inc(v.clone()) == 0) || (count_dec(v.clone()) == 0) && (count_dist(v.clone()) == 0)
}

// fn check_safe_p2(v: &Vec<usize>) -> bool {
//     true
// }

fn count_inc(v: Vec<usize>) -> usize {
    v.len() - 1 - v.windows(2).filter(|v| v[1] > v[0]).collect::<Vec<_>>().len()
}

fn count_dec(v: Vec<usize>) -> usize {
    v.len() - 1 - v.windows(2).filter(|v| v[1] < v[0]).collect::<Vec<_>>().len()
}

fn count_dist(v: Vec<usize>) -> usize {
    v.len() - 1 - v.windows(2).map(|v| ((v[0] as isize) - (v[1] as isize)).abs()).filter(|d| d <= &3 && d >= &1).collect::<Vec<_>>().len()
}

fn parse_input(s: &str) -> Vec<Vec<usize>> {
    let lines = s.lines();
    let mut ret = vec![];
    for line in lines{
        let temp = line.split(" ").map(|i| i.parse::<usize>().unwrap()).collect::<Vec<_>>();
        ret.push(temp);
    }
    ret
}

fn part1(v: Vec<Vec<usize>>) -> usize {
    let v = v.iter().map(|vt| check_safe_p1(&*vt)).filter(|i| i == &true).collect::<Vec<_>>();
    v.len()
}

// fn part2(v: Vec<Vec<usize>>) -> usize {
//     let v = v.iter().map(|vt| check_safe_p2(&*vt)).filter(|i| i == &true).collect::<Vec<_>>();
//     v.len()
// }


fn main() {
    let data = include_str!("input");
    let parsed = parse_input(data);
    println!("part1: {:?}", part1(parsed.clone()));
    // println!("part2: {:?}", part2(parsed.clone()));
}
