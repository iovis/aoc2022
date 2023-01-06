use std::ops::RangeInclusive;

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

fn p1(input: &str) -> usize {
    input
        .lines()
        .map(parse_ranges)
        .filter(|(a, b)| is_any_range_fully_contained(a, b))
        .count()
}

fn p2(input: &str) -> usize {
    input
        .lines()
        .map(parse_ranges)
        .filter(|(a, b)| do_ranges_overlap(a, b))
        .count()
}

fn parse_ranges(line: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>){
    let ranges = line.split_once(',').unwrap();

    // range1
    let r1 = ranges.0;
    let (start, end) = r1.split_once('-').unwrap();
    let (start, end) = (start.parse().unwrap(), end.parse().unwrap());
    let r1 = start..=end;

    // range2
    let r2 = ranges.1;
    let (start, end) = r2.split_once('-').unwrap();
    let (start, end) = (start.parse().unwrap(), end.parse().unwrap());
    let r2 = start..=end;

    (r1, r2)
}

fn is_any_range_fully_contained(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    // Is a contained in b?
    if a.start() >= b.start() && a.end() <= b.end() {
        return true;
    }

    // Is b contained in a?
    if b.start() >= a.start() && b.end() <= a.end() {
        return true;
    }

    false
}

fn do_ranges_overlap(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    if a.contains(b.start()) || a.contains(b.end()) {
        return true;
    }

    if b.contains(a.start()) || b.contains(a.end()) {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::is_any_range_fully_contained;

    use super::*;

    #[test]
    fn test_range_overlap() {
        let a = 2..=4;
        let b = 6..=8;

        assert!(!is_any_range_fully_contained(&a, &b));

        let a = 2..=8;
        let b = 3..=7;

        assert!(is_any_range_fully_contained(&a, &b));

        let a = 2..=6;
        let b = 4..=8;

        assert!(!is_any_range_fully_contained(&a, &b));

        let a = 6..=6;
        let b = 4..=6;

        assert!(is_any_range_fully_contained(&a, &b));
    }

    mod p1 {
        use crate::p1;

        #[test]
        fn test_example() {
            let input = indoc::indoc! {"
                2-4,6-8
                2-3,4-5
                5-7,7-9
                2-8,3-7
                6-6,4-6
                2-6,4-8
            "};

            assert_eq!(p1(input), 2);
        }
    }

    mod p2 {
        use crate::p2;

        #[test]
        fn test_example() {
            let input = indoc::indoc! {"
                2-4,6-8
                2-3,4-5
                5-7,7-9
                2-8,3-7
                6-6,4-6
                2-6,4-8
            "};

            assert_eq!(p2(input), 4);
        }
    }
}
