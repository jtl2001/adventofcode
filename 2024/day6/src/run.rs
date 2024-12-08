
pub fn run(input: &str, output: bool) {
    let mut grid: Vec<Vec<State>> = Vec::new();
    let mut guard = Guard {
        x: 0,
        y: 0,
        direction: Direction::Up,
    };

    for (x, r) in input.split_whitespace().enumerate() {
        let mut row: Vec<State> = Vec::new();
        for (y, val) in r.chars().enumerate() {
            if val == '#' {
                row.push(State::Wall);
            } else {
                if val != '.' && val != '@' {
                    guard.x = x;
                    guard.y = y;
                    match val {
                        '^' => guard.direction = Direction::Up,
                        '>' => guard.direction = Direction::Right,
                        'v' => guard.direction = Direction::Down,
                        '<' => guard.direction = Direction::Left,
                        c => panic!("Invalid character '{}'", c),
                    }
                }
                row.push(State::new_space());
            }
        }
        grid.push(row);
    }

    let mut can_make_loop: u32 = 0;
    let mut age: u32 = 0;

    grid[guard.x][guard.y].add_traversal(&guard.direction, &age);
    loop {
        match guard.step(&grid) {
            Ok(_) => (),
            Err(StepError::Wall) => guard.rotate_right(),
            Err(StepError::Oob) => break,
        }
        grid[guard.x][guard.y].add_traversal(&guard.direction, &age);
        if guard.look(&mut grid, &age) {
            can_make_loop += 1
        };
        age += 1;
    }

    let mut num_visited: u32 = 0;
    for row in grid {
        for value in row {
            if value.has_been_visited() {
                num_visited += 1;
            }
        }
    }

    if output {
        println!("Part 1: {}", num_visited);
        println!("Part 2: {}", can_make_loop);
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate_right(&mut self) {
        match self {
            Direction::Up => *self = Direction::Right,
            Direction::Right => *self = Direction::Down,
            Direction::Down => *self = Direction::Left,
            Direction::Left => *self = Direction::Up,
        }
    }
    fn as_coords(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

#[derive(Copy, Clone)]
enum State {
    Wall,
    Space(Traversal),
}

#[derive(Copy, Clone)]
struct Traversal {
    flags: u8,
    temp_wall: bool,
    orig_dir: Direction,
    temp_age: u32,
    first_age: u32,
    up_age: u32,
    right_age: u32,
    down_age: u32,
    left_age: u32,
}

impl State {
    fn new_space() -> State {
        State::Space(Traversal {
            flags: 0b0,
            temp_wall: false,
            orig_dir: Direction::Up,
            temp_age: 0,
            first_age: 0,
            up_age: 0,
            right_age: 0,
            down_age: 0,
            left_age: 0,
        })
    }
    fn is_wall(&self) -> bool {
        matches!(self, State::Wall)
    }
    fn is_temp_wall(&self) -> bool {
        match self {
            State::Space(s) => s.temp_wall,
            _ => false,
        }
    }
    fn add_temp_wall(&mut self) {
        match self {
            State::Space(s) => s.temp_wall = true,
            _ => panic!("Walls cannot be temp walls"),
        }
    }
    fn add_traversal(&mut self, dir: &Direction, age: &u32) {
        let first_traversal = !self.has_been_visited();
        match self {
            State::Wall => panic!("Only works on spaces"),
            State::Space(s) => {
                match dir {
                    Direction::Up => {
                        s.flags |= 0b1000_0000;
                        s.up_age = *age;
                    }
                    Direction::Right => {
                        s.flags |= 0b0100_0000;
                        s.right_age = *age;
                    }
                    Direction::Down => {
                        s.flags |= 0b0010_0000;
                        s.down_age = *age;
                    }
                    Direction::Left => {
                        s.flags |= 0b0001_0000;
                        s.left_age = *age;
                    }
                }
                if first_traversal {
                    s.orig_dir = *dir;
                    s.first_age = *age;
                };
            }
        }
    }
    fn get_first_traversal(&self) -> Direction {
        match self {
            State::Wall => panic!("Only works on spaces"),
            Self::Space(s) => s.orig_dir,
        }
    }
    fn get_first_age(&self) -> u32 {
        match self {
            State::Wall => panic!("Only works on spaces"),
            Self::Space(s) => s.first_age,
        }
    }
    fn add_temp_traversal(&mut self, dir: &Direction, age: &u32) {
        match self {
            State::Wall => panic!("Only works on spaces"),
            State::Space(s) => {
                if s.temp_age < *age {
                    s.flags &= 0b1111_0000;
                    s.temp_age = *age;
                }
                match dir {
                    Direction::Up => s.flags |= 0b0000_1000,
                    Direction::Right => s.flags |= 0b0000_0100,
                    Direction::Down => s.flags |= 0b0000_0010,
                    Direction::Left => s.flags |= 0b0000_0001,
                }
            }
        }
    }

    fn check_traversal(&self, dir: &Direction, age: &u32, intersection_age: &u32) -> bool {
        match self {
            State::Wall => panic!("Should not be on wall"),
            State::Space(s) => {
                // This should work, but it must add an edge case
                if match dir {
                    Direction::Up => {
                        (s.flags & 0b1000_0000 == 0b1000_0000) && (s.up_age < *intersection_age)
                    }
                    Direction::Right => {
                        (s.flags & 0b0100_0000 == 0b0100_0000) && (s.right_age < *intersection_age)
                    }
                    Direction::Down => {
                        (s.flags & 0b0010_0000 == 0b0010_0000) && (s.down_age < *intersection_age)
                    }
                    Direction::Left => {
                        (s.flags & 0b0001_0000 == 0b0001_0000) && (s.left_age < *intersection_age)
                    }
                } {
                    return true;
                }

                if s.temp_age < *age {
                    return false;
                }

                match dir {
                    Direction::Up => s.flags & 0b0000_1000 == 0b0000_1000,
                    Direction::Right => s.flags & 0b0000_0100 == 0b0000_0100,
                    Direction::Down => s.flags & 0b0000_0010 == 0b0000_0010,
                    Direction::Left => s.flags & 0b0000_0001 == 0b0000_0001,
                }
            }
        }
    }
    fn has_been_visited(&self) -> bool {
        match self {
            State::Wall => false,
            State::Space(s) => s.flags & 0b1111_0000 != 0b0000_0000,
        }
    }
}

#[derive(Copy, Clone)]
struct Guard {
    x: usize,
    y: usize,
    direction: Direction,
}

#[derive(Copy, Clone, Debug)]
enum StepError {
    Wall,
    Oob,
}

impl Guard {
    fn step(&mut self, grid: &[Vec<State>]) -> Result<(), StepError> {
        
        

        let (delta_x, delta_y) = self.direction.as_coords();

        let new_x = match self.x.checked_add_signed(delta_x) {
            Some(num) => num,
            None => return Err(StepError::Oob),
        };
        if new_x >= grid.len() {
            return Err(StepError::Oob);
        };

        let new_y = match self.y.checked_add_signed(delta_y) {
            Some(num) => num,
            None => return Err(StepError::Oob),
        };
        if new_y >= grid[0].len() {
            return Err(StepError::Oob);
        };

        if grid[new_x][new_y].is_wall() {
            return Err(StepError::Wall);
        }

        self.x = new_x;
        self.y = new_y;

        Ok(())
    }

    fn rotate_right(&mut self) {
        self.direction.rotate_right();
    }

    fn look(&self, grid: &mut [Vec<State>], age: &u32) -> bool {
        let mut dummy_guard = *self;
        match dummy_guard.step(grid) {
            Ok(_) => {
                let temp_x = dummy_guard.x;
                let temp_y = dummy_guard.y;

                if grid[temp_x][temp_x].is_temp_wall() {
                    return false;
                };

                let orig_state = grid[temp_x][temp_y];
                grid[temp_x][temp_y] = State::Wall;

                let mut intersection_age = *age + 1;

                if orig_state.has_been_visited() {
                    dummy_guard.direction = orig_state.get_first_traversal();
                    dummy_guard.rotate_right();
                    dummy_guard.rotate_right();
                    dummy_guard
                        .step(grid)
                        .expect("Shouldn't be able to backtrack to a wall");
                    dummy_guard.direction = orig_state.get_first_traversal();
                    intersection_age = orig_state.get_first_age();
                } else {
                    dummy_guard = *self;
                }

                loop {
                    match dummy_guard.step(grid) {
                        Err(StepError::Oob) => {
                            grid[temp_x][temp_y] = orig_state;
                            return false;
                        }
                        Err(StepError::Wall) => dummy_guard.rotate_right(),
                        _ => (),
                    }
                    if grid[dummy_guard.x][dummy_guard.y].check_traversal(
                        &dummy_guard.direction,
                        age,
                        &intersection_age,
                    ) {
                        grid[temp_x][temp_y] = orig_state;
                        grid[temp_x][temp_y].add_temp_wall();
                        return true;
                    }
                    grid[dummy_guard.x][dummy_guard.y]
                        .add_temp_traversal(&dummy_guard.direction, age);
                }
            }
            _ => false,
        }
    }
}
