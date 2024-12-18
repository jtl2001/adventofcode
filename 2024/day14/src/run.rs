use std::cmp::Ordering;

const WIDTH: isize = 101;
const HEIGHT: isize = 103;
const SECONDS: usize = 10000;

pub fn run(input: &str, output: bool) {
    let mut nums = input
        .split(|c: char| !c.is_numeric() && c != '-')
        .filter_map(|s| {
            if !s.is_empty() {
                Some(s.parse::<isize>().unwrap())
            } else {
                None
            }
        })
        .peekable();

    let mut grid = vec![vec![vec![0_u32; WIDTH as usize]; HEIGHT as usize]; SECONDS];

    while nums.peek().is_some() {
        let mut x = nums.next().unwrap();
        let mut y = nums.next().unwrap();
        let u = nums.next().unwrap();
        let v = nums.next().unwrap();

        for g in grid.iter_mut() {
            x = (x + u).rem_euclid(WIDTH);
            y = (y + v).rem_euclid(HEIGHT);
            g[y as usize][x as usize] += 1;
        }
    }

    let tree_index = grid
        .iter()
        .enumerate()
        .map(|(i, g)| (i, calc_energy(g)))
        .max_by_key(|(_, g)| *g)
        .unwrap()
        .0;

    let grid = &grid[99];
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for i in 0..HEIGHT as usize {
        for j in 0..WIDTH as usize {
            match i.cmp(&((HEIGHT / 2) as usize)) {
                Ordering::Less => match j.cmp(&((WIDTH / 2) as usize)) {
                    Ordering::Less => q1 += grid[i][j] as u32,
                    Ordering::Greater => q2 += grid[i][j] as u32,
                    Ordering::Equal => (),
                },
                Ordering::Greater => match j.cmp(&((WIDTH / 2) as usize)) {
                    Ordering::Less => q3 += grid[i][j] as u32,
                    Ordering::Greater => q4 += grid[i][j] as u32,
                    Ordering::Equal => (),
                },
                Ordering::Equal => (),
            }
        }
    }

    let safety_factor = q1 * q2 * q3 * q4;

    if output {
        println!("Part 1: {}", safety_factor);
        println!("Part 2: {}", tree_index + 1);
    }
}

fn calc_energy(grid: &[Vec<u32>]) -> u32 {
    let mut energy = 0;
    for i in 1..grid.len() - 1 {
        for j in 1..grid[i].len() - 1 {
            let val = grid[i][j];
            if val > 0 {
                energy += val * grid[i - 1][j];
                //energy += val * grid[i-1][j-1];
                energy += val * grid[i][j - 1];
                //energy += val * grid[i+1][j-1];
                energy += val * grid[i + 1][j];
                //energy += val * grid[i+1][j+1];
                energy += val * grid[i][j + 1];
                //energy += val * grid[i-1][j+1];
            }
        }
    }

    energy
}
