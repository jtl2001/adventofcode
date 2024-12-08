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

    let num_chunks = 20;
    let res_two_stack = &Mutex::new(Vec::<u64>::new());
    let res_three_stack = &Mutex::new(Vec::<u64>::new());

    thread::scope(|s| {
        let mut handles = Vec::new();
        for l_chunk in lines.chunks(lines.len() / (num_chunks - 1)) {
            let h = s.spawn(move || {
                let mut r2 = Vec::<u64>::new();
                let mut r3 = Vec::<u64>::new();
                for l in l_chunk {
                    let (two, three) = calculate_ops(l, l[1], 2, &l.len());
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

fn calculate_ops(vals: &Vec<u64>, acc: u64, index: usize, len: &usize) -> (bool, bool) {
    if index == *len {
        if acc == vals[0] {
            return (true, true);
        } else {
            return (false, false);
        }
    }
    if acc > vals[0] {
        return (false, false);
    }

    let (mut two, mut three) = calculate_ops(vals, acc + vals[index], index + 1, len);
    if two || three {
        return (two, three);
    }

    (two, three) = calculate_ops(vals, acc * vals[index], index + 1, len);
    if two || three {
        return (two, three);
    }

    return (
        false,
        calculate_ops(vals, concat(acc, vals[index]), index + 1, len).1,
    );
}

fn concat(mut a: u64, b: u64) -> u64 {
    let mut temp = b;

    while temp > 0 {
        a *= 10;
        temp /= 10;
    }
    let val = a + b;
    return val;
}
