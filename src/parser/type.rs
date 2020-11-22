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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_enum() {
        assert_eq!(
            parse_type(b"type AnimalCounter {\n\tuint32 rabbits = 1 ;\n\tdouble platypus = 2;\n}"),
            Ok((
                &b""[..],
                Type {
                    comments: Vec::new(),
                    name: "AnimalCounter".to_string(),
                    members: vec![
                        Member {
                            m_type: crate::ast::DataType::Uint32,
                            name: "rabbits".to_owned(),
                            id: 1,
                            comments: vec![]
                        },
                        Member {
                            m_type: crate::ast::DataType::Double,
                            name: "platypus".to_owned(),
                            id: 2,
                            comments: vec![]
                        },
                    ]
                }
            ))
        );
        assert_eq!(
            parse_type(b"// This is used to count animals\ntype AnimalCounter {\n\t// This is used to count rabbits\n\tuint32 rabbits = 1 ;\n\t// This is used to count platypus\n\tdouble platypus = 2;\n}"),
            Ok((
                &b""[..],
                Type {
                    comments: vec![" This is used to count animals".to_owned()],
                    name: "AnimalCounter".to_string(),
                    members: vec![
                        Member {
                            m_type: crate::ast::DataType::Uint32,
                            name: "rabbits".to_owned(),
                            id: 1,
                            comments: vec![" This is used to count rabbits".to_owned()]
                        },
                        Member {
                            m_type: crate::ast::DataType::Double,
                            name: "platypus".to_owned(),
                            id: 2,
                            comments: vec![" This is used to count platypus".to_owned()]
                        },
                    ]
                }
            ))
        );
    }
}
