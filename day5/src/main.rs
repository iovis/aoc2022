use color_eyre::Result;
#[allow(unused_imports)]
use itertools::Itertools;

use self::operation::{parse_operation, Operation};

mod operation;

pub fn main() -> Result<()> {
    color_eyre::install()?;
    let input = include_str!("input.txt");

    let a1 = p1(input);
    println!("a1: {a1:?}");

    Ok(())
}

fn p1(input: &str) -> String {
    let (containers, operation) = input.split_once("\n\n").unwrap();

    let mut containers = parse_containers(containers);

    let operations = parse_operations(operation);

    for operation in operations {
        for _ in 0..operation.qty {
            let src = operation.src;
            let dst = operation.dst;

            let tmp = containers[src].pop().unwrap();

            containers[dst].push(tmp);
        }
    }

    containers
        .iter()
        .filter_map(|container| container.last())
        .collect()
}

fn parse_operations(instructions: &str) -> Vec<Operation> {
    instructions
        .lines()
        .filter_map(|line| parse_operation(line).ok())
        .map(|(_, operation)| operation)
        .collect()
}

fn parse_containers(input: &str) -> Vec<Vec<char>> {
    // upside down
    let containers: Vec<Vec<char>> = input
        .split('\n')
        .into_iter()
        .rev()
        .map(|line| line.chars().collect())
        .collect();

    let headers = &containers[0];

    let mut buckets = vec![];

    for (i, char) in headers.iter().enumerate() {
        // if char is a digit, parse that column as a container
        if char.is_ascii_digit() {
            let bucket = containers
                .iter()
                .filter_map(|line| line.get(i))
                .filter(|char| char.is_ascii_alphabetic())
                .copied()
                .collect();

            buckets.push(bucket);
        }
    }

    buckets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_containers() {
        let input = indoc::indoc! {"
                [D]
            [N] [C]
            [Z] [M] [P]
             1   2   3 "
        };

        #[rustfmt::skip]
        let expected_containers = vec![
            vec!['Z', 'N'],
            vec!['M', 'C', 'D'],
            vec!['P'],
        ];

        let containers = parse_containers(input);

        assert_eq!(containers, expected_containers);
    }

    mod p1 {
        use crate::p1;

        #[test]
        fn test_example() {
            let input = indoc::indoc! {"
                    [D]
                [N] [C]
                [Z] [M] [P]
                 1   2   3

                move 1 from 2 to 1
                move 3 from 1 to 3
                move 2 from 2 to 1
                move 1 from 1 to 2
            "};

            assert_eq!(p1(input), "CMZ");
        }
    }
}
