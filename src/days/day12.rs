use crate::{
    BLANK_LINE,
    util::{iter::ChunkOps, parse::ParseOps},
};

pub const INPUT: &str = include_str!("../inputs/12/real.txt");

pub fn solve(input: &str) -> (i32, i32) {
    let mut p1 = 0;
    let parts = input.split(BLANK_LINE).collect::<Vec<_>>();
    if let Some((regions, shapes)) = parts.split_last() {
        let shapes = shapes.into_iter().map(shape_size).collect::<Vec<_>>();

        for region in regions.lines().map(parse_region) {
            let size = region[0] * region[1];

            let actual = region
                .iter()
                .skip(2)
                .enumerate()
                .map(|(shape, count)| count * shapes.get(shape).unwrap())
                .sum::<usize>();

            if actual < size {
                p1 += 1;
            }
        }
    }

    (p1, 0)
}

fn shape_size(shape: &&str) -> usize {
    shape
        .lines()
        .skip(1)
        .flat_map(|l| l.bytes())
        .filter(|&b| b == b'#')
        .count()
}

fn parse_region(line: &str) -> [usize; 8] {
    line.iter_unsigned::<usize>().chunk::<8>().next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = include_str!("../inputs/12/test.txt");

    #[test]
    fn test_with_example() {
        let (p1, p2) = solve(TEST);
        assert_eq!(p1, 521);
        assert_eq!(p2, 0);
    }
}
