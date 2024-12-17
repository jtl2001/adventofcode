pub fn run(input: &str, output: bool) {
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; input.find("\n").unwrap() + 2]; 1];
    for mut row in input
        .split_whitespace()
        .map(|s| s.chars().collect::<Vec<char>>())
    {
        let mut temp = vec!['.'];
        temp.append(&mut row);
        temp.push('.');
        grid.push(temp);
    }
    grid.push(grid[0].clone());

    for g in grid.iter() {
        debug_assert_eq!(g.len(), grid[0].len());
    }

    let mut checked: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    let mut cursor = Cursor::new(1, 1, 1, grid[0].len() - 2, grid.len() - 2);

    let mut full_price: u64 = 0;
    let mut discount_price: u64 = 0;

    while cursor.curr_index() < cursor.len() {
        let (x, y) = cursor.pos();
        debug_assert!(x > 0);
        debug_assert!(x < grid.len());
        debug_assert!(y > 0);
        debug_assert!(y < grid[x].len());

        if checked[x][y] {
            cursor.increment();
        } else {
            let curr_char = grid[x][y];
            let mut need_to_check: Vec<(usize, usize)> = Vec::new();
            need_to_check.push((x, y));
            let mut area: u64 = 0;
            let mut perimeter: u64 = 0;
            let mut sides: u64 = 0;

            while let Some((x, y)) = need_to_check.pop() {
                
                if !checked[x][y] {
                    checked[x][y] = true;
                    area += 1;

                    if grid[x - 1][y] == curr_char {
                        need_to_check.push((x - 1, y));
                    } else {
                        perimeter += 1;
                        if grid[x][y - 1] != curr_char || grid[x - 1][y - 1] == curr_char {
                            sides += 1;
                        }
                    }
                    if grid[x + 1][y] == curr_char {
                        need_to_check.push((x + 1, y));
                    } else {
                        perimeter += 1;
                        if grid[x][y + 1] != curr_char || grid[x + 1][y + 1] == curr_char {
                            sides += 1;
                        }
                    }
                    if grid[x][y - 1] == curr_char {
                        need_to_check.push((x, y - 1));
                    } else {
                        perimeter += 1;
                        if grid[x + 1][y] != curr_char || grid[x + 1][y - 1] == curr_char {
                            sides += 1;
                        }
                    }
                    if grid[x][y + 1] == curr_char {
                        need_to_check.push((x, y + 1));
                    } else {
                        perimeter += 1;
                        if grid[x - 1][y] != curr_char || grid[x - 1][y + 1] == curr_char {
                            sides += 1;
                        }
                    }
                }
            }

            full_price += area * perimeter;
            discount_price += area * sides;
        }
    }

    if output {
        println!("Part 1: {}", full_price);
        println!("Part 2: {}", discount_price);
    }
}

struct Cursor {
    x: usize,
    y: usize,
    buffer: usize,
    row_len: usize,
    max_len: usize,
}

impl Cursor {
    fn new(x: usize, y: usize, buffer: usize, row_len: usize, num_rows: usize) -> Cursor {
        Cursor {
            x,
            y,
            buffer,
            row_len,
            max_len: row_len * num_rows,
        }
    }
    fn curr_index(&self) -> usize {
        debug_assert!(self.x >= self.buffer);
        debug_assert!(self.y >= self.buffer);
        (self.x - self.buffer) * self.row_len + (self.y - self.buffer)
    }
    fn len(&self) -> usize {
        self.max_len
    }
    fn pos(&self) -> (usize, usize) {
        (self.x, self.y)
    }
    fn increment(&mut self) {
        self.y += 1;
        if self.y == self.row_len + self.buffer {
            self.y = self.buffer;
            self.x += 1;
        }
    }
}
