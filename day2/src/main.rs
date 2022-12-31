use color_eyre::Result;

/// # Types
///
/// Rock     A, X, 1
/// Paper    B, Y, 2
/// Scissors C, Z, 3
///
/// # Scoring
///
/// Score = Move + Outcome
///
/// Outcome
///   Loss = 0
///   Draw = 3
///   Win  = 6
pub fn main() -> Result<()> {
    color_eyre::install()?;

    let input = include_str!("input.txt");
    let a1 = p1(input);

    println!("{a1:?}");

    Ok(())
}

fn p1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(Outcome::from)
        .map(<usize>::from)
        .sum()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Symbol {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<&str> for Symbol {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Outcome {
    Loss { symbol: Symbol },
    Draw { symbol: Symbol },
    Win { symbol: Symbol },
}

impl From<(&str, &str)> for Outcome {
    fn from(value: (&str, &str)) -> Self {
        let game = (Symbol::from(value.0), Symbol::from(value.1));

        match game {
            (Symbol::Rock, symbol @ Symbol::Rock) => Self::Draw { symbol },
            (Symbol::Rock, symbol @ Symbol::Paper) => Self::Win { symbol },
            (Symbol::Rock, symbol @ Symbol::Scissors) => Self::Loss { symbol },
            (Symbol::Paper, symbol @ Symbol::Rock) => Self::Loss { symbol },
            (Symbol::Paper, symbol @ Symbol::Paper) => Self::Draw { symbol },
            (Symbol::Paper, symbol @ Symbol::Scissors) => Self::Win { symbol },
            (Symbol::Scissors, symbol @ Symbol::Rock) => Self::Win { symbol },
            (Symbol::Scissors, symbol @ Symbol::Paper) => Self::Loss { symbol },
            (Symbol::Scissors, symbol @ Symbol::Scissors) => Self::Draw { symbol },
        }
    }
}

impl From<Outcome> for usize {
    fn from(value: Outcome) -> Self {
        match value {
            Outcome::Loss { symbol } => symbol as usize,
            Outcome::Draw { symbol } => 3 + symbol as usize,
            Outcome::Win { symbol } => 6 + symbol as usize,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_symbols() {
        assert_eq!(Symbol::from("A"), Symbol::Rock);
        assert_eq!(Symbol::from("X"), Symbol::Rock);
        assert_eq!(Symbol::from("B"), Symbol::Paper);
        assert_eq!(Symbol::from("Y"), Symbol::Paper);
        assert_eq!(Symbol::from("C"), Symbol::Scissors);
        assert_eq!(Symbol::from("Z"), Symbol::Scissors);

        assert_eq!(Symbol::Rock as usize, 1);
        assert_eq!(Symbol::Paper as usize, 2);
        assert_eq!(Symbol::Scissors as usize, 3);
    }

    #[test]
    fn test_outcomes() {
        let games = [
            (
                ("A", "X"),
                Outcome::Draw {
                    symbol: Symbol::Rock,
                },
            ),
            (
                ("A", "Y"),
                Outcome::Win {
                    symbol: Symbol::Paper,
                },
            ),
            (
                ("A", "Z"),
                Outcome::Loss {
                    symbol: Symbol::Scissors,
                },
            ),
            (
                ("B", "X"),
                Outcome::Loss {
                    symbol: Symbol::Rock,
                },
            ),
            (
                ("B", "Y"),
                Outcome::Draw {
                    symbol: Symbol::Paper,
                },
            ),
            (
                ("B", "Z"),
                Outcome::Win {
                    symbol: Symbol::Scissors,
                },
            ),
            (
                ("C", "X"),
                Outcome::Win {
                    symbol: Symbol::Rock,
                },
            ),
            (
                ("C", "Y"),
                Outcome::Loss {
                    symbol: Symbol::Paper,
                },
            ),
            (
                ("C", "Z"),
                Outcome::Draw {
                    symbol: Symbol::Scissors,
                },
            ),
        ];

        for game in games {
            assert_eq!(Outcome::from(game.0), game.1);
        }
    }

    #[test]
    fn test_outcome_value() {
        let outcomes = [
            (
                Outcome::Draw {
                    symbol: Symbol::Scissors,
                },
                3 + 3,
            ),
            (
                Outcome::Loss {
                    symbol: Symbol::Rock,
                },
                0 + 1,
            ),
            (
                Outcome::Win {
                    symbol: Symbol::Paper,
                },
                6 + 2,
            ),
        ];

        for outcome in outcomes {
            assert_eq!(<usize>::from(outcome.0), outcome.1);
        }
    }

    mod p1 {
        use crate::p1;

        #[test]
        fn test_example() {
            let input = indoc::indoc! {"
                A Y
                B X
                C Z
            "};

            assert_eq!(p1(input), 15);
        }
    }
}
