use crate::ast::DataType;
use nom::alt;
use nom::bytes::complete::take_while;
use nom::character::complete::multispace0;
use nom::character::is_alphabetic;
use nom::complete;
use nom::delimited;
use nom::do_parse;
use nom::map_res;
use nom::named;
use nom::one_of;
use nom::pair;
use nom::separated_pair;
use nom::tag;

named!(
    pub parse_one_generic<DataType>,
    delimited!(
        tag!("<"),
        delimited!(
            multispace0,
            parse_type_without_generics,
            multispace0
        ),
        tag!(">")
    )
);

named!(
    pub parse_two_generics<(DataType, DataType)>,
    delimited!(
        tag!("<"),
        delimited!(
            multispace0,
            separated_pair!(
                parse_type_without_generics,
                delimited!(
                    multispace0,
                    tag!(","),
                    multispace0
                ),
                parse_type_without_generics
            ),
            multispace0
        ),
        tag!(">")
    )
);

named!(
    pub parse_primitive<DataType>,
    alt!(
        complete!(tag!("bool"))     => { |_| DataType::Bool }     |
        complete!(tag!("float"))    => { |_| DataType::Float }    |
        complete!(tag!("bytes"))    => { |_| DataType::Bytes }    |
        complete!(tag!("int32"))    => { |_| DataType::Int32 }    |
        complete!(tag!("int64"))    => { |_| DataType::Int64 }    |
        complete!(tag!("string"))   => { |_| DataType::String }   |
        complete!(tag!("double"))   => { |_| DataType::Double }   |
        complete!(tag!("uint32"))   => { |_| DataType::Uint32 }   |
        complete!(tag!("uint64"))   => { |_| DataType::Uint64 }   |
        complete!(tag!("sint32"))   => { |_| DataType::SInt32 }   |
        complete!(tag!("sint64"))   => { |_| DataType::SInt64 }   |
        complete!(tag!("fixed32"))  => { |_| DataType::Fixed32 }  |
        complete!(tag!("fixed64"))  => { |_| DataType::Fixed64 }  |
        complete!(tag!("sfixed32")) => { |_| DataType::SFixed32 } |
        complete!(tag!("sfixed64")) => { |_| DataType::SFixed64 } |
        complete!(tag!("EntityId")) => { |_| DataType::EntityID } |
        complete!(tag!("Entity"))   => { |_| DataType::Entity }
    )
);

named!(
    pub parse_user_type<String>,
    do_parse!(
        first_letter: one_of!("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
            >> rest: map_res!(complete!(take_while(is_alphabetic)), |s| std::str::from_utf8(s))
            >> (first_letter.to_string() + rest)
    )
);

named!(
    pub parse_generic_type<DataType>,
    alt!(
        complete!(
            pair!(tag!("map"), parse_two_generics)
        ) => {|(_, generics): (_, (DataType, DataType))| DataType::Map(Box::new(generics.0), Box::new(generics.1)) } |
        complete!(
            pair!(tag!("list"), parse_one_generic)
        ) => {|(_, generic)|  DataType::List(Box::new(generic)) } |
        complete!(
            pair!(tag!("option"), parse_one_generic)
        ) => {|(_, generic)|  DataType::Option(Box::new(generic)) }
    )
);

named!(
    parse_type_without_generics<DataType>,
    alt!(
        complete!(parse_primitive) |
        complete!(parse_user_type) => { |s| DataType::UserDefined(s) }
    )
);

named!(
    pub parse_type<DataType>,
    alt!(
        parse_type_without_generics |
        complete!(parse_generic_type)
    )
);

#[cfg(test)]
mod tests {

    use super::*;
    use nom::{error::Error, error::ErrorKind, Err};

    #[test]
    fn test_parse_primitive() {
        assert_eq!(parse_primitive(b"bool"), Ok((&b""[..], DataType::Bool)));
        assert_eq!(parse_primitive(b"uint32"), Ok((&b""[..], DataType::Uint32)));
        assert_eq!(parse_primitive(b"uint64"), Ok((&b""[..], DataType::Uint64)));
        assert_eq!(parse_primitive(b"int32"), Ok((&b""[..], DataType::Int32)));
        assert_eq!(parse_primitive(b"int64"), Ok((&b""[..], DataType::Int64)));
        assert_eq!(parse_primitive(b"sint32"), Ok((&b""[..], DataType::SInt32)));
        assert_eq!(parse_primitive(b"sint64"), Ok((&b""[..], DataType::SInt64)));
        assert_eq!(
            parse_primitive(b"fixed32"),
            Ok((&b""[..], DataType::Fixed32))
        );
        assert_eq!(
            parse_primitive(b"fixed64"),
            Ok((&b""[..], DataType::Fixed64))
        );
        assert_eq!(
            parse_primitive(b"sfixed32"),
            Ok((&b""[..], DataType::SFixed32))
        );
        assert_eq!(
            parse_primitive(b"sfixed64"),
            Ok((&b""[..], DataType::SFixed64))
        );
        assert_eq!(parse_primitive(b"float"), Ok((&b""[..], DataType::Float)));
        assert_eq!(parse_primitive(b"double"), Ok((&b""[..], DataType::Double)));
        assert_eq!(parse_primitive(b"string"), Ok((&b""[..], DataType::String)));
        assert_eq!(parse_primitive(b"bytes"), Ok((&b""[..], DataType::Bytes)));
        assert_eq!(
            parse_primitive(b"EntityId"),
            Ok((&b""[..], DataType::EntityID))
        );
        assert_eq!(parse_primitive(b"Entity"), Ok((&b""[..], DataType::Entity)));
        assert_eq!(
            parse_primitive(b"CustomComponent"),
            Err(Err::Error(Error::new(
                &b"CustomComponent"[..],
                ErrorKind::Alt
            )))
        );
    }

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
            parse_primitive(b"customComponent"),
            Err(Err::Error(Error::new(
                &b"customComponent"[..],
                ErrorKind::Alt
            )))
        );
    }
}
