use itertools::Itertools;
use std::collections::HashMap;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

pub fn run(input: &str, output: bool) {
    let mut x_dim = 0;
    let mut y_dim = 0;

    let mut frequencies: HashMap<char, Vec<Point>> = HashMap::new();

    for row in input.split_whitespace() {
        y_dim = 0;
        for c in row.trim().chars() {
            if c != '.' {
                if !frequencies.contains_key(&c) {
                    frequencies.insert(c, Vec::new());
                }
                frequencies
                    .get_mut(&c)
                    .unwrap()
                    .push(Point::new_from_coords(x_dim, y_dim));
            }
            y_dim += 1;
        }
        x_dim += 1;
    }

    let mut strict_grid = vec![vec![false; y_dim as usize]; x_dim as usize];
    let mut full_grid = strict_grid.clone();

    for (_, f) in frequencies {
        for (a, b) in f.iter().tuple_combinations() {
            let diff = b - *a;
            try_add(b + diff, &mut strict_grid);
            try_add(a - diff, &mut strict_grid);

            let mut temp = a.clone();
            while try_add_with_result(temp, &mut full_grid).is_ok() {
                temp += diff;
            }
            temp = a - diff;
            while try_add_with_result(temp, &mut full_grid).is_ok() {
                temp -= diff;
            }
        }
    }

    let mut strict_counter: u32 = 0;
    for row in strict_grid {
        for col in row {
            if col {
                strict_counter += 1
            }
        }
    }

    let mut full_counter: u32 = 0;
    for row in full_grid {
        for col in row {
            if col {
                full_counter += 1
            }
        }
    }

    if output {
        println!("Part 1: {}", strict_counter);
        println!("Part 2: {}", full_counter);
    }
}

fn try_add(p: Point, grid: &mut Vec<Vec<bool>>) {
    if p.is_not_neg() {
        let x = p.x as usize;
        let y = p.y as usize;
        if x < grid.len() && y < grid[x].len() {
            grid[x][y] = true;
        }
    }
}

fn try_add_with_result(p: Point, grid: &mut Vec<Vec<bool>>) -> Result<(), ()> {
    if p.is_not_neg() {
        let x = p.x as usize;
        let y = p.y as usize;
        if x < grid.len() && y < grid[x].len() {
            grid[x][y] = true;
            return Ok(());
        }
    }
    return Err(());
}

#[derive(Hash, Debug, Copy, Clone, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new_from_coords(x: isize, y: isize) -> Point {
        Point { x: x, y: y }
    }
    fn is_not_neg(&self) -> bool {
        self.x >= 0 && self.y >= 0
    }
}

impl Add<Point> for &Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign<Point> for Point {
    fn sub_assign(&mut self, rhs: Point) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Sub<Point> for &Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Neg for Point {
    type Output = Point;
    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}
