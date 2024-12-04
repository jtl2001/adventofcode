use std::fs;

fn main() {
    let input = fs::read_to_string(".\\src\\input.txt").expect("Failed to read file");
    let input = input.trim();

    let reports = input.split("\n").map(|r| {
        r.trim()
            .split(" ")
            .map(|l| l.parse::<u32>().expect("NaN"))
            .collect()
    });

    let mut safe_reports_strict: u32 = 0;
    let mut safe_reports_lazy: u32 = 0;

    for r in reports {
        match test_saftey(&r) {
            Option::None => {
                safe_reports_strict += 1;
                safe_reports_lazy += 1;
            }
            Option::Some(index) => {
                // test in order of most likely to fix
                let mut test_list: Vec<usize> = vec![index - 1, index];

                // Edge case where index 2 can fail if the first index causes the
                // wrong monotonicity to be detected
                if index == 2 {
                    test_list.push(0);
                }

                if test_list.iter().any(|i| test_saftey_remove(&r, &i)) {
                    safe_reports_lazy += 1;
                }
            }
        }
    }

    println!("Part 1: {safe_reports_strict}");
    println!("Part 2: {safe_reports_lazy}");
}

fn test_saftey(report: &Vec<u32>) -> Option<usize> {
    let monotone_test: fn(u32, u32) -> bool;

    if report[1] > report[0] {
        monotone_test = |curr, prev| curr > prev;
    } else if report[0] > report[1] {
        monotone_test = |curr, prev| prev > curr;
    } else {
        return Option::Some(1);
    }

    for i in 1..report.len() {
        if !monotone_test(report[i], report[i - 1]) {
            return Option::Some(i);
        }
        let diff = report[i].abs_diff(report[i - 1]);
        if diff < 1 || diff > 3 {
            return Option::Some(i);
        }
    }
    return Option::None;
}

fn test_saftey_remove(report: &Vec<u32>, index: &usize) -> bool {
    let mut variant = report.clone();
    variant.remove(*index);
    if test_saftey(&variant) == Option::None {
        return true;
    }
    return false;
}
