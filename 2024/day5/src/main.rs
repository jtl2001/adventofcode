use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string(".\\src\\input.txt").expect("Failed to read file");
    let input = input.trim();

    let mut lines = input.split("\n").map(|s| s.trim());

    let mut orders: HashMap<u32, HashSet<u32>> = HashMap::new();

    loop {
        let s = lines.next().unwrap().trim();
        if s == "" {
            break;
        }

        let mut s = s.split('|');

        let first: u32 = s.next().unwrap().parse().expect("NaN");
        let second: u32 = s.next().unwrap().parse().expect("NaN");

        if !orders.contains_key(&first) {
            orders.insert(first, HashSet::new());
        }

        orders.get_mut(&first).unwrap().insert(second);
    }

    let mut sum_correct: u32 = 0;
    let mut sum_incorrect: u32 = 0;

    for l in lines {
        let mut updates: Vec<u32> = l
            .trim()
            .split(',')
            .map(|n| n.parse::<u32>().expect("NaN"))
            .collect();

        let presorted =
            updates.is_sorted_by(|i, j| compare_with_edges(i, j, &orders) == Ordering::Less);

        if presorted {
            sum_correct += updates[updates.len() / 2];
        } else {
            updates.sort_by(|i, j| compare_with_edges(i, j, &orders));
            sum_incorrect += updates[updates.len() / 2];
        }
    }

    println!("Part 1: {}", sum_correct);
    println!("Part 1: {}", sum_incorrect);
}

fn compare_with_edges(i: &u32, j: &u32, map: &HashMap<u32, HashSet<u32>>) -> Ordering {
    if i == j {
        return Ordering::Equal;
    };
    return match map.get(&i) {
        Some(s) => match s.contains(&j) {
            true => Ordering::Less,
            false => Ordering::Greater,
        },
        None => Ordering::Greater,
    };
}
