use std::cmp::min;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::mem;

pub fn run(input: &str, output: bool) {
    let input: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    // cursors to current front and back
    // they will start at the ends and make their way towards each other
    let mut start_index = 1;
    let mut end_index = input.len() - 1;
    // value of the current end index
    // used to "subtract" from the last number, as input[end_index] -= foo,
    // but without actually changing the input array
    let mut curr_end = input[end_index];
    // index in the full disk array, but we never actually need to allocate that array,
    // just need to know our theoretical position in it
    let mut unwrapped_index = input[0];

    let mut part_1_result: u64 = 0;
    loop {
        // while start_index < end_index
        match start_index.cmp(&end_index) {
            Ordering::Less => {
                if start_index % 2 == 0 {
                    // start index is on a file, can safely add entire file to the accumulation
                    part_1_result += (start_index as u64) / 2
                        * sum_of_run(unwrapped_index, input[start_index]);
                    unwrapped_index += input[start_index];
                    start_index += 1
                } else {
                    // start index on empty space, run is the remaining length of the empty space
                    let mut run = input[start_index];
                    while run > 0 {
                        // find next file if needed
                        while curr_end == 0 {
                            end_index -= 2;
                            curr_end = input[end_index];
                        }
                        // handle edge case where end index can overtake the start at the end of the computation
                        if end_index <= start_index {
                            break;
                        }
                        // add chunk length
                        let chunk_size = min(run, curr_end);
                        part_1_result +=
                            (end_index as u64) / 2 * sum_of_run(unwrapped_index, chunk_size) as u64;
                        unwrapped_index += chunk_size;
                        run -= chunk_size;
                        curr_end -= chunk_size;
                    }
                    start_index += 1;
                }
            }
            Ordering::Equal => {
                // edge case where start index is the end index on the last loop
                part_1_result += (start_index as u64) / 2 * sum_of_run(unwrapped_index, curr_end);
                break;
            }
            Ordering::Greater => break,
        }
    }

    let mut files = Vec::new();
    let mut spaces: Vec<Vec<Space>> = vec![Vec::new(); 9];
    let mut unwrapped_index = 0;

    for i in 0..input.len() {
        if i % 2 == 0 {
            files.push(File::new(i / 2, input[i], unwrapped_index));
            unwrapped_index += input[i];
        } else {
            let mut size = input[i];
            // files may be 0 length, if so the spaces should combine
            let mut temp = i + 1;
            while input[temp] == 0 {
                size += input[temp + 1];
                temp += 2;
            }

            if size >= 9 {
                spaces[8].push(Space::new(size, unwrapped_index));
            } else if size > 0 {
                spaces[size - 1].push(Space::new(size, unwrapped_index));
            }
            unwrapped_index += size;
        }
    }

    let mut files = BinaryHeap::from(mem::take(&mut files));
    let mut spaces: Vec<BinaryHeap<Space>> = spaces
        .iter_mut()
        .map(|s| BinaryHeap::from(mem::take(s)))
        .collect();

    let mut part_2_result: u64 = 0;

    while !files.is_empty() {
        let mut f = files.pop().unwrap();
        match next_space_index(f.size, &spaces) {
            Some(heap_index) => {
                let mut s = spaces[heap_index].pop().unwrap();
                debug_assert!(s.size >= f.size);

                if f.index > s.index {
                    f.index = s.index;
                    files.push(f);

                    if s.size > f.size {
                        s.size -= f.size;
                        s.index += f.size;
                        if s.size > 8 {
                            spaces[8].push(s);
                        } else {
                            spaces[s.size - 1].push(s);
                        }
                    }
                } else {
                    part_2_result += (f.name as u64) * sum_of_run(f.index, f.size);
                }
            }
            None => {
                part_2_result += (f.name as u64) * sum_of_run(f.index, f.size);
            }
        }
    }

    if output {
        println!("Part 1: {}", part_1_result);
        println!("Part 1: {}", part_2_result);
    }
}

fn sum_of_run(start: usize, len: usize) -> u64 {
    // start with n*(n-1)/2 for sum 1..n, then subtract m*(m-1)/2
    // follow through the algebra
    (len * (2 * start + len - 1) / 2) as u64
}

fn next_space_index(min_size: usize, spaces: &[BinaryHeap<Space>]) -> Option<usize> {
    spaces
        .iter()
        .enumerate()
        .filter_map(|(index, h)| match h.peek() {
            None => None,
            Some(s) => {
                if s.size >= min_size {
                    return Some((index, h));
                }
                None
            }
        })
        .max_by_key(|(_, h)| h.peek().unwrap())
        .map(|(i, _)| i)
}

#[derive(Debug, Clone, Copy)]
struct File {
    name: usize,
    size: usize,
    index: usize,
}

#[derive(Debug, Clone, Copy)]
struct Space {
    size: usize,
    index: usize,
}

impl File {
    fn new(name: usize, size: usize, index: usize) -> File {
        File { name, size, index }
    }
}

impl Space {
    fn new(size: usize, index: usize) -> Space {
        Space { size, index }
    }
}

// Files at larger index should be compared as greater
impl Ord for File {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl PartialOrd for File {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for File {}

// Spaces at larger index should be compared as smaller
impl Ord for Space {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.index.cmp(&self.index)
    }
}

impl PartialOrd for Space {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Space {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for Space {}
