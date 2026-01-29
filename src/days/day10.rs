//! # Day 10: Factory Joltage Puzzle
//!
//! ## The Problem
//! We have buttons that toggle lights (Part 1) and reduce joltages (Part 2).
//! Each button affects a subset of 16 positions. We need to find the minimum
//! button presses to reach target states.
//!
//! ## The "Bifurcate" Algorithm (Part 2)
//!
//! The key insight is that we can solve this recursively by "bifurcating" -
//! dividing the problem in half at each step.
//! Inspired by <https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory>/
//! 
//! ### Core Observation
//! If we press a set of buttons that:
//! 1. Makes all joltages even (by matching odd parities via XOR)
//! 2. Subtracts the right amounts so we can divide by 2
//!
//! Then: `f(joltages) = min(button_count + 2 * f(joltages_after_divide))`
//!
//! ### Example
//! Joltages: [6, 3, 4]  (binary: 110, 011, 100)
//! Parities: [0, 1, 0]  (need buttons `XORing` to 010 to make all even)
//!
//! Say button B toggles position 1 and subtracts 1 from position 1:
//! - After B: [6, 2, 4] - all even!
//! - Divide by 2: [3, 1, 2]
//! - Recurse to solve [3, 1, 2], say it needs K presses
//! - Total: 1 + 2*K presses
//!
//! At each DFS call, iterate through all 2^n button combinations
//! to find ones matching the target XOR pattern.
//!
//! PRECOMPUTE all button combinations once per puzzle line,
//! grouping them by their XOR pattern into an array:
//!
//! ```text
//! mask_to_combos[xor_pattern] = [(count, diff), (count, diff), ...]
//! ```

use crate::util::hash::*;
use crate::util::parse::*;

pub const INPUT: &str = include_str!("../inputs/10/real.txt");

pub fn solve(input: &str) -> (usize, u64) {
    let mut p1 = 0;
    let mut p2 = 0;

    // Reusable buffers
    let mut buttons = Vec::with_capacity(16);
    let mut button_diffs = Vec::with_capacity(16);
    let mut mask_to_combos: [Vec<(u32, [u16; 16])>; 1024] = std::array::from_fn(|_| Vec::new());
    let mut cache = FastMap::new();

    for line in input.lines() {
        buttons.clear();
        button_diffs.clear();
        
        let mut lights = 0_u16;
        let mut joltages = [0_u16; 16];

        for part in line.split_whitespace() {
            match part.as_bytes()[0] {
                b'[' => {
                    for (i, &b) in part[1..part.len()-1].as_bytes().iter().enumerate() {
                        if b == b'#' {
                            lights |= 1 << i;
                        }
                    }
                }
                b'(' => {
                    let mut mask = 0_u16;
                    let mut diff = [0_u16; 16];
                    for idx in part.iter_unsigned::<usize>() {
                        mask |= 1 << idx;
                        diff[idx] = 1;
                    }
                    buttons.push(mask);
                    button_diffs.push(diff);
                }
                b'{' => {
                    for (i, val) in part.iter_unsigned::<u16>().enumerate() {
                        joltages[i] = val;
                    }
                }
                _ => {}
            }
        }

        // Precompute all button combinations grouped by XOR mask
        for combos in &mut mask_to_combos {
            combos.clear();
        }
        precompute_combos(&buttons, &button_diffs, &mut mask_to_combos);

        p1 += mask_to_combos[lights as usize]
            .iter()
            .map(|(count, _)| *count as usize)
            .min()
            .expect("to have a valid count");

        cache.clear();
        p2 += dfs(&mask_to_combos, &mut cache, joltages);
    }

    (p1, p2)
}

/// Precompute all 2^n button combinations, storing (count, diff) grouped by XOR mask
fn precompute_combos(
    buttons: &[u16],
    button_diffs: &[[u16; 16]],
    mask_to_combos: &mut [Vec<(u32, [u16; 16])>; 1024],
) {
    let n = buttons.len();
    for combo in 0..(1_u32 << n) {
        let mut xor_mask = 0_usize;
        let mut diff = [0_u16; 16];
        let mut count = 0_u32;

        for i in 0..n {
            if combo & (1 << i) != 0 {
                xor_mask ^= buttons[i] as usize;
                count += 1;
                for j in 0..16 {
                    diff[j] += button_diffs[i][j];
                }
            }
        }

        mask_to_combos[xor_mask].push((count, diff));
    }
}

fn dfs(
    mask_to_combos: &[Vec<(u32, [u16; 16])>; 1024],
    cache: &mut FastMap<[u16; 16], u64>,
    joltages: [u16; 16],
) -> u64 {
    if joltages == [0; 16] {
        return 0;
    }

    if let Some(&result) = cache.get(&joltages) {
        return result;
    }

    let mut pattern = 0_usize;
    for i in 0..16 {
        pattern |= (joltages[i] as usize & 1) << i;
    }

    const INF: u64 = 1_000_000;
    let mut best = INF;

    for &(count, diff) in &mask_to_combos[pattern] {
        let mut valid = true;
        let mut next = [0_u16; 16];
        for i in 0..16 {
            if diff[i] > joltages[i] {
                valid = false;
                break;
            }
            next[i] = (joltages[i] - diff[i]) / 2;
        }
        if !valid {
            continue;
        }

        let sub = dfs(mask_to_combos, cache, next);
        if sub < INF {
            best = best.min(2 * sub + count as u64);
        }
    }

    cache.insert(joltages, best);
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = include_str!("../inputs/10/test.txt");

    #[test]
    fn test_with_example() {
        let (p1, p2) = solve(TEST);
        assert_eq!(p1, 7);
        assert_eq!(p2, 33);
    }
}
