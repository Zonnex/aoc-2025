use std::collections::HashSet;

pub const INPUT: &str = include_str!("../inputs/02/real.txt");

pub fn solve(input: &str) -> (u64, u64) {
    let p1 = p1(input);
    let p2 = p2(input);

    (p1, p2)
}

type Range = (u64, u64);

fn p1(input: &str) -> u64 {
    input
        .split(',')
        .map(|range| range.split_once('-').expect("valid range"))
        .map(|(min, max)| (min.parse().unwrap(), max.parse().unwrap()))
        .map(shrink_range)
        .map(|(min, max)| {
            if min >= max {
                return 0;
            }
            let mut current = min / 10_u64.pow(digits(min) / 2);
            let mut sum = 0;
            loop {
                let d = digits(current);
                let power = 10_u64.pow(d);
                let repeated = current * power + current;

                if max < repeated {
                    break;
                }

                if repeated < min {
                    current += 1;
                    continue;
                }

                if repeated <= max {
                    sum += repeated;
                }
                current += 1;
            }
            sum
        })
        .sum::<u64>()
}

fn shrink_range((min, max): Range) -> (u64, u64) {
    let d_min = digits(min);
    let min = match d_min % 2 == 1 {
        true => 10_u64.pow(d_min),
        false => min,
    };

    let d_max = digits(max);
    let max = match d_max % 2 == 1 {
        true => 10_u64.pow(d_max - 1) - 1,
        false => max,
    };

    (min, max)
}

fn digits(x: u64) -> u32 {
    if x == 0 {
        return 1;
    }
    x.ilog10() + 1
}

fn p2(input: &str) -> u64 {

    input
        .split(',')
        .map(|line| {
            let (l, r) = line.split_once('-').unwrap();
            let l = l.parse().unwrap();
            let r = r.parse().unwrap();

            let mut seen = HashSet::new();
            for base_digits in 1..=10 {
                let base_min = 10_u64.pow(base_digits - 1).max(1);
                let base_max = 10_u64.pow(base_digits);

                for reps in 2..=20 / base_digits {
                    for base in base_min..base_max {
                        let num: u64 = base.to_string().repeat(reps as usize).parse().unwrap();
                        if num > r { break; }
                        if num >= l { seen.insert(num); }
                    }
                }
            }
            seen.into_iter().sum::<u64>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = include_str!("../inputs/02/test.txt");

    #[test]
    fn test_with_example() {
        let (p1, p2) = solve(TEST);
        assert_eq!(p1, 1227775554);
        assert_eq!(p2, 4174379265);
    }
}
