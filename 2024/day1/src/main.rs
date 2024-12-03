use std::cmp::Ordering;
use std::fs;

fn main() {
    let input = fs::read_to_string(".\\src\\input.txt").expect("Failed to read file");
    let input = input.trim();

    let (mut left, mut right) = parse_input(&input);

    left.sort();
    right.sort();

    let mut sum_diff: u32 = 0;

    left.iter()
        .zip(right.iter())
        .for_each(|(l, r)| sum_diff += l.abs_diff(*r));

    println!("Part 1: {sum_diff}");

    let mut left = left.iter();
    let mut right = right.iter();

    let mut l = *left.next().expect("Should contain at least one entry");
    let mut r = *right.next().expect("Should contain at least one entry");

    let mut value = 0;
    let mut left_count = 0;
    let mut right_count = 0;

    let mut total = 0;

    loop {
        if l == value {
            left_count += 1;
            match left.next() {
                Some(num) => l = *num,
                None => break,
            }
        } else if r == value {
            right_count += 1;
            match right.next() {
                Some(num) => r = *num,
                None => break,
            }
        } else {
            match l.cmp(&r) {
                Ordering::Greater => {
                    increment_total(&mut value, &mut left_count, &mut right_count, &mut total);
                    match right.next() {
                        Some(num) => r = *num,
                        None => break,
                    }
                }
                Ordering::Less => {
                    increment_total(&mut value, &mut left_count, &mut right_count, &mut total);
                    match left.next() {
                        Some(num) => l = *num,
                        None => break,
                    }
                }
                Ordering::Equal => {
                    if l != value {
                        increment_total(&mut value, &mut left_count, &mut right_count, &mut total);
                    }
                    value = l;
                    left_count += 1;
                    right_count += 1;
                    match right.next() {
                        Some(num) => r = *num,
                        None => break,
                    }
                    match left.next() {
                        Some(num) => l = *num,
                        None => break,
                    }
                }
            }
        }
    }

    if value != 0 {
        increment_total(&mut value, &mut left_count, &mut right_count, &mut total);
    }

    println!("Part 2: {total}");
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    let lines = input.split("\n");

    for s in lines {
        let val: Vec<&str> = s.trim().split_whitespace().collect();

        left.push(val[0].parse().expect("Nan"));
        right.push(val[1].parse().expect("Nan"));
    }

    return (left, right);
}

fn increment_total(value: &mut u32, left_count: &mut u32, right_count: &mut u32, total: &mut u32) {
    *total += *value * *left_count * *right_count;
    *value = 0;
    *left_count = 0;
    *right_count = 0;
}
