use std::collections::HashSet;

use color_eyre::Result;
#[allow(unused_imports)]
use itertools::Itertools;

pub fn main() -> Result<()> {
    color_eyre::install()?;
    let input = include_str!("input.txt");

    let a1 = p1(input);
    println!("a1: {a1:?}");

    let a2 = p2(input);
    println!("a2: {a2:?}");

    Ok(())
}

/// Find the common item per line
/// - each line is split in two
fn p1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| {
            (
                HashSet::<char>::from_iter(a.chars()),
                HashSet::<char>::from_iter(b.chars()),
            )
        })
        .collect_vec() // Seems to be necessary to avoid dangling references
        .iter()
        .flat_map(|(a, b)| a.intersection(b))
        .map(priority)
        .sum()

    // let mut sum = 0;
    //
    // for line in input.lines() {
    //     let (one, two) = line.split_at(line.len() / 2);
    //     let one = HashSet::<char>::from_iter(one.chars());
    //     let two = HashSet::<char>::from_iter(two.chars());
    //
    //     let intersection = one.intersection(&two).next().unwrap();
    //
    //     sum += priority(intersection);
    // }
    //
    // sum
}

/// Find the common item between 3 lines
fn p2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| HashSet::<char>::from_iter(line.chars()))
        .collect_vec()
        .iter()
        .tuples() // Split in groups of 3
        .map(|(a, b, c)| find_common_badge(a, b, c))
        .map(|c| priority(&c))
        .sum()
}

fn priority(item: &char) -> i32 {
    if item.is_lowercase() {
        *item as i32 - 96
    } else {
        *item as i32 - 64 + 26
    }
}

fn find_common_badge(a: &HashSet<char>, b: &HashSet<char>, c: &HashSet<char>) -> char {
    let ab = a.intersection(b).copied();
    let ab = HashSet::<char>::from_iter(ab);

    *c.intersection(&ab).next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(16, priority(&'p'));
        assert_eq!(38, priority(&'L'));
        assert_eq!(42, priority(&'P'));
        assert_eq!(22, priority(&'v'));
        assert_eq!(20, priority(&'t'));
        assert_eq!(19, priority(&'s'));
    }

    #[test]
    fn test_find_common_badge() {
        let a = HashSet::from(['a', 'b', 'C']);
        let b = HashSet::from(['e', 'B', 'C']);
        let c = HashSet::from(['d', 'f', 'C']);

        assert_eq!(find_common_badge(&a, &b, &c), 'C');
    }

    mod p1 {
        use crate::p1;

        #[test]
        fn test_example() {
            let input: &str = indoc::indoc! {"
                vJrwpWtwJgWrhcsFMMfFFhFp
                jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
                PmmdzqPrVvPwwTWBwg
                wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
                ttgJtRGJQctTZtZT
                CrZsJsPPZsGzwwsLwLmpwMDw
            "};

            assert_eq!(p1(input), 157);
        }
    }

    mod p2 {
        use crate::p2;

        #[test]
        fn test_example() {
            let input = indoc::indoc! {"
                vJrwpWtwJgWrhcsFMMfFFhFp
                jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
                PmmdzqPrVvPwwTWBwg
                wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
                ttgJtRGJQctTZtZT
                CrZsJsPPZsGzwwsLwLmpwMDw
            "};

            assert_eq!(p2(input), 70);
        }
    }
}
