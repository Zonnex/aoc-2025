use crate::util::grid::Grid;
use crate::util::hash::*;
use crate::util::point::*;

use itertools::Itertools as _;
use std::collections::VecDeque;

pub const INPUT: &str = include_str!("../inputs/09/real.txt");
const UNKNOWN: i64 = 2;
const INSIDE: i64 = 1;
const OUTSIDE: i64 = 0;

struct Point2D(u64, u64);

impl Point2D {
    fn parse(line: &str) -> Point2D {
        let (x, y) = line.split_once(',').unwrap();

        Point2D(x.parse().unwrap(), y.parse().unwrap())
    }
}

pub fn solve(input: &str) -> (u64, u64) {
    let points = input.lines().map(Point2D::parse).collect::<Vec<_>>();

    let p1 = p1(&points);
    let p2 = p2(&points);

    (p1, p2)
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

// 1. Compress: Add sentinel values (MIN/MAX) to coordinates, sort, dedup, create index mapping
// 2. Shrink points: Map original coordinates to compressed indices
// 3. Mark boundary: Draw polygon edges on compressed grid as INSIDE
// 4. Flood fill: From (0,0) mark all reachable cells as OUTSIDE
// 5. Prefix sums: Build 2D prefix sum of "inside" cells
// 6. Check pairs: For each vertex pair, use prefix sum to verify all cells in rectangle are inside, compute real area if valid
fn p2(points: &[Point2D]) -> u64 {
    let size = points.len();
    let (xs, ys) = compress(points);

    let shrunk: Vec<_> = points.iter().map(|p| (xs[&p.0], ys[&p.1])).collect();
    let mut grid = build_grid(size, xs, ys, &shrunk);

    for y in 1..grid.height {
        for x in 1..grid.width {
            let point = Point::new(x, y);
            let value = i64::from(grid[point] != OUTSIDE);
            grid[point] = value + grid[point + UP] + grid[point + LEFT] - grid[point + UP + LEFT];
        }
    }

    let mut area = 0;
    for i in 0..size {
        for j in i + 1..size {
            let (x1, y1, x2, y2) = minmax(shrunk[i], shrunk[j]);

            let expected = (x2 - x1 + 1) as i64 * (y2 - y1 + 1) as i64;

            let actual = grid[Point::new(x2, y2)]
                - grid[Point::new(x1 - 1, y2)]
                - grid[Point::new(x2, y1 - 1)]
                + grid[Point::new(x1 - 1, y1 - 1)];

            if expected == actual {
                let Point2D(px1, py1) = points[i];
                let Point2D(px2, py2) = points[j];
                let dx = px1.abs_diff(px2) + 1;
                let dy = py1.abs_diff(py2) + 1;
                area = area.max(dx * dy);
            }
        }
    }

    area
}

fn build_grid(
    size: usize,
    xs: FastMap<u64, i32>,
    ys: FastMap<u64, i32>,
    shrunk: &Vec<(i32, i32)>,
) -> Grid<i64> {
    let mut grid = Grid::new(xs.len() as i32, ys.len() as i32, UNKNOWN);

    // Draw edges between consecutive vertices (polygon boundary)
    for i in 0..size {
        let (x1, y1, x2, y2) = minmax(shrunk[i], shrunk[(i + 1) % size]);
        for x in x1..=x2 {
            for y in y1..=y2 {
                grid[Point::new(x, y)] = INSIDE;
            }
        }
    }

    // Step 3: Flood fill from origin (0,0) to mark OUTSIDE
    let mut todo = VecDeque::from([ORIGIN]);
    grid[ORIGIN] = OUTSIDE;

    while let Some(point) = todo.pop_front() {
        for next in ORTHOGONAL.map(|o| point + o) {
            if grid.contains(next) && grid[next] == UNKNOWN {
                grid[next] = OUTSIDE;
                todo.push_back(next);
            }
        }
    }
    grid
}

fn compress(points: &[Point2D]) -> (FastMap<u64, i32>, FastMap<u64, i32>) {
    let mut xs: Vec<_> = points.iter().map(|points| points.0).collect();
    xs.push(u64::MIN);
    xs.push(u64::MAX);
    xs.sort_unstable();
    xs.dedup();
    let xs = xs.iter().enumerate().map(|(i, &n)| (n, i as i32)).collect();

    let mut ys: Vec<_> = points.iter().map(|points| points.1).collect();
    ys.push(u64::MIN);
    ys.push(u64::MAX);
    ys.sort_unstable();
    ys.dedup();
    let ys = ys.iter().enumerate().map(|(i, &n)| (n, i as i32)).collect();

    (xs, ys)
}

#[inline]
fn minmax((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> (i32, i32, i32, i32) {
    (x1.min(x2), y1.min(y2), x1.max(x2), y1.max(y2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = include_str!("../inputs/09/test.txt");

    #[test]
    fn test_with_example() {
        let (p1, p2) = solve(TEST);
        assert_eq!(p1, 50);
        assert_eq!(p2, 24); // Update with correct expected value
    }
}
