use crate::{
    ast::{Command, DataType},
    parser::{
        data_type::parse_type,
        utils::{snake_case as parse_command_name, ws0},
    },
};

use nom::{
    bytes::complete::tag,
    character::complete::multispace1,
    character::complete::{char, multispace0},
    combinator::map,
    multi::separated_list0,
    sequence::delimited,
    sequence::{preceded, tuple},
    IResult,
};

pub fn parse_args(input: &[u8]) -> IResult<&[u8], Vec<DataType>> {
    delimited(
        char('('),
        ws0(separated_list0(ws0(char(',')), parse_type)),
        char(')'),
    )(input)
}

pub fn parse_command(input: &[u8]) -> IResult<&[u8], Command> {
    map(
        tuple((
            preceded(tag("command"), preceded(multispace1, parse_type)),
            preceded(multispace1, parse_command_name),
            preceded(multispace0, parse_args),
        )),
        |(ty, name, args)| Command {
            r_type: ty,
            name,
            args,
        },
    )(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_command() {
        assert_eq!(
            parse_command(b"command uint32 count_rabbits()"),
            Ok((
                &b""[..],
                Command {
                    name: "count_rabbits".to_string(),
                    args: vec![],
                    r_type: DataType::Uint32
                }
            ))
        );
        assert_eq!(
            parse_command(b"command uint32 count_rabbits(bool)"),
            Ok((
                &b""[..],
                Command {
                    name: "count_rabbits".to_string(),
                    args: vec![DataType::Bool],
                    r_type: DataType::Uint32
                }
            ))
        );
    }
}
