
fn check_safe(v: &Vec<usize>) -> bool {
    (check_inc(v.clone())
    || check_dec(v.clone()))
    && check_window(v.clone())
}

fn check_inc(v: Vec<usize>) -> bool {
    let mut v2 = v.clone();
    v2.sort();
    v == v2
}

fn check_dec(v: Vec<usize>) -> bool {
    let mut v2 = v.clone();
    v2.sort_by(|a, b| b.cmp(a));
    v == v2
}

fn check_window(v: Vec<usize>) -> bool {
    let mut it = v.clone();
    for window in it.windows(2) {
        let mut dist = 0;
        if window[0] > window[1] {
            dist = window[0] - window[1];
        } else {
            dist = window[1] - window[0];
        }
        if dist > 3 || dist < 1 {
            return false;
        } else {
        }
    }
    return true;
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
    let v = v.iter().map(|vt| check_safe(&*vt)).collect::<Vec<_>>();
    v.iter().filter(|cs| *cs == &true).count()
}

fn part2(v: Vec<Vec<usize>>) -> usize {
    let mut count = 0;
    for t in v.iter() {
        for ii in 0..t.len() {
            let mut tt = t.clone();
            tt.remove(ii);
            if(check_safe(&tt)) {
                count += 1;
                break;
            }
        }
    }
    count
}


fn main() {
    let data = include_str!("input");
    let parsed = parse_input(data);
    println!("part1: {:?}", part1(parsed.clone()));
    println!("part1: {:?}", part2(parsed.clone()));
}
