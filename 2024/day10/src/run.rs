use rustc_hash::FxHashSet as HashSet;
use std::ops::AddAssign;

pub fn run(input: &str, output: bool) {
    // This breaks the input up into a 2d vector of u8s, and buffers
    // every side with a high value (16 in this case)
    // For example,
    //
    //           16 16 16 16 16 16
    // 0123      16  0  1  2  3 16
    // 1234  ->  16  1  2  3  4 16
    // 8765      16  8  7  6  5 16
    // 9876      16  9  8  7  6 16
    //           16 16 16 16 16 16
    //
    let mut grid: Vec<Vec<u8>> = vec![vec![16;input.find("\n").unwrap()+2];1];
    for mut row in input.split_ascii_whitespace().map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>()) {
        let mut temp = vec![16];
        temp.append(&mut row);
        temp.push(16);
        grid.push(temp);
    }
    grid.push(grid[0].clone());

    let mut states: Vec<Vec<Option<State>>> = vec![vec![None;grid[0].len()];grid.len()];

    debug_assert!(grid.iter().all(|x| x.len() == grid[0].len()));
    debug_assert!(states.len() == grid.len());
    debug_assert!(states.iter().all(|x| x.len() == grid[0].len()));

    let mut part_1 = 0;
    let mut part_2 = 0;
    // only need to iterate through real values
    for x in 1..grid.len()-1 {
        for y in 1..grid[0].len()-1 {
            if grid[x][y] == 0 {
                let s = search(x, y, &grid, &mut states);
                part_1 += s.reachable_9s.len();
                part_2 += s.distinct_paths;
            }
        }
    }

    if output {
        println!("Part 1: {}", part_1);
        println!("Part 2: {}", part_2);
    }
}

fn search(x: usize, y: usize, grid: &Vec<Vec<u8>>, state: &mut Vec<Vec<Option<State>>>) -> State {
    debug_assert!(x > 0);
    debug_assert!(y < grid.len());
    debug_assert!(x > 0);
    debug_assert!(y < grid[x].len());
    match &state[x][y] {
        Some(s) => s.clone(),
        None => {
            match grid[x][y] {
                9 => state[x][y] = Some(State::new_9(x, y)),
                num => {
                    let mut new_state = State::new();
                    [(x-1, y), (x+1, y), (x, y-1), (x, y+1)].iter().for_each(|(new_x, new_y)| {
                        debug_assert!(*new_x < grid.len());
                        debug_assert!(*new_y < grid[*new_x].len());
                        if grid[*new_x][*new_y] == num + 1 {
                            new_state += search(*new_x, *new_y, grid, state);
                        }
                    });
                    state[x][y] = Some(new_state);
                },
            }
            state[x][y].clone().unwrap()
        },
    }
}

//fn search(x: usize, y: usize, )

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Point {
    x: usize,
    y: usize,
}


#[derive(Debug, Clone)]
struct State {
    reachable_9s: HashSet<Point>,
    distinct_paths: u32,
}

impl State {
    fn new_9(x: usize, y: usize) -> State {
        State {
            reachable_9s: [Point{x,y}].into_iter().collect(),
            distinct_paths: 1,
        }
    }
    fn new() -> State {
        State {
            reachable_9s: HashSet::default(),
            distinct_paths: 0,
        }
    }
}

impl AddAssign for State {
    fn add_assign(&mut self, rhs: Self) {
        self.reachable_9s.extend(rhs.reachable_9s);
        self.distinct_paths += rhs.distinct_paths;
    }
}