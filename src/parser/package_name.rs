use nom::{
    bytes::complete::tag,
    character::complete::{char, multispace0, multispace1},
    multi::separated_list1,
    sequence::{delimited, pair},
    IResult,
};

use crate::parser::utils::snake_case;

pub fn parse_package_components(input: &[u8]) -> IResult<&[u8], Vec<String>> {
    separated_list1(char('.'), snake_case)(input)
}

pub fn parse_package_name(input: &[u8]) -> IResult<&[u8], Vec<String>> {
    delimited(
        pair(tag("package"), multispace1),
        parse_package_components,
        pair(multispace0, char(';')),
    )(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_package_name() {
        assert_eq!(
            parse_package_name(b"package io.nebulis;"),
            Ok((&b""[..], vec!["io".to_string(), "nebulis".to_string()]))
        )
    }
}
