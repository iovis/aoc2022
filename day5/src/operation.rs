use nom::bytes::complete::tag;
use nom::character::complete::{digit1, multispace0, multispace1};
use nom::combinator::{map, map_res};
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub struct Operation {
    pub qty: usize,
    pub src: usize,
    pub dst: usize,
}

pub fn parse(input: &str) -> IResult<&str, Operation> {
    map(
        tuple((parse_move, parse_source, parse_destination)),
        |(qty, src, dst)| Operation { qty, src, dst },
    )(input)
}

fn parse_move(input: &str) -> IResult<&str, usize> {
    preceded(tag("move"), parse_quantity)(input)
}

fn parse_source(input: &str) -> IResult<&str, usize> {
    preceded(tag("from"), parse_container)(input)
}

fn parse_destination(input: &str) -> IResult<&str, usize> {
    preceded(tag("to"), parse_container)(input)
}

#[rustfmt::skip]
fn parse_quantity(input: &str) -> IResult<&str, usize> {
    delimited(
        multispace1,
        parse_digit,
        multispace1,
    )(input)
}

#[rustfmt::skip]
fn parse_container(input: &str) -> IResult<&str, usize> {
    map(
        delimited(
            multispace1,
            parse_digit,
            multispace0,
        ),
        |n| n - 1  // containers are 1 based but struct is 0 based
    )(input)
}

fn parse_digit(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_operation() {
        let input = "move 2 from 3 to 1";

        assert_eq!(
            parse(input),
            Ok((
                "",
                Operation {
                    qty: 2,
                    src: 2,
                    dst: 0,
                }
            ))
        );
    }
}
