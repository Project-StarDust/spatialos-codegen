use std::convert::identity;

use nom::character::{complete::multispace0, is_digit};
use nom::combinator::map;
use nom::combinator::peek;
use nom::combinator::value;
use nom::multi::fold_many1;
use nom::multi::many0;
use nom::sequence::pair;
use nom::IResult;
use nom::{branch::alt, bytes::complete::is_not, bytes::complete::tag, sequence::delimited};
use nom::{bytes::complete::take_while1, character::complete::char, combinator::map_res};
use nom::{character::complete::one_of, combinator::map_opt, multi::separated_list1};

pub fn uppercase(input: &[u8]) -> IResult<&[u8], char> {
    one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")(input)
}

pub fn lowercase(input: &[u8]) -> IResult<&[u8], char> {
    one_of("abcdefghijklmnopqrstuvwxyz")(input)
}

pub fn lower_alphanum(input: &[u8]) -> IResult<&[u8], char> {
    one_of("abcdefghijklmnopqrstuvwxyz1234567890")(input)
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
    map_res(map_res(take_while1(is_digit), std::str::from_utf8), |s| {
        s.parse::<usize>()
    })(input)
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
    map(
        many0(delimited(multispace0, parse_comment, multispace0)),
        |c| {
            c.into_iter()
                .filter_map(identity)
                .map(|s| s.replace("\"", "\\\""))
                .collect::<Vec<_>>()
        },
    )(input)
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_camel_case_component() {
        assert_eq!(
            camel_case_component(b"CamelCase"),
            Ok(("Case".as_bytes(), "Camel".to_string()))
        )
    }
    #[test]
    fn test_camel_case_component_end() {
        assert_eq!(
            camel_case_component(b"Camel"),
            Ok(("".as_bytes(), "Camel".to_string()))
        )
    }
}
