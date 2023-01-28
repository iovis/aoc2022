use std::collections::HashSet;

use color_eyre::Result;

pub fn main() -> Result<()> {
    color_eyre::install()?;
    let input = include_str!("input.txt");

    let a1 = p1(input);
    println!("a1: {a1:?}");

    let a2 = p2(input);
    println!("a2: {a2:?}");

    Ok(())
}

/// Find position of 4 consecutive unique chars
fn p1(input: &str) -> usize {
    solver(input, 4)
}

/// Find position of 14 consecutive unique chars
fn p2(input: &str) -> usize {
    solver(input, 14)
}

fn solver(input: &str, window_size: usize) -> usize {
    let input = input.as_bytes();

    input
        .windows(window_size)
        .position(|slice| slice.iter().collect::<HashSet<_>>().len() == window_size)
        .map(|pos| pos + window_size)
        .unwrap()
}

#[cfg(test)]
mod tests {
    // use super::*;

    mod p1 {
        use crate::p1;

        #[test]
        fn test_example() {
            assert_eq!(p1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
            assert_eq!(p1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
            assert_eq!(p1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
            assert_eq!(p1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
        }
    }
}
