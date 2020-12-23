use crate::{
    ast::{Enum, Variant},
    parser::{
        utils::{camel_case as parse_enum_name, parse_comments, ws0, ws1},
        variant::parse_variant,
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

fn parse_variants(input: &[u8]) -> IResult<&[u8], Vec<Variant>> {
    separated_list1(
        multispace0,
        terminated(parse_variant, tuple((multispace0, char(';')))),
    )(input)
}

fn parse_enum_body(input: &[u8]) -> IResult<&[u8], Vec<Variant>> {
    delimited(char('{'), ws0(parse_variants), char('}'))(input)
}

pub fn parse_enum(input: &[u8]) -> IResult<&[u8], Enum> {
    map(
        tuple((
            parse_comments,
            preceded(tag("enum"), ws1(parse_enum_name)),
            parse_enum_body,
        )),
        |(comments, name, variants)| Enum {
            name,
            variants,
            comments,
        },
    )(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_enum() {
        assert_eq!(
            parse_enum(
                b"enum AnimalCounter {\n\tRABBITS_COUNTER = 1 ;\n\tPLATYPUS_COUNTER = 2;\n}"
            ),
            Ok((
                &b""[..],
                Enum {
                    comments: Vec::new(),
                    name: "AnimalCounter".to_string(),
                    variants: vec![
                        Variant {
                            name: "RABBITS_COUNTER".to_owned(),
                            id: 1,
                            comments: vec![]
                        },
                        Variant {
                            name: "PLATYPUS_COUNTER".to_owned(),
                            id: 2,
                            comments: vec![]
                        },
                    ]
                }
            ))
        );
        assert_eq!(
            parse_enum(b"// This is used to count animals\nenum AnimalCounter {\n\t// This is used to count rabbits\n\tRABBITS_COUNTER = 1;\n\t// This is used to count platypus\n\tPLATYPUS_COUNTER = 2 ;\n}"),
            Ok((
                &b""[..],
                Enum {
                    comments: vec![" This is used to count animals".to_owned()],
                    name: "AnimalCounter".to_string(),
                    variants: vec![
                        Variant {
                            name: "RABBITS_COUNTER".to_owned(),
                            id: 1,
                            comments: vec![" This is used to count rabbits".to_owned()]
                        },
                        Variant {
                            name: "PLATYPUS_COUNTER".to_owned(),
                            id: 2,
                            comments: vec![" This is used to count platypus".to_owned()]
                        },
                    ]
                }
            ))
        );
    }
}
