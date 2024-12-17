pub fn run(input: &str, output: bool) {
    let mut nums = input
        .split(|c: char| !c.is_numeric())
        .filter_map(|s| {
            if !s.is_empty() {
                Some(s.parse::<i64>().unwrap())
            } else {
                None
            }
        })
        .peekable();

    let mut tokens_part_1 = 0;
    let mut tokens_part_2 = 0;

    while nums.peek().is_some() {
        let x1 = nums.next().unwrap();
        let y1 = nums.next().unwrap();
        let x2 = nums.next().unwrap();
        let y2 = nums.next().unwrap();
        let mut x_target = nums.next().unwrap();
        let mut y_target = nums.next().unwrap();

        let denominator = x1 * y2 - x2 * y1;

        let a_numerator = y2 * x_target - x2 * y_target;
        let b_numerator = x1 * y_target - y1 * x_target;
        x_target += 10000000000000;
        y_target += 10000000000000;
        let a_numerator_real = y2 * x_target - x2 * y_target;
        let b_numerator_real = x1 * y_target - y1 * x_target;

        if a_numerator % denominator == 0 && b_numerator % denominator == 0 {
            tokens_part_1 += 3 * a_numerator / denominator;
            tokens_part_1 += b_numerator / denominator;
        }
        if a_numerator_real % denominator == 0 && b_numerator_real % denominator == 0 {
            tokens_part_2 += 3 * a_numerator_real / denominator;
            tokens_part_2 += b_numerator_real / denominator;
        }
    }

    if output {
        println!("Part 1: {}", tokens_part_1);
        println!("Part 1: {}", tokens_part_2);
    }
}
