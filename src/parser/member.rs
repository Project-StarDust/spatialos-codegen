use nom::{
    character::complete::{char, multispace1},
    combinator::map,
    sequence::{pair, separated_pair},
    IResult,
};

use crate::{
    ast::{DataType, Member},
    parser::{
        data_type::parse_type,
        utils::{parse_comments, parse_usize, snake_case as parse_member_name, ws0},
    },
};

fn parse_member_type_name(input: &[u8]) -> IResult<&[u8], (DataType, String)> {
    separated_pair(parse_type, multispace1, parse_member_name)(input)
}

pub fn parse_member(input: &[u8]) -> IResult<&[u8], Member> {
    map(
        pair(
            parse_comments,
            separated_pair(parse_member_type_name, ws0(char('=')), parse_usize),
        ),
        |(comments, ((ty, name), id))| Member {
            m_type: ty,
            name,
            id,
            comments,
        },
    )(input)
}
