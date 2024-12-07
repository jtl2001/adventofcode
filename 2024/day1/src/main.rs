use std::cmp::Ordering;
use std::fs;

fn main() {
    let input = fs::read_to_string(".\\src\\input.txt").expect("Failed to read file");
    let input = input.trim();

    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    parse_input(&input, &mut left, &mut right);

    left.sort();
    right.sort();

    let mut sum_diff: u32 = 0;

    left.iter()
        .zip(right.iter())
        .for_each(|(l, r)| sum_diff += l.abs_diff(*r));

    println!("Part 1: {sum_diff}");

    let mut total: u32 = 0;
    
    let mut left = left.iter().peekable();
    let mut right = right.iter().peekable();

    while left.peek().is_some() && right.peek().is_some() {
        let l: &u32 = left.peek().unwrap();
        let r: &u32 = right.peek().unwrap();
        match l.cmp(&r) {
            Ordering::Greater => {
                while right.next_if(|&x| x < l).is_some() {}
            },
            Ordering::Less => {
                while left.next_if(|&x| x < r).is_some() {}
            },
            Ordering::Equal => {
                let mut l_count = 1;
                let mut r_count = 1;

                left.next();
                right.next();

                while right.next_if(|&x| x == l).is_some() {
                    r_count += 1;
                }
                while left.next_if(|&x| x == l).is_some() {
                    l_count += 1;
                }

                total += l * l_count * r_count;
            }
        }
    }

    println!("Part 2: {total}");
}

fn parse_input(input: &str, left: &mut Vec<u32>, right: &mut Vec<u32>) {
    let lines = input.split("\n");

    for s in lines {
        let mut val = s.trim().split_whitespace();

        left.push(val.next().unwrap().parse().unwrap());
        right.push(val.next().unwrap().parse().unwrap());
    }
}