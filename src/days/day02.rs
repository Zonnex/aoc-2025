pub const INPUT: &str = include_str!("../inputs/02/real.txt");

type Pattern = [u32; 2];

// Repeating patterns: [digits, base]
// Pattern repeated exactly twice: [2,1]=55, [4,2]=1212, [6,3]=123123, etc.
const P1_PATTERNS: [Pattern; 5] = [[2, 1], [4, 2], [6, 3], [8, 4], [10, 5]];

// Pattern repeated 3+ times: [3,1]=111, [6,2]=121212, etc.
const P2_PATTERNS: [Pattern; 6] = [[3, 1], [5, 1], [6, 2], [7, 1], [9, 3], [10, 2]];

// Patterns to subtract (inclusion-exclusion for double counting)
// e.g. 111111 is both [6,3] (111 twice) and [6,1] (1 six times)
const EXCLUSION: [Pattern; 2] = [[6, 1], [10, 1]];

struct Range {
    min: u64,
    max: u64,
}

impl Range {
    fn parse(input: &str) -> Range {
        let (l, r) = input.split_once('-').unwrap();
        Range {
            min: l.parse().unwrap(),
            max: r.parse().unwrap(),
        }
    }
}

pub fn solve(input: &str) -> (u64, u64) {
    let ranges = input.split(',').map(Range::parse).collect::<Vec<_>>();

    let p1 = compute(&P1_PATTERNS, &ranges);
    let p2 = p1 + compute(&P2_PATTERNS, &ranges) - compute(&EXCLUSION, &ranges);

    (p1, p2)
}

fn compute(patterns: &[Pattern], ranges: &[Range]) -> u64 {
    let mut sum = 0;

    for &[digits, base_digits] in patterns {
        let digits = u64::pow(10, digits);
        let base = u64::pow(10, base_digits);

        let step = (digits - 1) / (base - 1);
        let start = step * (base / 10);
        let end = step * (base - 1);

        for r in ranges {
            let lower = u64::max(r.min.next_multiple_of(step), start);
            let upper = u64::min(r.max, end);

            if lower <= upper {
                let n = (upper - lower) / step;
                let triangular = n * (n + 1) / 2;
                sum += lower * (n + 1) + step * triangular;
            }
        }
    }

    sum
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
