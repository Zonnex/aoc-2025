pub const INPUT: &str = include_str!("../inputs/03/real.txt");

pub fn solve(input: &str) -> (u64, u64) {
    let mut p1 = 0;
    let mut p2 = 0;

    for bank in input.lines() {
        let bank = bank.as_bytes();
        p1 += largest_voltage::<2>(bank);
        p2 += largest_voltage::<12>(bank);
    }

    (p1, p2)
}

fn largest_voltage<const SIZE: usize>(bank: &[u8]) -> u64 {
    let (rest, tail) = bank.split_at(bank.len() - SIZE);
    let mut top: [u8; SIZE] = tail.try_into().unwrap();

    for &b in rest.iter().rev() {
        if top[0] <= b {
            let mut carry = top[0];
            top[0] = b;
            for i in 1..SIZE {
                if carry < top[i] {
                    break;
                }
                (top[i], carry) = (carry, top[i]);
            }
        }
    }
    to_number(top)
}

#[inline]
fn to_number<const SIZE: usize>(top: [u8; SIZE]) -> u64 {
    top.into_iter().fold(0, |acc, d| acc * 10 + (d - b'0') as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = include_str!("../inputs/03/test.txt");

    #[test]
    fn test_with_example() {
        let (p1, p2) = solve(TEST);
        assert_eq!(p1, 357);
        assert_eq!(p2, 3121910778619);
    }
}
