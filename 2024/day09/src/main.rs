use std::fs::read_to_string;
mod run;
use run::run;
use std::time::Instant;

fn main() {
    let input = read_to_string(".\\src\\input.txt").expect("Failed to read file");
    let input = input.trim();

    run(input, true);

    let start = Instant::now();
    let num_reps: u32 = 1000;

    if true {
        for _i in 0..num_reps {
            run(input, false);
        }

        let end = Instant::now();
        let time = (end - start) / num_reps;
        println!(
            "Average runtime over {} runs: {}",
            print_thousands_separator(num_reps),
            print_time_units(time.as_secs_f64())
        );
    }
}

fn print_time_units(mut time: f64) -> String {
    let units = ["s", "ms", "us", "ns", "ps", "fs", "as"];

    let mut i = 0;
    while time < 1.0 && i < units.len() - 1 {
        i += 1;
        time *= 1000.0;
    }

    format!("{:.2} {}", time, units[i])
}

fn print_thousands_separator(mut num: u32) -> String {
    let mut string = String::new();

    while num >= 1000 {
        string = format!(",{:03}{}", num % 1000, string);
        num /= 1000;
    }

    format!("{}{}", num, string)
}
