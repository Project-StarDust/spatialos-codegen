use crate::{
    ast::{Member, Type},
    parser::{
        member::parse_member,
        utils::{camel_case as parse_type_name, parse_comments, ws0, ws1},
    },
};

use nom::{
    bytes::complete::tag,
    character::complete::{char, multispace0},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

fn parse_members(input: &[u8]) -> IResult<&[u8], Vec<Member>> {
    separated_list1(
        multispace0,
        terminated(parse_member, pair(multispace0, char(';'))),
    )(input)
}

fn parse_type_body(input: &[u8]) -> IResult<&[u8], Vec<Member>> {
    delimited(char('{'), ws0(parse_members), char('}'))(input)
}

pub fn parse_type(input: &[u8]) -> IResult<&[u8], Type> {
    map(
        tuple((
            parse_comments,
            preceded(tag("type"), ws1(parse_type_name)),
            parse_type_body,
        )),
        |(comments, name, members)| Type {
            name,
            members,
            comments,
        },
    )(input)
}
