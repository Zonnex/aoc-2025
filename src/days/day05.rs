use crate::BLANK_LINE;

pub const INPUT: &str = include_str!("../inputs/05/real.txt");

pub fn solve(input: &str) -> (usize, u64) {
    let (fresh_ranges, ingredients) = parse_input(input);
    let p1 = ingredients
        .into_iter()
        .filter(|&i| {
            for range in &fresh_ranges {
                if range.contains(i) {
                    return true;
                }
            }
            false
        })
        .count();

    let p2 = fresh_ranges
        .into_iter()
        .map(|r| r.len())
        .sum();

    (p1, p2)
}

#[inline]
fn parse_input(input: &str) -> (Vec<Range>, Vec<u64>) {
    let (ranges, ingredients) = input.split_once(BLANK_LINE).unwrap();
    let mut ranges = ranges
        .lines()
        .map(|l| {
            let (min, max) = l.split_once('-').expect("line to match pattern {a}-{b}");
            Range {
                min: min.parse::<u64>().unwrap(),
                max: max.parse::<u64>().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    ranges.sort_by_key(|r| r.min);

    let mut result: Vec<Range> = Vec::new();
    for range in ranges {
        match result.last_mut() {
            Some(last) if last.max + 1 >= range.min => {
                last.max = last.max.max(range.max);
            }
            _ => {
                // No overlap - add as new range
                result.push(range);
            }
        }
    }

    let ingredients = ingredients
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    (result, ingredients)
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Range {
    min: u64,
    max: u64,
}

impl Range {
    #[inline]
    fn len(&self) -> u64 {
        self.max - self.min + 1
    }

    #[inline]
    fn contains(&self, v: u64) -> bool {
        self.min <= v && v <= self.max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = include_str!("../inputs/05/test.txt");

    #[test]
    fn example() {
        let (p1, p2) = solve(TEST);
        assert_eq!(p1, 3);
        assert_eq!(p2, 14);
    }
}
