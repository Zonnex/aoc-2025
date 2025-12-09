use itertools::Itertools;
use std::{collections::HashSet, ops::Sub};

use crate::util::heap::MinHeap;

pub const INPUT: &str = include_str!("../inputs/08/real.txt");

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Point3D(i64, i64, i64);

impl Point3D {
    fn parse(line: &str) -> Point3D {
        let mut iter = line.split(',').flat_map(str::parse);
        Point3D(
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
    }

    fn distance_sq(self, other: Self) -> u64 {
        let d = self - other;
        (d.0 * d.0 + d.1 * d.1 + d.2 * d.2) as u64
    }
}

impl Sub for Point3D {
    type Output = Point3D;

    fn sub(self, rhs: Self) -> Self::Output {
        Point3D(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

pub fn solve(input: &str) -> (usize, i64) {
    run::<1000>(input)
}

fn run<const PAIRS: usize>(input: &str) -> (usize, i64) {
    let points: Vec<_> = input.lines().map(Point3D::parse).collect();

    let mut heap = MinHeap::with_capacity(points.len() * points.len() / 2);
    for (a, b) in points.iter().tuple_combinations() {
        heap.push(a.distance_sq(*b), (*a, *b));
    }

    let mut circuits: Vec<HashSet<Point3D>> = points.into_iter()
        .map(|p| HashSet::from([p]))
        .collect();

    let mut p1 = 0;
    let mut p2 = 0;
    let mut pops = 0;
    loop {
        let Some((_, (a, b))) = heap.pop() else {
            break;
        };
        pops += 1;

        let circuit_a = circuits.iter().position(|c| c.contains(&a));
        let circuit_b = circuits.iter().position(|c| c.contains(&b));

        match (circuit_a, circuit_b) {
            (Some(ia), Some(ib)) if ia == ib => {}
            (Some(ia), Some(ib)) => {
                let (keep, remove) = (ia.min(ib), ia.max(ib));
                let merged = circuits.remove(remove);
                circuits[keep].extend(merged);
            }
            (Some(idx), None) => {
                circuits[idx].insert(b);
            }
            (None, Some(idx)) => {
                circuits[idx].insert(a);
            }
            (None, None) => {
                circuits.push(HashSet::from([a, b]));
            }
        }

        if pops == PAIRS {
            circuits.sort_by_key(|c| c.len());
            p1 = circuits.iter().rev().take(3).map(|c| c.len()).product();
        }

        if circuits.len() == 1 {
            p2 = a.0 * b.0;
            break;
        }
    }
    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = include_str!("../inputs/08/test.txt");

    #[test]
    fn test_with_example() {
        let (p1, p2) = run::<10>(TEST);
        assert_eq!(p1, 40);
        assert_eq!(p2, 25272);
    }
}
