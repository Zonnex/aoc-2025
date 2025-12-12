use itertools::Itertools;

pub const INPUT: &str = include_str!("../inputs/09/real.txt");

struct Point2D(u64, u64);

impl Point2D {
    fn parse(line: &str) -> Point2D {
        let (x, y) = line.split_once(',').unwrap();

        Point2D(x.parse().unwrap(), y.parse().unwrap())
    }
}

pub fn solve(input: &str) -> (u64, i32) {
    let points = input.lines().map(Point2D::parse).collect::<Vec<_>>();

    let p1 = p1(&points);

    (p1, 0)
}

fn p1(points: &[Point2D]) -> u64 {
    points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let w = u64::abs_diff(a.0, b.0) + 1;
            let h = u64::abs_diff(a.1, b.1) + 1;
            
            w * h
        })
        .max()
        .unwrap()
}

fn p2(points: &[Point2D]) -> u64 {
    // learn coordinate compression

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = include_str!("../inputs/09/test.txt");

    #[test]
    fn test_with_example() {
        let (p1, p2) = solve(TEST);
        assert_eq!(p1, 50);
        assert_eq!(p2, 0);
    }
}
