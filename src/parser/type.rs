use crate::{
    ast::{Enum, Member, Type},
    parser::{
        member::parse_member,
        r#enum::parse_enum,
        utils::{camel_case as parse_type_name, parse_comments, ws0, ws1},
    },
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, map_res},
    multi::many0,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug)]
enum TypeProperty {
    Member(Member),
    Type(Type),
    Enum(Enum),
}

#[derive(Default)]
struct TypeBuilder {
    pub name: Option<String>,
    pub members: Vec<Member>,
    pub comments: Vec<String>,
    pub types: Vec<Type>,
    pub enums: Vec<Enum>,
}

impl TypeBuilder {
    pub fn with_property(mut self, property: TypeProperty) -> Self {
        match property {
            TypeProperty::Member(m) => self.members.push(m),
            TypeProperty::Enum(e) => self.enums.push(e),
            TypeProperty::Type(t) => self.types.push(t),
        };
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_comments(mut self, comments: Vec<String>) -> Self {
        self.comments = comments;
        self
    }

    pub fn build(self) -> Result<Type, &'static str> {
        let name = self.name.ok_or("Name could not be found")?;
        Ok(Type {
            name,
            members: self.members,
            comments: self.comments,
            types: self.types,
            enums: self.enums,
        })
    }
}

fn parse_property(input: &[u8]) -> IResult<&[u8], TypeProperty> {
    ws0(alt((
        terminated(ws0(map(parse_member, TypeProperty::Member)), char(';')),
        map(parse_type, TypeProperty::Type),
        map(parse_enum, TypeProperty::Enum),
    )))(input)
}

fn parse_properties(input: &[u8]) -> IResult<&[u8], Vec<TypeProperty>> {
    many0(ws0(parse_property))(input)
}

fn parse_type_body(input: &[u8]) -> IResult<&[u8], Vec<TypeProperty>> {
    delimited(char('{'), ws0(parse_properties), char('}'))(input)
}

pub fn parse_type(input: &[u8]) -> IResult<&[u8], Type> {
    map_res(
        map(
            ws0(tuple((
                parse_comments,
                preceded(tag("type"), ws1(parse_type_name)),
                parse_type_body,
            ))),
            |(comments, name, properties)| {
                properties
                    .into_iter()
                    .fold(TypeBuilder::default(), |acc, val| acc.with_property(val))
                    .with_name(name)
                    .with_comments(comments)
            },
        ),
        |tb| tb.build(),
    )(input)
}

#[cfg(test)]
mod tests {

    use crate::ast::Variant;

    use super::*;

    const EMPTY_TYPE: &str = "type AnimalCounter { }";

    const SIMPLE_TYPE: &str = "
        type AnimalCounter {
            uint32 rabbits = 1;
            double platypus = 2;
        }";

    const COMMENTED_TYPE: &str = "
        // This is used to count animals
        type AnimalCounter {
            // This is used to count rabbits
            uint32 rabbits = 1 ;
            // This is used to count platypus
            double platypus = 2;
        }";

    const NESTED_TYPE: &str = "
        type Rabbit {

            type LifeState {
                uint32 health = 1;
            }

            enum Gender {
                MALE = 1;
                FEMALE = 2;
            }
            
            string name = 1;
            LifeState life_state = 2;
            Gender gender = 3;
        }";

    #[test]
    fn test_parse_type() {
        assert_eq!(
            parse_type(EMPTY_TYPE.as_bytes()),
            Ok((
                &b""[..],
                Type {
                    comments: Vec::new(),
                    name: "AnimalCounter".to_string(),
                    members: vec![],
                    types: vec![],
                    enums: vec![]
                }
            ))
        );
        assert_eq!(
            parse_type(SIMPLE_TYPE.as_bytes()),
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
                    ],
                    types: vec![],
                    enums: vec![]
                }
            ))
        );
        assert_eq!(
            parse_type(COMMENTED_TYPE.as_bytes()),
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
                    ],
                    types: vec![],
                    enums: vec![]
                }
            ))
        );
        assert_eq!(
            parse_type(NESTED_TYPE.as_bytes()),
            Ok((
                &b""[..],
                Type {
                    comments: vec![],
                    name: "Rabbit".to_string(),
                    members: vec![
                        Member {
                            m_type: crate::ast::DataType::String,
                            name: "name".to_owned(),
                            id: 1,
                            comments: vec![]
                        },
                        Member {
                            m_type: crate::ast::DataType::UserDefined("LifeState".to_owned()),
                            name: "life_state".to_owned(),
                            id: 2,
                            comments: vec![]
                        },
                        Member {
                            m_type: crate::ast::DataType::UserDefined("Gender".to_owned()),
                            name: "gender".to_owned(),
                            id: 3,
                            comments: vec![]
                        },
                    ],
                    types: vec![Type {
                        comments: vec![],
                        name: "LifeState".to_owned(),
                        members: vec![Member {
                            m_type: crate::ast::DataType::Uint32,
                            name: "health".to_owned(),
                            id: 1,
                            comments: vec![],
                        },],
                        types: vec![],
                        enums: vec![]
                    }],
                    enums: vec![Enum {
                        comments: vec![],
                        name: "Gender".to_owned(),
                        variants: vec![
                            Variant {
                                comments: vec![],
                                name: "MALE".to_owned(),
                                id: 1
                            },
                            Variant {
                                comments: vec![],
                                name: "FEMALE".to_owned(),
                                id: 2
                            }
                        ]
                    }]
                }
            ))
        );
    }
}
