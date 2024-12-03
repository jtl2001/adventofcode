use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string(".\\src\\input.txt").expect("Failed to read file");
    let input = input.trim();

    let instructions_pattern = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let number_pattern = Regex::new(r"\d+").unwrap();

    let mut sumproduct: u32 = instructions_pattern
        .find_iter(input)
        .map(|m| {
            number_pattern
                .find_iter(m.as_str())
                .map(|n| n.as_str().parse::<u32>().expect("NaN"))
                .product::<u32>()
        })
        .sum::<u32>();

    println!("Part 1: {}", sumproduct);

    let complex_pattern = Regex::new(r"mul\(\d+,\d+\)|do(?:n't)?\(\)").unwrap();
    let filter_dont = Regex::new(r"(?:don't\(\))+(?:mul\(\d+,\d+\))+").unwrap();

    let mut instructions = complex_pattern
        .find_iter(input)
        .map(|m| m.as_str())
        .collect::<String>();
    instructions = filter_dont.split(instructions.as_str()).collect::<String>();

    sumproduct = instructions_pattern
        .find_iter(instructions.as_str())
        .map(|m| {
            number_pattern
                .find_iter(m.as_str())
                .map(|n| n.as_str().parse::<u32>().expect("NaN"))
                .product::<u32>()
        })
        .sum::<u32>();

    println!("Part 2: {}", sumproduct);
}
