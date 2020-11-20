use crate::{
    ast::Value,
    parser::utils::{parse_comments, parse_usize, upper_snake_case as parse_value_name, ws0},
};
use nom::{
    character::complete::char,
    combinator::map,
    sequence::{preceded, tuple},
    IResult,
};

pub fn parse_value(input: &[u8]) -> IResult<&[u8], Value> {
    map(
        tuple((
            parse_comments,
            parse_value_name,
            preceded(ws0(char('=')), parse_usize),
        )),
        |(comments, name, id)| Value { name, id, comments },
    )(input)
}
