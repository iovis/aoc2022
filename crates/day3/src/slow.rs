use std::collections::HashSet;

use itertools::Itertools;

/// Find the common item per line
/// - each line is split in two
#[must_use]
pub fn p1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| {
            (
                a.chars().collect::<HashSet<char>>(),
                b.chars().collect::<HashSet<char>>(),
            )
        })
        .collect_vec() // Seems to be necessary to avoid dangling references
        .iter()
        .flat_map(|(a, b)| a.intersection(b))
        .map(|c| priority(*c))
        .sum()
}

/// Find the common item between 3 lines
pub fn p2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.chars().collect::<HashSet<char>>())
        .tuples() // Split in groups of 3
        .map(|(a, b, c)| find_common_badge(&a, &b, &c))
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

fn find_common_badge(a: &HashSet<char>, b: &HashSet<char>, c: &HashSet<char>) -> char {
    let ab = a.intersection(b).copied();
    let ab = ab.collect::<HashSet<char>>();

    *c.intersection(&ab).next().unwrap()
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

    #[test]
    fn test_find_common_badge() {
        let a = HashSet::from(['a', 'b', 'C']);
        let b = HashSet::from(['e', 'B', 'C']);
        let c = HashSet::from(['d', 'f', 'C']);

        assert_eq!(find_common_badge(&a, &b, &c), 'C');
    }

    mod p1 {
        use crate::slow::p1;

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
        use crate::slow::p2;

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
