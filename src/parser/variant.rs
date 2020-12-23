use crate::{
    ast::Variant,
    parser::utils::{parse_comments, parse_u32, upper_snake_case as parse_value_name, ws0},
};
use nom::{
    character::complete::char,
    combinator::map,
    sequence::{preceded, tuple},
    IResult,
};

pub fn parse_variant(input: &[u8]) -> IResult<&[u8], Variant> {
    map(
        tuple((
            parse_comments,
            parse_value_name,
            preceded(ws0(char('=')), parse_u32),
        )),
        |(comments, name, id)| Variant { name, id, comments },
    )(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_value() {
        assert_eq!(
            parse_variant(b"RABBITS_COUNTER = 1"),
            Ok((
                &b""[..],
                Variant {
                    comments: Vec::new(),
                    name: "RABBITS_COUNTER".to_string(),
                    id: 1
                }
            ))
        );
        assert_eq!(
            parse_variant(b"// This action is a rabbit counter\nRABBITS_COUNTER = 1"),
            Ok((
                &b""[..],
                Variant {
                    comments: vec![" This action is a rabbit counter".to_string()],
                    name: "RABBITS_COUNTER".to_string(),
                    id: 1
                }
            ))
        );
    }
}
