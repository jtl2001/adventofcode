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
                let mut test_list: Vec<usize> = vec![index, index - 1];
                if index > 1 {
                    test_list.push(index - 2);
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
    let logical_test: fn(u32, u32) -> bool;

    if report[1] > report[0] {
        logical_test = |c, p| c > p;
    } else if report[0] > report[1] {
        logical_test = |c, p| p > c;
    } else {
        return Option::Some(1);
    }

    for i in 1..report.len() {
        if !logical_test(report[i], report[i - 1]) {
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
