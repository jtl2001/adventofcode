use std::sync::Mutex;
use std::thread;

pub fn run(input: &str, output: bool) {
    let lines: Vec<Vec<u64>> = input
        .split("\n")
        .map(|s| {
            s.trim()
                .replacen(':', "", 1)
                .split_whitespace()
                .map(|num| num.parse::<u64>().expect("NaN"))
                .collect()
        })
        .collect();

    let num_chunks = std::cmp::min(20, lines.len());
    let res_two_stack = &Mutex::new(Vec::<u64>::new());
    let res_three_stack = &Mutex::new(Vec::<u64>::new());

    calculate_ops(&lines[0], lines[0][0], lines[0].len() - 1);

    thread::scope(|s| {
        let mut handles = Vec::new();
        for l_chunk in lines.chunks(lines.len() / (num_chunks)) {
            let h = s.spawn(move || {
                let mut r2 = Vec::<u64>::new();
                let mut r3 = Vec::<u64>::new();
                for l in l_chunk {
                    let (two, three) = calculate_ops(l, l[0], &l.len() - 1);
                    if two {
                        r2.push(l[0]);
                        r3.push(l[0]);
                    } else if three {
                        r3.push(l[0]);
                    }
                }
                {
                    let mut r2_global = res_two_stack.lock().unwrap();
                    r2_global.append(&mut r2);
                }
                let mut r3_global = res_three_stack.lock().unwrap();
                r3_global.append(&mut r3);
            });
            handles.push(h);
        }
        for h in handles {
            h.join().unwrap();
        }
    });

    let valid_2_op: u64 = res_two_stack.lock().unwrap().iter().sum();
    let valid_3_op: u64 = res_three_stack.lock().unwrap().iter().sum();

    if output {
        println!("Part 1: {}", valid_2_op);
        println!("Part 2: {}", valid_3_op);
    }
}

fn calculate_ops(vals: &Vec<u64>, acc: u64, index: usize) -> (bool, bool) {
    if index == 0 {
        if acc == 0 {
            return (true, true);
        } else {
            return (false, false);
        }
    }

    let (mut two, mut three) = (false, false);
    let (mut temp1, mut temp2);

    if acc >= vals[index] {
        (temp1, temp2) = calculate_ops(vals, acc.checked_sub(vals[index]).unwrap(), index - 1);
        two |= temp1;
        three |= temp2;
        if two {
            return (true, three);
        }
    }

    if acc % vals[index] == 0 {
        (temp1, temp2) = calculate_ops(vals, acc / vals[index], index - 1);
        two |= temp1;
        three |= temp2;
        if three {
            return (two, true);
        }
    }

    if let Option::Some(s) = un_concat(acc, vals[index]) {
        (_, temp2) = calculate_ops(vals, s, index - 1);
        three |= temp2;
        return (false, three);
    }

    (two, three)
}

fn un_concat(mut a: u64, mut b: u64) -> Option<u64> {
    if a < b {
        return Option::None;
    };
    if a == b {
        return Option::Some(0);
    };

    while b > 0 && a % 10 == b % 10 {
        a /= 10;
        b /= 10;
    }
    if b == 0 {
        return Option::Some(a);
    }
    Option::None
}
