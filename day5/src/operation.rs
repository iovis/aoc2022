use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::sequence::{preceded, tuple};
use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub struct Operation {
    quantity: usize,
    src: usize,
    dest: usize,
}

pub fn parse_operation(input: &str) -> IResult<&str, Operation> {
    map(
        tuple((
            preceded(tag("move "), parse_digit),
            preceded(tag(" from "), parse_container),
            preceded(tag(" to "), parse_container),
        )),
        |(quantity, src, dest)| Operation {
            quantity,
            src,
            dest,
        },
    )(input)
}

fn parse_digit(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

fn parse_container(input: &str) -> IResult<&str, usize> {
    map(parse_digit, |number| number - 1)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_operation() {
        let input = "move 2 from 3 to 1";

        assert_eq!(
            parse_operation(input),
            Ok((
                "",
                Operation {
                    quantity: 2,
                    src: 2,
                    dest: 0,
                }
            ))
        );
    }
}
