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
        utils::{parse_comments, parse_u32, snake_case as parse_member_name, ws0},
    },
};

fn parse_member_type_name(input: &[u8]) -> IResult<&[u8], (DataType, String)> {
    separated_pair(parse_type, multispace1, parse_member_name)(input)
}

pub fn parse_member(input: &[u8]) -> IResult<&[u8], Member> {
    map(
        pair(
            parse_comments,
            separated_pair(parse_member_type_name, ws0(char('=')), parse_u32),
        ),
        |(comments, ((ty, name), id))| Member {
            m_type: ty,
            name,
            id,
            comments,
        },
    )(input)
}

#[cfg(test)]
mod tests {

    use crate::ast::DataType;

    use super::*;

    #[test]
    fn test_parse_member() {
        assert_eq!(
            parse_member(b"uint32 rabbits = 1"),
            Ok((
                &b""[..],
                Member {
                    comments: Vec::new(),
                    name: "rabbits".to_string(),
                    m_type: DataType::Uint32,
                    id: 1
                }
            ))
        );
        assert_eq!(
            parse_member(b"// This is the number of rabbits\nuint32 rabbits = 1"),
            Ok((
                &b""[..],
                Member {
                    comments: vec![" This is the number of rabbits".to_string()],
                    name: "rabbits".to_string(),
                    m_type: DataType::Uint32,
                    id: 1
                }
            ))
        );
    }
}
