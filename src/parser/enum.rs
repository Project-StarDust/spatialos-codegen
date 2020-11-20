use crate::{
    ast::{Enum, Value},
    parser::{
        utils::{camel_case as parse_enum_name, parse_comments, ws0, ws1},
        value::parse_value,
    },
};
use nom::{
    bytes::complete::tag,
    character::complete::{char, multispace0},
    combinator::map,
    multi::separated_list1,
    sequence::delimited,
    sequence::{preceded, terminated, tuple},
    IResult,
};

fn parse_values(input: &[u8]) -> IResult<&[u8], Vec<Value>> {
    separated_list1(
        multispace0,
        terminated(parse_value, tuple((multispace0, char(';')))),
    )(input)
}

fn parse_enum_body(input: &[u8]) -> IResult<&[u8], Vec<Value>> {
    delimited(char('{'), ws0(parse_values), char('}'))(input)
}

pub fn parse_enum(input: &[u8]) -> IResult<&[u8], Enum> {
    map(
        tuple((
            parse_comments,
            preceded(tag("enum"), ws1(parse_enum_name)),
            parse_enum_body,
        )),
        |(comments, name, values)| Enum {
            name,
            values,
            comments,
        },
    )(input)
}
