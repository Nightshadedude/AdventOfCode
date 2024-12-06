use std::fs;
use std::collections::HashMap;

fn read_file(name: &str) -> String {
    return fs::read_to_string(name)
    .expect("Should have been able to read the file");
}

fn parse_input_part1(s: &str) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let (page_order, updates) = s.split_once("\n\n").unwrap();
    let page_order = page_order.lines().map(|s| s.split("|").map(|ss| ss.parse::<usize>().unwrap()).collect::<Vec<_>>()).collect::<Vec<Vec<usize>>>();
    let updates = updates.lines().map(|s| s.split(",").map(|ss| ss.parse::<usize>().unwrap()).collect::<Vec<_>>()).collect::<Vec<Vec<usize>>>();
    let mut po_hash = HashMap::new();
    for po in page_order {
        if po_hash.get(&po[0]).is_none() {
            po_hash.insert(po[0], vec![po[1]]);
        } else {
            if let Some(v) = po_hash.get_mut(&po[0]) {
                v.push(po[1]);
            }
        }
    }
    // println!("{:?}", &po_hash);
    (po_hash, updates)
}


fn part1(po_hash: HashMap<usize, Vec<usize>>, updates: Vec<Vec<usize>>) -> usize {
    let mut valid_updates = vec![];
    let mut invalid_updates = vec![];
    for update in updates {
        // println!("in update: {:?}", &update);
        let mut valid = true;
        for (ii,up) in update.iter().enumerate() {
            if let Some(rule_vec) = po_hash.get(&up) {
                // println!("[{}] - checking up: {} - rulevec: {:?}", ii, &up, &rule_vec);
                for rule in rule_vec {
                    // println!("in update[0..ii]: {:?}", &update[0..ii]);
                    for u in &update[0..ii] {
                        if u == rule {
                            // println!("rule broken:{}|{}", &up, &rule);
                            valid = false;
                            break;
                        }
                    }
                    if !valid { break; }
                }
            }
            if !valid { break; }
        }
        if valid {
            valid_updates.push(update.clone());
        } else {
            invalid_updates.push(update.clone());
        }

    }
    println!("part 2: {:?}", part2(po_hash.clone(), invalid_updates));
    valid_updates.iter().map(|v| v[v.len()/2]).sum()
}

fn part2(po_hash: HashMap<usize, Vec<usize>>, updates: Vec<Vec<usize>>) -> usize {
    let mut reordered = vec![];
    for update in updates {
        reordered.push(make_valid(&update, &po_hash));
    }
    reordered.iter().map(|v| v[v.len()/2]).sum()
}

fn make_valid(update: &Vec<usize>, po_hash: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let len = update.len()-1;
    let mut pending = update.clone();
    let mut temp = pending.clone();
    let mut valid = false;
    while !valid{
        let mut updates_made = false;
        for (ii,up) in pending.clone().iter().enumerate() {
            if let Some(rule_vec) = po_hash.get(&up) {
                // println!("[{}] - checking up: {} - rulevec: {:?}", ii, &up, &rule_vec);
                for rule in rule_vec {
                    // println!("in update[0..ii]: {:?}", &update[0..ii]);
                    // println!("reviwing rule: {:?} - update[..]: {:?}", rule, &update[0..ii]);
                    for (jj,u) in pending[0..ii].iter().enumerate() {
                        if u == rule {
                            temp.swap(ii, jj);
                            updates_made = true;
                            // println!("after swap: {:?}", &pending);
                        }
                        if updates_made { break; }
                    }
                    if updates_made { break; }
                }
            }
            if updates_made { break; }
        }
        if !updates_made { valid = true; }
        else { pending = temp.clone() } 
    }
    pending
}

fn main() {
    println!("Hello, world!");
    let parsed = parse_input_part1(&read_file("input"));
    println!("part1: {:?}", part1(parsed.0, parsed.1));
}

