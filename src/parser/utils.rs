use std::{convert::identity, iter::FromIterator};

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{char, multispace0, multispace1, one_of},
    combinator::{map, map_opt, map_res, peek, value},
    error::ParseError,
    multi::{fold_many1, many0, many1, separated_list1},
    sequence::{delimited, pair},
    IResult,
};

pub fn ws0<'a, F: 'a, O, E: ParseError<&'a [u8]>>(
    inner: F,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], O, E>
where
    F: FnMut(&'a [u8]) -> IResult<&'a [u8], O, E>,
{
    delimited(multispace0, inner, multispace0)
}

pub fn ws1<'a, F: 'a, O, E: ParseError<&'a [u8]>>(
    inner: F,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], O, E>
where
    F: FnMut(&'a [u8]) -> IResult<&'a [u8], O, E>,
{
    delimited(multispace1, inner, multispace1)
}

pub fn uppercase(input: &[u8]) -> IResult<&[u8], char> {
    one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")(input)
}

pub fn digit(input: &[u8]) -> IResult<&[u8], char> {
    one_of("0123456789")(input)
}

pub fn lowercase(input: &[u8]) -> IResult<&[u8], char> {
    one_of("abcdefghijklmnopqrstuvwxyz")(input)
}

pub fn lower_alphanum(input: &[u8]) -> IResult<&[u8], char> {
    alt((lowercase, digit))(input)
}

pub fn camel_case_component(input: &[u8]) -> IResult<&[u8], String> {
    map(pair(uppercase, many0(lower_alphanum)), |(c, s)| {
        c.to_string() + &s.iter().collect::<String>()
    })(input)
}

pub fn camel_case(input: &[u8]) -> IResult<&[u8], String> {
    fold_many1(camel_case_component, String::new(), |acc, val| acc + &val)(input)
}

pub fn snake_case_component(input: &[u8]) -> IResult<&[u8], String> {
    fold_many1(lower_alphanum, String::new(), |acc, val| {
        acc + &val.to_string()
    })(input)
}

pub fn upper_snake_case_component(input: &[u8]) -> IResult<&[u8], String> {
    fold_many1(uppercase, String::new(), |acc, val| acc + &val.to_string())(input)
}

pub fn snake_case(input: &[u8]) -> IResult<&[u8], String> {
    map_opt(
        map(separated_list1(char('_'), snake_case_component), |v| {
            let mut it = v.into_iter();
            it.next().map(|e| it.fold(e, |acc, val| acc + "_" + &val))
        }),
        identity,
    )(input)
}

pub fn upper_snake_case(input: &[u8]) -> IResult<&[u8], String> {
    map_opt(
        map(
            separated_list1(char('_'), upper_snake_case_component),
            |v| {
                let mut it = v.into_iter();
                it.next().map(|e| it.fold(e, |acc, val| acc + "_" + &val))
            },
        ),
        identity,
    )(input)
}

pub fn parse_usize(input: &[u8]) -> IResult<&[u8], usize> {
    map_res(map(many1(digit), String::from_iter), |s| s.parse::<usize>())(input)
}

pub fn parse_comment(input: &[u8]) -> IResult<&[u8], Option<String>> {
    map(
        pair(
            tag("//"),
            alt((
                map(
                    map(map_res(is_not("\n\r"), std::str::from_utf8), |o| {
                        o.to_string()
                    }),
                    Some,
                ),
                value(None, peek(one_of("\n\r"))),
            )),
        ),
        |(_, c)| c,
    )(input)
}

pub fn parse_comments(input: &[u8]) -> IResult<&[u8], Vec<String>> {
    map(many0(ws0(parse_comment)), |c| {
        c.into_iter()
            .filter_map(identity)
            .map(|s| s.replace("\"", "\\\""))
            .collect::<Vec<_>>()
    })(input)
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_camel_case_component() {
        assert_eq!(
            camel_case_component(b"CamelCase"),
            Ok((&b"Case"[..], "Camel".to_string()))
        )
    }
    #[test]
    fn test_camel_case_component_end() {
        assert_eq!(
            camel_case_component(b"Camel"),
            Ok((&b""[..], "Camel".to_string()))
        )
    }
}
