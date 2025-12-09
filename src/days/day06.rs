pub const INPUT: &str = include_str!("../inputs/06/real.txt");

pub fn solve(input: &str) -> (u64, u64) {
    let p1 = part1::<4>(input);
    let p2 = part2::<4>(input);

    (p1, p2)
}

fn part1<const N: usize>(input: &str) -> u64 {
    let mut columns: Vec<Column<N>> = Vec::new();
    for (y, line) in input.lines().enumerate().take(N) {
        for (x, column) in line.split_whitespace().enumerate() {
            let number = column.parse::<u64>().unwrap();
            match columns.get_mut(x) {
                Some(c) => c.values[y] = number,
                None => {
                    let mut c = Column::default();
                    c.values[y] = number;
                    columns.push(c);
                }
            }
        }
    }

    for (x, operator) in input.lines().last().unwrap().split_whitespace().enumerate() {
        columns[x].operator = match operator {
            "+" => Operator::Add,
            "*" => Operator::Mul,
            _ => unreachable!("Error in input"),
        }
    }

    

    columns
        .into_iter()
        .map(|c| match c.operator {
            Operator::Add => c.values.into_iter().sum::<u64>(),
            Operator::Mul => c.values.into_iter().product::<u64>(),
            Operator::Unspecified => panic!("Error in algorithm"),
        })
        .sum()
}

enum Operator {
    Unspecified,
    Add,
    Mul,
}

struct Column<const N: usize> {
    values: [u64; N],
    operator: Operator,
}

impl<const N: usize> Default for Column<N> {
    fn default() -> Self {
        Self {
            values: [0; N],
            operator: Operator::Unspecified,
        }
    }
}

fn part2<const N: usize>(input: &str) -> u64 {
    let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
    let operator_line = grid[N];
    let grid_width = operator_line.len() - 1;

    let mut p2 = 0;
    let mut column_numbers = Vec::new();
    let mut x_iter = (0..=grid_width).rev();

    while let Some(x) = x_iter.next() {
        let mut num = 0u64;
        for y in 0..N {
            let byte = grid[y][x];
            if byte.is_ascii_digit() {
                num = num * 10 + (byte - b'0') as u64;
            }
        }
        column_numbers.push(num);

        match operator_line[x] {
            b'+' => {
                p2 += column_numbers.iter().sum::<u64>();
                column_numbers.clear();
                x_iter.next(); // Skip blank separator
            }
            b'*' => {
                p2 += column_numbers.iter().product::<u64>();
                column_numbers.clear();
                x_iter.next(); // Skip blank separator
            }
            _ => {}
        }
    }

    p2
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = include_str!("../inputs/06/test.txt");

    #[test]
    fn test_with_example() {
        let p1 = part1::<3>(TEST);
        let p2 = part2::<3>(TEST);
        assert_eq!(p1, 4277556);
        assert_eq!(p2, 3263827);
    }
}
