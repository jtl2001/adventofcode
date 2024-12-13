use std::collections::HashMap;

pub fn run(input: &str, output: bool) {
    let num: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut rocks = Vec::new();
    let mut temp = num.clone();
    let mut index_map: HashMap<u64, u64> = HashMap::new();
    let mut curr_index = 0;
    while temp.len() > 0 {
        let n: u64 = temp.pop().unwrap();
        if !index_map.contains_key(&n) {
            index_map.insert(n, curr_index);
            curr_index += 1;
            let (x, is_second, y) = process_rock(n);
            rocks.push(Rock {
                count: 0,
                buffer: 0,
                children: (x, is_second, y),
            });
            temp.push(x);
            if is_second {
                temp.push(y);
            }
        }
    }

    for r in rocks.iter_mut() {
        r.children.0 = *index_map.get(&r.children.0).unwrap();
        r.children.2 = *index_map.get(&r.children.2).unwrap();
    }

    for n in num {
        rocks[*index_map.get(&n).unwrap() as usize].count = 1;
    }

    for _ in 1..=25 {
        for i in 0..rocks.len() {
            let r = rocks[i];
            if r.count != 0 {
                rocks[r.children.0 as usize].buffer += r.count;
                if r.children.1 {
                    rocks[r.children.2 as usize].buffer += r.count;
                }
            }
        }
        for r in rocks.iter_mut() {
            r.count = r.buffer;
            r.buffer = 0;
        }
    }

    let mut sum_1 = 0;
    for r in rocks.iter() {
        sum_1 += r.count
    }

    for _ in 26..=75 {
        for i in 0..rocks.len() {
            let r = rocks[i];
            if r.count != 0 {
                rocks[r.children.0 as usize].buffer += r.count;
                if r.children.1 {
                    rocks[r.children.2 as usize].buffer += r.count;
                }
            }
        }
        for r in rocks.iter_mut() {
            r.count = r.buffer;
            r.buffer = 0;
        }
    }

    let mut sum_2 = 0;
    for r in rocks.iter() {
        sum_2 += r.count
    }

    if output {
        println!("Part 1: {}", sum_1);
        println!("Part 2: {}", sum_2);
    }
}

#[derive(Debug, Clone, Copy)]
struct Rock {
    count: u64,
    buffer: u64,
    children: (u64, bool, u64),
}

fn process_rock(x: u64) -> (u64, bool, u64) {
    if x == 0 {
        return (1, false, 0);
    }
    let count = x.ilog10() + 1;
    if count % 2 == 0 {
        let half = (10 as u64).pow(count / 2);
        (x % half, true, x / half)
    } else {
        (x * 2024, false, 0)
    }
}
