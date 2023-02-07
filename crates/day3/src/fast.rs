use itertools::Itertools;

/// Find the common item per line
/// - each line is split in two
#[must_use]
pub fn p1(input: &str) -> i32 {
    input
        .lines()
        .map(str::as_bytes)
        .map(|line| line.split_at(line.len() / 2))
        .map(|(left, right)| {
            // Keep state in a u64, where the fist bit is 'A' and 'z' is the 57th bit

            let mut state_left = 0u64;
            for letter in left {
                state_left |= 1 << (letter - b'A');
            }

            let mut state_right = 0u64;
            for letter in right {
                state_right |= 1 << (letter - b'A');
            }

            // println!("left:  {state_left:#066b}");
            // println!("right: {state_right:#066b}");

            let common_letter = state_left & state_right;
            // println!("inter: {common_letter:#066b}");

            (common_letter.trailing_zeros() as u8 + b'A') as char
        })
        .map(priority)
        .sum()
}

/// Find the common item between 3 lines
pub fn p2(input: &str) -> i32 {
    input
        .lines()
        .map(str::as_bytes)
        .tuples() // Split in groups of 3
        .map(|(a, b, c)| {
            // Keep state in a u64, where the fist bit is 'A' and 'z' is the 57th bit

            let mut state_a = 0u64;
            for letter in a {
                state_a |= 1 << (letter - b'A');
            }

            let mut state_b = 0u64;
            for letter in b {
                state_b |= 1 << (letter - b'A');
            }

            let mut state_c = 0u64;
            for letter in c {
                state_c |= 1 << (letter - b'A');
            }

            // println!("left:  {state_left:#066b}");
            // println!("right: {state_right:#066b}");

            let common_letter = state_a & state_b & state_c;
            // println!("inter: {common_letter:#066b}");

            (common_letter.trailing_zeros() as u8 + b'A') as char
        })
        .map(priority)
        .sum()
}

fn priority(item: char) -> i32 {
    if item.is_lowercase() {
        item as i32 - 96
    } else {
        item as i32 - 64 + 26
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(16, priority('p'));
        assert_eq!(38, priority('L'));
        assert_eq!(42, priority('P'));
        assert_eq!(22, priority('v'));
        assert_eq!(20, priority('t'));
        assert_eq!(19, priority('s'));
    }

    // #[test]
    // fn test_find_common_badge() {
    //     let a = HashSet::from(['a', 'b', 'C']);
    //     let b = HashSet::from(['e', 'B', 'C']);
    //     let c = HashSet::from(['d', 'f', 'C']);
    //
    //     assert_eq!(find_common_badge(&a, &b, &c), 'C');
    // }

    mod p1 {
        use super::p1;

        #[test]
        fn test_example() {
            let input: &str = indoc::indoc! {"
                vJrwpWtwJgWrhcsFMMfFFhFp
                jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
                PmmdzqPrVvPwwTWBwg
                wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
                ttgJtRGJQctTZtZT
                CrZsJsPPZsGzwwsLwLmpwMDw"
            };

            assert_eq!(p1(input), 157);
        }
    }

    mod p2 {
        use super::p2;

        #[test]
        fn test_example() {
            let input = indoc::indoc! {"
                vJrwpWtwJgWrhcsFMMfFFhFp
                jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
                PmmdzqPrVvPwwTWBwg
                wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
                ttgJtRGJQctTZtZT
                CrZsJsPPZsGzwwsLwLmpwMDw"
            };

            assert_eq!(p2(input), 70);
        }
    }
}
