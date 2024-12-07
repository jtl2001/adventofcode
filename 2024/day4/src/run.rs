pub fn run(input: &str, output: bool) {
    let input: Vec<Vec<char>> = input
        .split_whitespace()
        .map(|s| s.chars().collect())
        .collect();

    let mut count_xmas: u32 = 0;
    let mut count_mas_x: u32 = 0;

    let length = input.len();
    let width = input[0].len();

    for i in 0..length {
        for j in 0..width {
            match input[i][j] {
                'X' => count_xmas += search_char_xmas(i, j, &input),
                'A' => {
                    if i > 0 && i < length - 1 && j > 0 && j < width - 1 {
                        count_mas_x += test_mas_x(i, j, &input);
                    }
                }
                _ => continue,
            }
        }
    }

    if output {
        println!("Part 1: {}", count_xmas);
        println!("Part 2: {}", count_mas_x);
    }
}

fn search_char_xmas(i: usize, j: usize, input: &Vec<Vec<char>>) -> u32 {
    const DIRECTIONS: [(isize, isize); 8] = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    let mut count = 0;
    for d in DIRECTIONS {
        if search_direction_xmas(i, j, d, input) {
            count += 1
        }
    }

    return count;
}

fn search_direction_xmas(
    mut i: usize,
    mut j: usize,
    dir: (isize, isize),
    input: &Vec<Vec<char>>,
) -> bool {
    const MAS: [char; 3] = ['M', 'A', 'S'];

    for k in 0..3 {
        i = match i.checked_add_signed(dir.0) {
            Some(num) => num,
            None => return false,
        };
        j = match j.checked_add_signed(dir.1) {
            Some(num) => num,
            None => return false,
        };

        if MAS[k]
            != match input.get(i) {
                Some(s) => match s.get(j) {
                    Some(c) => *c,
                    None => return false,
                },
                None => return false,
            }
        {
            return false;
        }
    }
    return true;
}

fn test_mas_x(i: usize, j: usize, input: &Vec<Vec<char>>) -> u32 {
    // already checked that i is not at extrema
    let pair_1 = (
        input[i.checked_add(1).unwrap()][j.checked_add(1).unwrap()],
        input[i.checked_sub(1).unwrap()][j.checked_sub(1).unwrap()],
    );
    if !(pair_1 == ('M', 'S') || pair_1 == ('S', 'M')) {
        return 0;
    }
    let pair_2 = (
        input[i.checked_add(1).unwrap()][j.checked_sub(1).unwrap()],
        input[i.checked_sub(1).unwrap()][j.checked_add(1).unwrap()],
    );
    if !(pair_2 == ('M', 'S') || pair_2 == ('S', 'M')) {
        return 0;
    }
    return 1;
}
