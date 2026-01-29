use std::time::Instant;

/// Macro to declare all days and generate the `solve_day` dispatcher.
///
/// Each day module must export:
/// - `pub const INPUT: &str` - the puzzle input
/// - `pub fn solve(input: &str) -> (A, B)` where A, B: Display
///
/// Adding a new day = add one identifier to this list.
macro_rules! days {
    ($($day:ident),* $(,)?) => {
        $(mod $day;)*

        pub fn solve_day(day: u8) -> f64 {
            let time = Instant::now();
            match day {
                $(days!(@num $day) => {
                    let (p1, p2) = $day::solve($day::INPUT);
                    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
                    println!("\n=== Day {:02} ===", days!(@num $day));
                    println!("  · Part 1: {}", p1);
                    println!("  · Part 2: {}", p2);
                    println!("  · Elapsed: {:.4} ms", elapsed_ms);
                    elapsed_ms
                },)*
                _ => 0.0,
            }
        }
    };
    // Helper to extract day number from identifier like day01 -> 1
    (@num day01) => { 1 };
    (@num day02) => { 2 };
    (@num day03) => { 3 };
    (@num day04) => { 4 };
    (@num day05) => { 5 };
    (@num day06) => { 6 };
    (@num day07) => { 7 };
    (@num day08) => { 8 };
    (@num day09) => { 9 };
    (@num day10) => { 10 };
    (@num day11) => { 11 };
    (@num day12) => { 12 };
}

days!(
    day01,
    day02,
    day03,
    day04,
    day05,
    day06,
    day07,
    day08,
    day09,
    day10,
    day11,
    day12,
);


