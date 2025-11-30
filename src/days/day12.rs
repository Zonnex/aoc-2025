pub const INPUT: &str = include_str!("../inputs/12/real.txt");

pub fn solve(_input: &str) -> (i32, i32) {
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = include_str!("../inputs/12/test.txt");

    #[test]
    fn test_with_example() {
        let (p1, p2) = solve(TEST);
        assert_eq!(p1, 0);
        assert_eq!(p2, 0);
    }
}
