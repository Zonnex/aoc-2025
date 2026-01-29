use crate::util::hash::{FastSet, FastSetBuilder as _};
use crate::util::point::{self, Point};

pub const INPUT: &str = include_str!("../inputs/04/real.txt");

fn is_unstable(p: Point, set: &FastSet<Point>) -> bool {
    let neighbors = point::DIAGONAL
        .iter()
        .filter(|&&d| set.contains(&(p + d)))
        .count();
    
    neighbors < 4
}

pub fn solve(input: &str) -> (usize, usize) {
    let mut set = parse(input);
    let initial = set.len();

    let mut todo: FastSet<Point> = set.iter().copied().collect();
    let mut p1 = 0;
    
    while !todo.is_empty() {
        let unstable = todo
            .drain()
            .filter(|&p| set.contains(&p) && is_unstable(p, &set))
            .collect::<Vec<_>>();

        if unstable.is_empty() {
            break;
        }
        if p1 == 0 {
            p1 = unstable.len();
        }

        for &p in &unstable {
            set.remove(&p);
            todo.extend(
                point::DIAGONAL
                    .iter()
                    .map(|&d| p + d)
                    .filter(|n| set.contains(n)),
            );
        }
    }
    let p2 = initial - set.len();

    (p1, p2)
}

fn parse(input: &str) -> FastSet<Point> {
    let mut set: FastSet<Point> = FastSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, b) in line.bytes().enumerate() {
            if b == b'@' {
                set.insert(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }
    set
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = include_str!("../inputs/04/test.txt");

    #[test]
    fn test_with_example() {
        let (p1, p2) = solve(TEST);
        assert_eq!(p1, 13);
        assert_eq!(p2, 43);
    }
}
