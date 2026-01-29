use crate::util::grid::Grid;
use crate::util::hash::{FastMap, FastMapBuilder as _, FastSet, FastSetBuilder as _};
use crate::util::point::Point;

pub const INPUT: &str = include_str!("../inputs/07/real.txt");

pub fn solve(input: &str) -> (usize, u64) {
    let grid = Grid::parse(input);
    let mut splits: FastSet<Point> = FastSet::new();
    let mut cache: FastMap<Point, u64> = FastMap::new();
    
    let s = find_start(&grid);
    let timelines = dfs(&grid, s, &mut splits, &mut cache);
    
    let p1 = splits.len();
    let p2 = timelines;
    (p1, p2)
}

fn dfs(
    grid: &Grid<u8>,
    start: Point,
    splitters: &mut FastSet<Point>,
    cache: &mut FastMap<Point, u64>,
) -> u64 {
    if let Some(&cached) = cache.get(&start) {
        return cached;
    }
    
    let mut beam = Point { x: start.x, y: start.y + 2 }; // skip blank row
    
    loop {
        match grid.get(beam) {
            None => {
                cache.insert(start, 1);
                return 1;
            }
            Some(b'.') => beam.y += 1,
            Some(b'^') => {
                splitters.insert(beam);
                let left = Point { x: beam.x - 1, y: beam.y };
                let right = Point { x: beam.x + 1, y: beam.y };
                
                let left = dfs(grid, left, splitters, cache);
                let right = dfs(grid, right, splitters, cache);
                let paths = left + right;
                cache.insert(start, paths);
                return paths;
            }
            _ => unreachable!("Error in code")
        }
    }
}

fn find_start(grid: &Grid<u8>) -> Point {
    for x in 0..grid.width {
        if grid.bytes[x as usize] == b'S' {
            return Point { x, y: 0 };
        }
    }
    panic!("Error in input")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = include_str!("../inputs/07/test.txt");

    #[test]
    fn test_with_example() {
        let (p1, p2) = solve(TEST);
        assert_eq!(p1, 21);
        assert_eq!(p2, 40);
    }
}
