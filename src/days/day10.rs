pub const INPUT: &str = include_str!("../inputs/10/real.txt");

// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
// [lights]
// (buttons)
// {joltage requirement}

pub fn solve(input: &str) -> (usize, usize) {    
    let p1 = p1(input);
    let p2 = p2(input);

    (p1, p2)
}

fn p1(input: &str) -> usize {
    let mut sum = 0;
    let mut button_buffer: Vec<u16> = Vec::with_capacity(10);
    for line in input.lines() {
        let mut pattern = 0;
        for part in line.split_whitespace() {
            match part.as_bytes()[0] {
                b'[' => {
                    for (i, b) in part.trim_matches(&['[', ']']).as_bytes().iter().enumerate() {
                        match b {
                            b'#' => pattern |= 1 << i,
                            _ => continue,
                        }
                    }
                }
                b'(' => {
                    let button = parse_button(part);
                    button_buffer.push(button);
                }
                b'{' => {
                    continue; // ignored in part 1
                }
                _ => unreachable!(),
            }
        }

        sum += test_machine(pattern, &button_buffer);

        button_buffer.clear();
    }
    sum
}

#[inline]
fn parse_button(pattern: &str) -> u16 {
    // produce a bitmask.
    // if a button is (2,3), then we can precompute it as 0011
    // which makes XOR'ing simple
    pattern
        .bytes()
        .filter(|b| b.is_ascii_digit())
        .fold(0u16, |acc, b| acc | (1 << (b - b'0')))
}

fn test_machine(pattern: u16, buttons: &[u16]) -> usize {
    let n = buttons.len();
    let limit = 1 << n;

    // Test by number of buttons pressed (1, 2, 3, ...)
    for button_presses in 1..=n {
        // Start with the smallest number that has exactly num_buttons bits set
        // e.g., num_buttons=1 -> 0b001, num_buttons=2 -> 0b011, num_buttons=3 -> 0b111
        let mut buttons_to_press: u32 = (1 << button_presses) - 1;

        while buttons_to_press < limit {
            // using buttons_to_press we can pull out the buttons we should press
            let mut test = 0u16;
            for i in 0..n {
                if buttons_to_press & (1 << i) != 0 {
                    test ^= buttons[i];  // Just XOR the precomputed bitmask!
                }
            }
            if test == pattern {
                return button_presses;
            }

            // Gosper's hack: get next number with same popcount
            let lowest_bit = buttons_to_press & buttons_to_press.wrapping_neg();
            let ripple = buttons_to_press + lowest_bit;
            let ones = (buttons_to_press ^ ripple) >> (lowest_bit.trailing_zeros() + 2);
            buttons_to_press = ripple | ones;
        }
    }

    unreachable!("Error in code")
}

fn p2(input: &str) -> usize {
    // Here we most definitely want to use BFS.
    // we can again encode the buttons nicely but this time using base 10
    // so the button (2,3) would be 0011, so 11.
    // button (0,1) would be 1100
    // this allows us to do good pruning of branches.
    // As soon as any digit exceeds the target we quit.
    // example: 1999, we can only ever increment the first counter once
    // any combination that exceeds 1 in this position is an invalid branch

    // for line in input.lines() {
    //     for part in line.split_whitespace() {
    //         match part.as_bytes()[0] {
    //             b'[' => continue, // ignored in part 2
    //             b'(' => {
    //                 let button = parse_button(part);
    //                 button_buffer.push(button);
    //             }
    //             b'{' => {
    //                 continue; // ignored for now
    //             }
    //             _ => unreachable!(),
    //         }
    //     }

    // }

    0
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
