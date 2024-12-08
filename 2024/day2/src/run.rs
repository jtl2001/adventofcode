use std::cmp::Ordering;

pub fn run(input: &str, output: bool) {

    let reports = input.split("\n").map(|r| {
        r.trim()
            .split(" ")
            .map(|l| l.parse::<u32>().expect("NaN"))
            .collect::<Vec<u32>>()
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

                if test_list.iter().any(|i| test_saftey_remove(&r, i)) {
                    safe_reports_lazy += 1;
                }
            }
        }
    }

    if output {
        println!("Part 1: {safe_reports_strict}");
        println!("Part 2: {safe_reports_lazy}");
    }
}

fn test_saftey(report: &[u32]) -> Option<usize> {
    let monotone_test: fn(u32, u32) -> bool = match report[1].cmp(&report[0]) {
        Ordering::Greater => |curr, prev| curr > prev,
        Ordering::Less => |curr, prev| prev > curr,
        Ordering::Equal => return Option::Some(1),
    };

    for i in 1..report.len() {
        if !monotone_test(report[i], report[i - 1]) {
            return Option::Some(i);
        }
        let diff = report[i].abs_diff(report[i - 1]);
        if !(1..=3).contains(&diff) {
            return Option::Some(i);
        }
    }
    Option::None
}

fn test_saftey_remove(report: &[u32], index: &usize) -> bool {
    let mut variant = report.to_owned();
    variant.remove(*index);
    if test_saftey(&variant).is_none() {
        return true;
    }
    false
}
