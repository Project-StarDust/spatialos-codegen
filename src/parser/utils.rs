use std::convert::identity;

use nom::alt;
use nom::character::{complete::multispace0, is_digit};
use nom::delimited;
use nom::fold_many1;
use nom::is_not;
use nom::many0;
use nom::map;
use nom::map_opt;
use nom::map_res;
use nom::named;
use nom::one_of;
use nom::pair;
use nom::peek;
use nom::separated_list;
use nom::tag;
use nom::take_while1;
use nom::tap;

named!(
    pub uppercase<char>,
    one_of!("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
);

named!(
    pub lowercase<char>,
    one_of!("abcdefghijklmnopqrstuvwxyz")
);

named!(
    pub lower_alphanum<char>,
    one_of!("abcdefghijklmnopqrstuvwxyz0123456789")
);

named!(
    pub camel_case_component<String>,
    map!(pair!(uppercase, many0!(lower_alphanum)), |(c, s)| c.to_string() + &s.iter().collect::<String>())
);

named!(
    pub camel_case<String>,
    fold_many1!(camel_case_component, String::new(), |acc, val| acc + &val)
);

named!(
    pub snake_case_component<String>,
    fold_many1!(lower_alphanum, String::new(), |acc, val| acc + &val.to_string())
);

named!(
    pub upper_snake_case_component<String>,
    fold_many1!(uppercase, String::new(), |acc, val| acc + &val.to_string())
);

named!(
    pub snake_case<String>,
    map_opt!(map!(separated_list!(tag!("_"), snake_case_component), |v| {
        let mut it = v.into_iter();
        it.next().map(|e| it.fold(e, |acc, val| acc + "_" + &val))
    }), |o| o)
);

named!(
    pub upper_snake_case<String>,
    map_opt!(map!(separated_list!(tag!("_"), upper_snake_case_component), |v| {
        let mut it = v.into_iter();
        it.next().map(|e| it.fold(e, |acc, val| acc + "_" + &val))
    }), |o| o)
);

named!(
    pub parse_usize<usize>,
    map_res!(
        map_res!(take_while1!(is_digit), std::str::from_utf8),
        |s: &str| s.parse::<usize>()
    )
);

named!(
    pub parse_comment<Option<String>>,
    map!(pair!(
        tag!("//"),
        alt!(
            map!(map_res!(is_not!("\n\r"), std::str::from_utf8), |o| o.to_string()) => {|d| Some(d)} |
            peek!(one_of!("\n\r")) => {|_| None}
        )
    ), |(_, c)| c)
);

named!(
    pub parse_comments<Vec<String>>,
    map!(
        many0!(delimited!(multispace0, parse_comment, multispace0)),
        |c| c.into_iter().filter_map(identity).map(|s| s.replace("\"", "\\\"")).collect::<Vec<_>>()
    )
);
