use crate::{
    ast::Event,
    parser::{
        data_type::parse_type,
        utils::{snake_case as parse_event_name, ws1},
    },
};

use nom::{
    bytes::complete::tag,
    combinator::map,
    sequence::{pair, preceded},
    IResult,
};

pub fn parse_event(input: &[u8]) -> IResult<&[u8], Event> {
    map(
        pair(preceded(tag("event"), ws1(parse_type)), parse_event_name),
        |(ty, name)| Event { r_type: ty, name },
    )(input)
}
