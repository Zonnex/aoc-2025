//! # Playground
//!
//! Kruskal's algorithm to build a minimum spanning tree by processing edges
//! (pairs of points) in order of increasing distance.
//!
//! ## Optimization: Bucket Sort
//! Instead of using a min-heap for all O(n²) pairs, we bucket pairs by distance
//! into 5 ranges. Each bucket is sorted independently. This avoids heap overhead
//! during pair generation and exploits the fact that we only need approximate
//! ordering until we actually process the pairs.
//!
//! ## Union-Find
//! Track connected components with union-find (disjoint set) with path compression
//! and union-by-size for near O(1) amortized operations.
//!
//! - Part 1: Product of the 3 largest component sizes after 1000 edges processed
//! - Part 2: Product of x-coordinates when all points merge into one component

use crate::util::iter::ChunkOps as _;
use crate::util::parse::ParseOps as _;

pub const INPUT: &str = include_str!("../inputs/08/real.txt");

type Point = [usize; 3];
type Pair = (u16, u16, usize);

const BUCKETS: usize = 5;
const BUCKET_SIZE: usize = 100_000_000; // 10_000^2

pub fn solve(input: &str) -> (usize, usize) {
    let points: Vec<Point> = input.iter_unsigned().chunk::<3>().collect();
    let mut buckets: Vec<Vec<Pair>> = vec![vec![]; BUCKETS];

    for (i, &p1) in points.iter().enumerate() {
        for (j, &p2) in points.iter().enumerate().skip(i + 1) {
            let dx = p1[0].abs_diff(p2[0]);
            let dy = p1[1].abs_diff(p2[1]);
            let dz = p1[2].abs_diff(p2[2]);
            let dist = dx * dx + dy * dy + dz * dz;
            let bucket = (dist / BUCKET_SIZE).min(BUCKETS - 1);
            buckets[bucket].push((i as u16, j as u16, dist));
        }
    }

    // Sort each bucket by distance
    for bucket in &mut buckets {
        bucket.sort_unstable_by_key(|&(.., d)| d);
    }

    let (p1, p2) = run::<1000>(&points, &buckets);
    (p1, p2)
}

fn run<const PAIRS: usize>(points: &[Point], buckets: &[Vec<Pair>]) -> (usize, usize) {
    let n = points.len();
    let mut parent: Vec<usize> = (0..n).collect();
    let mut size: Vec<usize> = vec![1; n];

    let mut p1 = 0;
    let mut pops = 0;

    for &(i, j, _) in buckets.iter().flat_map(|b| b.iter()) {
        let (i, j) = (i as usize, j as usize);
        pops += 1;

        let merged_size = union(&mut parent, &mut size, i, j);

        if pops == PAIRS {
            let mut sizes: Vec<_> = (0..n).filter(|&x| parent[x] == x).map(|x| size[x]).collect();
            sizes.sort_unstable_by(|a, b| b.cmp(a));
            p1 = sizes.iter().take(3).product();
        }

        if merged_size == n {
            return (p1, points[i][0] * points[j][0]);
        }
    }

    (p1, 0)
}

fn find(parent: &mut [usize], mut x: usize) -> usize {
    while parent[x] != x {
        let p = parent[x];
        parent[x] = parent[p]; // path compression
        x = p;
    }
    x
}

fn union(parent: &mut [usize], size: &mut [usize], x: usize, y: usize) -> usize {
    let (mut rx, mut ry) = (find(parent, x), find(parent, y));
    if rx != ry {
        if size[rx] < size[ry] {
            (rx, ry) = (ry, rx);
        }
        parent[ry] = rx;
        size[rx] += size[ry];
    }
    size[rx]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = include_str!("../inputs/08/test.txt");

    #[test]
    fn test_with_example() {
        let points: Vec<Point> = TEST.iter_unsigned().chunk::<3>().collect();
        let mut buckets: Vec<Vec<Pair>> = vec![vec![]; BUCKETS];

        for (i, &p1) in points.iter().enumerate() {
            for (j, &p2) in points.iter().enumerate().skip(i + 1) {
                let dx = p1[0].abs_diff(p2[0]);
                let dy = p1[1].abs_diff(p2[1]);
                let dz = p1[2].abs_diff(p2[2]);
                let dist = dx * dx + dy * dy + dz * dz;
                let bucket = (dist / BUCKET_SIZE).min(BUCKETS - 1);
                buckets[bucket].push((i as u16, j as u16, dist));
            }
        }
        for bucket in &mut buckets {
            bucket.sort_unstable_by_key(|&(.., d)| d);
        }

        let (p1, p2) = run::<10>(&points, &buckets);
        assert_eq!(p1, 40);
        assert_eq!(p2, 25272);
    }
}
