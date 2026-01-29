mod days;
mod util;

use std::env;

#[cfg(unix)]
const BLANK_LINE: &str = "\n\n";

#[cfg(windows)]
const BLANK_LINE: &str = "\r\n\r\n";

fn main() {
    let args = env::args().collect::<Vec<_>>();
    println!("Advent of Code 2025 Solutions");
    println!("=============================\n");
    if args.len() > 1 {
        println!("Running days: {:?}", &args[1..]);
    } else {
        println!("Running all days");
    }

    let days: Vec<u8> = if args.len() > 1 {
        args[1..]
            .iter()
            .map(|x| {
                x.parse()
                    .unwrap_or_else(|v| panic!("Not a valid day: {}", v))
            })
            .collect()
    } else {
        (1..=25).collect()
    };

    let mut runtime = 0.0;

    for day in days {
        runtime += days::solve_day(day);
    }

    println!("Total runtime: {:.4} ms", runtime);
}

