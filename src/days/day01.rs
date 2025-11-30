pub const INPUT: &str = include_str!("../inputs/01/real.txt");

pub fn solve(input: &str) -> (i32, i32) {
    let mut p1 = 0;
    let mut p2 = 0;
    let mut dial = 50;
    for line in input.split("\n") {
        let delta = parse_line(line);

        if delta >= 0 {
            p2 += (dial + delta) / 100;
        } else {
            let reversed = (100 - dial).rem_euclid(100);
            p2 += (reversed - delta) / 100;
        }
        dial = (dial + delta).rem_euclid(100);
        if dial == 0 {
            p1 += 1;
        }
    }

    (p1, p2)
}

fn parse_line(line: &str) -> i32 {
    let bytes = line.as_bytes();
    let sign = match bytes[0] {
        b'L' => -1,
        b'R' => 1,
        _ => unreachable!(),
    };
    let steps = line[1..].parse::<i32>().expect("steps to be a number");
    sign * steps
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = include_str!("../inputs/01/test.txt");

    #[test]
    fn test_with_example() {
        let (p1, p2) = solve(TEST);
        assert_eq!(p1, 3);
        assert_eq!(p2, 6);
    }
}
