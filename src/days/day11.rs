use std::collections::{HashMap, VecDeque};

pub const INPUT: &str = include_str!("../inputs/11/real.txt");

pub fn solve(input: &str) -> (u64, u64) {
    let connections = parse_connections(input);

    let p1 = p1(&connections);
    let p2 = p2(&connections);

    (p1, p2)
}

fn parse_connections(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|l| {
            let (from, to) = l.split_once(": ").unwrap();
            let to = to.split_whitespace().collect::<Vec<_>>();
            (from, to)
        })
        .collect::<HashMap<_, _>>()
}

fn p1(connections: &HashMap<&str, Vec<&str>>) -> u64 {
    let start = "you";
    let end = "out";
    let mut queue = VecDeque::new();
    for output in connections.get(start).unwrap() {
        queue.push_back(output);
    }

    let mut paths = 0;
    while let Some(current) = queue.pop_front() {
        if current == &end {
            paths += 1;
            continue;
        }
        for output in connections.get(current).unwrap() {
            queue.push_back(output);
        }
    }

    paths
}

fn p2(connections: &HashMap<&str, Vec<&str>>) -> u64 {
    let mut cache: HashMap<(&str, u8), u64> = HashMap::new();
    dfs("svr", 0, connections, &mut cache)
}

fn dfs<'a>(
    current: &'a str,
    checkpoints: u8, // bit 0 = visited "dac", bit 1 = visited "fft"
    connections: &HashMap<&'a str, Vec<&'a str>>,
    cache: &mut HashMap<(&'a str, u8), u64>,
) -> u64 {
    let checkpoints = match current {
        "dac" => checkpoints | 1,
        "fft" => checkpoints | 2,
        _ => checkpoints,
    };

    if current == "out" {
        return u64::from(checkpoints == 0b11);
    }

    if let Some(&result) = cache.get(&(current, checkpoints)) {
        return result;
    }

    let paths = connections
        .get(current)
        .map_or(0, |outputs| {
            outputs
                .iter()
                .map(|&next| dfs(next, checkpoints, connections, cache))
                .sum()
        });

    cache.insert((current, checkpoints), paths);
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_P1: &str = include_str!("../inputs/11/test_p1.txt");
    const TEST_P2: &str = include_str!("../inputs/11/test_p2.txt");

    #[test]
    fn test_with_example() {
        let p1 = p1(&parse_connections(TEST_P1));
        let p2 = p2(&parse_connections(TEST_P2));

        assert_eq!(p1, 5);
        assert_eq!(p2, 2);
    }
}
