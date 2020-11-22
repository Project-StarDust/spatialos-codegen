use crate::{
    ast::DataType,
    parser::utils::{uppercase, ws0},
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::take_while,
    character::complete::char,
    character::is_alphabetic,
    combinator::{map, map_res, value},
    sequence::separated_pair,
    sequence::{delimited, pair},
    IResult,
};

pub fn parse_one_generic(input: &[u8]) -> IResult<&[u8], DataType> {
    delimited(char('<'), ws0(parse_type_without_generics), char('>'))(input)
}

pub fn parse_two_generics(input: &[u8]) -> IResult<&[u8], (DataType, DataType)> {
    delimited(
        char('<'),
        ws0(separated_pair(
            parse_type_without_generics,
            ws0(char(',')),
            parse_type_without_generics,
        )),
        char('>'),
    )(input)
}

pub fn parse_primitive(input: &[u8]) -> IResult<&[u8], DataType> {
    alt((
        value(DataType::Bool, tag("bool")),
        value(DataType::Float, tag("float")),
        value(DataType::Bytes, tag("bytes")),
        value(DataType::Int32, tag("int32")),
        value(DataType::Int64, tag("int64")),
        value(DataType::String, tag("string")),
        value(DataType::Double, tag("double")),
        value(DataType::Uint32, tag("uint32")),
        value(DataType::Uint64, tag("uint64")),
        value(DataType::SInt32, tag("sint32")),
        value(DataType::SInt64, tag("sint64")),
        value(DataType::Fixed32, tag("fixed32")),
        value(DataType::Fixed64, tag("fixed64")),
        value(DataType::SFixed32, tag("sfixed32")),
        value(DataType::SFixed64, tag("sfixed64")),
        value(DataType::EntityID, tag("EntityId")),
        value(DataType::Entity, tag("Entity")),
    ))(input)
}

pub fn parse_user_type(input: &[u8]) -> IResult<&[u8], String> {
    map(
        pair(
            uppercase,
            map_res(take_while(is_alphabetic), std::str::from_utf8),
        ),
        |(first_letter, rest)| first_letter.to_string() + rest,
    )(input)
}

pub fn parse_generic_type(input: &[u8]) -> IResult<&[u8], DataType> {
    alt((
        map(pair(tag("map"), parse_two_generics), |(_, generics)| {
            DataType::Map(Box::new(generics.0), Box::new(generics.1))
        }),
        map(pair(tag("list"), parse_one_generic), |(_, generic)| {
            DataType::List(Box::new(generic))
        }),
        map(pair(tag("option"), parse_one_generic), |(_, generic)| {
            DataType::Option(Box::new(generic))
        }),
    ))(input)
}

pub fn parse_type_without_generics(input: &[u8]) -> IResult<&[u8], DataType> {
    alt((parse_primitive, map(parse_user_type, DataType::UserDefined)))(input)
}

pub fn parse_type(input: &[u8]) -> IResult<&[u8], DataType> {
    alt((parse_type_without_generics, parse_generic_type))(input)
}

#[cfg(test)]
mod tests {

    use super::*;
    use nom::{error::Error, error::ErrorKind, Err};

    #[test]
    fn test_parse_type() {
        assert_eq!(parse_type(b"bool"), Ok((&b""[..], DataType::Bool)));
        assert_eq!(parse_type(b"uint32"), Ok((&b""[..], DataType::Uint32)));
        assert_eq!(parse_type(b"uint64"), Ok((&b""[..], DataType::Uint64)));
        assert_eq!(parse_type(b"int32"), Ok((&b""[..], DataType::Int32)));
        assert_eq!(parse_type(b"int64"), Ok((&b""[..], DataType::Int64)));
        assert_eq!(parse_type(b"sint32"), Ok((&b""[..], DataType::SInt32)));
        assert_eq!(parse_type(b"sint64"), Ok((&b""[..], DataType::SInt64)));
        assert_eq!(parse_type(b"fixed32"), Ok((&b""[..], DataType::Fixed32)));
        assert_eq!(parse_type(b"fixed64"), Ok((&b""[..], DataType::Fixed64)));
        assert_eq!(parse_type(b"sfixed32"), Ok((&b""[..], DataType::SFixed32)));
        assert_eq!(parse_type(b"sfixed64"), Ok((&b""[..], DataType::SFixed64)));
        assert_eq!(parse_type(b"float"), Ok((&b""[..], DataType::Float)));
        assert_eq!(parse_type(b"double"), Ok((&b""[..], DataType::Double)));
        assert_eq!(parse_type(b"string"), Ok((&b""[..], DataType::String)));
        assert_eq!(parse_type(b"bytes"), Ok((&b""[..], DataType::Bytes)));
        assert_eq!(parse_type(b"EntityId"), Ok((&b""[..], DataType::EntityID)));
        assert_eq!(parse_type(b"Entity"), Ok((&b""[..], DataType::Entity)));
        assert_eq!(
            parse_type(b"CustomComponent"),
            Ok((
                &b""[..],
                DataType::UserDefined("CustomComponent".to_string())
            ))
        );
        assert_eq!(
            parse_type(b"map<float, bool>"),
            Ok((
                &b""[..],
                DataType::Map(Box::new(DataType::Float), Box::new(DataType::Bool))
            ))
        );
        assert_eq!(
            parse_type(b"option<bool>"),
            Ok((&b""[..], DataType::Option(Box::new(DataType::Bool))))
        );
        assert_eq!(
            parse_type(b"list<bool>"),
            Ok((&b""[..], DataType::List(Box::new(DataType::Bool))))
        );
        assert_eq!(
            parse_primitive(b"customComponent"),
            Err(Err::Error(Error::new(
                &b"customComponent"[..],
                ErrorKind::Tag
            )))
        );
    }
}
