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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_value() {
        assert_eq!(
            parse_value(b"RABBITS_COUNTER = 1"),
            Ok((
                &b""[..],
                Value {
                    comments: Vec::new(),
                    name: "RABBITS_COUNTER".to_string(),
                    id: 1
                }
            ))
        );
        assert_eq!(
            parse_value(b"// This action is a rabbit counter\nRABBITS_COUNTER = 1"),
            Ok((
                &b""[..],
                Value {
                    comments: vec![" This action is a rabbit counter".to_string()],
                    name: "RABBITS_COUNTER".to_string(),
                    id: 1
                }
            ))
        );
    }
}
