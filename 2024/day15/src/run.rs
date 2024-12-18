use std::collections::VecDeque;

pub fn run(input: &str, output: bool) {
    let mut grid: Vec<Vec<State>> = Vec::new();
    let mut double_grid: Vec<Vec<State>> = Vec::new();
    let mut instructions: Vec<char> = Vec::new();
    let mut cursor_1 = Cursor { x: 0, y: 0 };
    let mut cursor_2 = Cursor { x: 0, y: 0 };

    {
        let split_index = input.find("\n\n").unwrap();
        let (input_maze, input_instructions) = input.split_at(split_index);

        for r in input_maze.split_whitespace() {
            let mut temp = Vec::new();
            let mut double_temp = Vec::new();
            for c in r.chars() {
                match c {
                    '.' => {
                        temp.push(State::Space);
                        double_temp.push(State::Space);
                        double_temp.push(State::Space);
                    }
                    '#' => {
                        temp.push(State::Wall);
                        double_temp.push(State::Wall);
                        double_temp.push(State::Wall);
                    }
                    'O' => {
                        temp.push(State::Box);
                        double_temp.push(State::Box);
                        double_temp.push(State::BoxRight);
                    }
                    '@' => {
                        cursor_1 = Cursor {
                            x: grid.len(),
                            y: temp.len(),
                        };
                        temp.push(State::Space);
                        cursor_2 = Cursor {
                            x: double_grid.len(),
                            y: double_temp.len(),
                        };
                        double_temp.push(State::Space);
                        double_temp.push(State::Space);
                    }
                    _ => (),
                }
            }
            grid.push(temp);
            double_grid.push(double_temp);
        }

        for r in input_instructions.split_whitespace() {
            for c in r.chars() {
                instructions.push(c);
            }
        }
    }

    for i in instructions {
        let dir: (isize, isize) = match i {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => panic!("unrecognized instruction"),
        };

        let (next_x, next_y) = (
            cursor_1.x.checked_add_signed(dir.0).unwrap(),
            cursor_1.y.checked_add_signed(dir.1).unwrap(),
        );
        if grid[next_x][next_y] == State::Space {
            cursor_1.x = next_x;
            cursor_1.y = next_y;
        } else {
            let (mut look_x, mut look_y) = (next_x, next_y);

            while grid[look_x][look_y] == State::Box {
                look_x = look_x.checked_add_signed(dir.0).unwrap();
                look_y = look_y.checked_add_signed(dir.1).unwrap();
            }

            if grid[look_x][look_y] == State::Space {
                grid[look_x][look_y] = State::Box;
                grid[next_x][next_y] = State::Space;
                cursor_1.x = next_x;
                cursor_1.y = next_y;
            }
        }

        let (next_x, next_y) = (
            cursor_2.x.checked_add_signed(dir.0).unwrap(),
            cursor_2.y.checked_add_signed(dir.1).unwrap(),
        );
        match double_grid[next_x][next_y] {
            State::Space => {
                cursor_2.x = next_x;
                cursor_2.y = next_y;
            }
            State::Wall => continue,
            _ => {
                let mut checklist = VecDeque::new();
                let mut checked = Vec::new();
                if double_grid[next_x][next_y] == State::Box {
                    checklist.push_back((next_x, next_y));
                } else {
                    checklist.push_back((next_x, next_y - 1));
                }

                let mut hitwall = false;
                while !checklist.is_empty() {
                    let (box_x, box_y) = checklist.pop_front().unwrap();

                    if dir.1 != 1 {
                        // check left
                        match double_grid[box_x.checked_add_signed(dir.0).unwrap()]
                            [box_y.checked_add_signed(dir.1).unwrap()]
                        {
                            State::Space => (),
                            State::Wall => {
                                hitwall = true;
                                break;
                            }
                            State::Box => checklist.push_back((
                                box_x.checked_add_signed(dir.0).unwrap(),
                                box_y.checked_add_signed(dir.1).unwrap(),
                            )),
                            State::BoxRight => checklist.push_back((
                                box_x.checked_add_signed(dir.0).unwrap(),
                                box_y.checked_add_signed(dir.1 - 1).unwrap(),
                            )),
                        }
                    }
                    if dir.1 != -1 {
                        // check right
                        match double_grid[box_x.checked_add_signed(dir.0).unwrap()]
                            [box_y.checked_add_signed(dir.1).unwrap() + 1]
                        {
                            State::Space => (),
                            State::Wall => {
                                hitwall = true;
                                break;
                            }
                            State::Box => checklist.push_back((
                                box_x.checked_add_signed(dir.0).unwrap(),
                                box_y.checked_add_signed(dir.1).unwrap() + 1,
                            )),
                            State::BoxRight => (),
                        }
                    }

                    checked.push((box_x, box_y));
                }

                if hitwall {
                    continue;
                }

                while let Some((box_x, box_y)) = checked.pop() {
                    
                    double_grid[box_x][box_y] = State::Space;
                    double_grid[box_x][box_y + 1] = State::Space;
                    double_grid[box_x.checked_add_signed(dir.0).unwrap()]
                        [box_y.checked_add_signed(dir.1).unwrap()] = State::Box;
                    double_grid[box_x.checked_add_signed(dir.0).unwrap()]
                        [box_y.checked_add_signed(dir.1).unwrap() + 1] = State::BoxRight;
                }

                cursor_2.x = next_x;
                cursor_2.y = next_y;
            }
        }
    }

    let mut gps_sum = 0;
    for (i, r) in grid.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if *c == State::Box {
                gps_sum += 100 * i + j;
            }
        }
    }

    let mut double_gps_sum = 0;
    for (i, r) in double_grid.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if *c == State::Box {
                double_gps_sum += 100 * i + j;
            }
        }
    }

    if output {
        println!("Part 1: {}", gps_sum);
        println!("Part 2: {}", double_gps_sum);
    }
}

#[derive(Debug, PartialEq, Eq)]
enum State {
    Wall,
    Space,
    Box,
    BoxRight,
}

#[derive(Debug)]
struct Cursor {
    x: usize,
    y: usize,
}
