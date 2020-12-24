use crate::{
    ast::{Command, Component, Enum, Event, Member, Type},
    parser::{
        command::parse_command,
        event::parse_event,
        member::parse_member,
        r#enum::parse_enum,
        r#type::parse_type,
        utils::camel_case as parse_component_name,
        utils::{parse_comments, parse_u32, ws0, ws1},
    },
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, map_res},
    multi::many1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug)]
enum ComponentProperty {
    ID(u32),
    Member(Member),
    Command(Command),
    Event(Event),
    Type(Type),
    Enum(Enum),
}

#[derive(Default)]
struct ComponentBuilder {
    pub name: Option<String>,
    pub id: Option<u32>,
    pub members: Vec<Member>,
    pub commands: Vec<Command>,
    pub events: Vec<Event>,
    pub comments: Vec<String>,
    pub types: Vec<Type>,
    pub enums: Vec<Enum>,
}

impl ComponentBuilder {
    pub fn with_property(mut self, property: ComponentProperty) -> Self {
        match property {
            ComponentProperty::ID(id) => self.id = Some(id),
            ComponentProperty::Member(m) => self.members.push(m),
            ComponentProperty::Event(e) => self.events.push(e),
            ComponentProperty::Command(c) => self.commands.push(c),
            ComponentProperty::Type(t) => self.types.push(t),
            ComponentProperty::Enum(e) => self.enums.push(e),
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

    pub fn build(self) -> Result<Component, &'static str> {
        let name = self.name.ok_or("Name could not be found")?;
        let id = self.id.ok_or("ID could not be found")?;
        Ok(Component {
            name,
            id,
            members: self.members,
            commands: self.commands,
            events: self.events,
            comments: self.comments,
            enums: self.enums,
            types: self.types,
        })
    }
}

fn parse_id(input: &[u8]) -> IResult<&[u8], u32> {
    preceded(tag("id"), preceded(ws0(char('=')), parse_u32))(input)
}

fn parse_direct_property(input: &[u8]) -> IResult<&[u8], ComponentProperty> {
    terminated(
        ws0(alt((
            map(parse_id, ComponentProperty::ID),
            map(parse_member, ComponentProperty::Member),
            map(parse_command, ComponentProperty::Command),
            map(parse_event, ComponentProperty::Event),
        ))),
        char(';'),
    )(input)
}

fn parse_property(input: &[u8]) -> IResult<&[u8], ComponentProperty> {
    ws0(alt((
        parse_direct_property,
        map(parse_type, ComponentProperty::Type),
        map(parse_enum, ComponentProperty::Enum),
    )))(input)
}

fn parse_properties(input: &[u8]) -> IResult<&[u8], Vec<ComponentProperty>> {
    many1(ws0(parse_property))(input)
}

fn parse_component_body(input: &[u8]) -> IResult<&[u8], Vec<ComponentProperty>> {
    delimited(char('{'), ws0(parse_properties), char('}'))(input)
}

pub fn parse_component(input: &[u8]) -> IResult<&[u8], Component> {
    map_res(
        map(
            tuple((
                ws0(parse_comments),
                preceded(tag("component"), ws1(parse_component_name)),
                parse_component_body,
            )),
            |(comments, name, properties)| {
                properties
                    .into_iter()
                    .fold(ComponentBuilder::default(), |acc, val| {
                        acc.with_property(val)
                    })
                    .with_name(name)
                    .with_comments(comments)
            },
        ),
        |builder| builder.build(),
    )(input)
}

#[cfg(test)]
mod tests {

    use crate::ast::{UserDefinedType, Variant};

    use super::*;

    const SIMPLE_COMPONENT: &str = "
        component AnimalCounter {
            id = 1001;
            uint32 rabbits = 1;
            double platypus = 2;
            command uint32 count_platypus(Field);
            event Rabbit new_rabbit;
        }";

    const COMMENTED_COMPONENT: &str = "
        // This is used to count animals
        component AnimalCounter {
            id = 1001;
            
            // This is used to count rabbits
            uint32 rabbits = 1;
            
            // This is used to count platypus
            double platypus = 2;

            command uint32 count_platypus(Field);
            event Rabbit new_rabbit;
        }";

    const NESTED_COMPONENT: &str = "
        // This is used to count animals
        component AnimalCounter {
            id = 1001;
            
            // This is used to count rabbits
            uint32 rabbits = 1;
            
            // This is used to count platypus
            double platypus = 2;

            enum LifeState {
                ALIVE = 0;
                DEAD = 1;
            }

            type Rabbit {
                enum Gender {
                    MALE = 1;
                    FEMALE = 2;
                }
                
                string name = 1;
                LifeState life_state = 2;
                Gender gender = 3;

            }

            command uint32 count_platypus(Field);
            event Rabbit new_rabbit;
        }";

    #[test]
    fn test_parse_component() {
        assert_eq!(
            parse_component(SIMPLE_COMPONENT.as_bytes()),
            Ok((
                &b""[..],
                Component {
                    id: 1001,
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
                    events: vec![Event {
                        name: "new_rabbit".to_owned(),
                        r_type: crate::ast::DataType::UserDefined(UserDefinedType::Unresolved(
                            "Rabbit".to_owned()
                        ))
                    }],
                    commands: vec![Command {
                        name: "count_platypus".to_owned(),
                        r_type: crate::ast::DataType::Uint32,
                        args: vec![crate::ast::DataType::UserDefined(
                            UserDefinedType::Unresolved("Field".to_owned())
                        )]
                    }],
                    enums: vec![],
                    types: vec![]
                }
            ))
        );
        assert_eq!(
            parse_component(COMMENTED_COMPONENT.as_bytes()),
            Ok((
                &b""[..],
                Component {
                    id: 1001,
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
                    events: vec![Event {
                        name: "new_rabbit".to_owned(),
                        r_type: crate::ast::DataType::UserDefined(UserDefinedType::Unresolved(
                            "Rabbit".to_owned()
                        ))
                    }],
                    commands: vec![Command {
                        name: "count_platypus".to_owned(),
                        r_type: crate::ast::DataType::Uint32,
                        args: vec![crate::ast::DataType::UserDefined(
                            UserDefinedType::Unresolved("Field".to_owned())
                        )]
                    }],
                    enums: vec![],
                    types: vec![]
                }
            ))
        );
        assert_eq!(
            parse_component(NESTED_COMPONENT.as_bytes()),
            Ok((
                &b""[..],
                Component {
                    comments: vec![" This is used to count animals".to_owned()],
                    name: "AnimalCounter".to_string(),
                    id: 1001,
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
                    events: vec![Event {
                        name: "new_rabbit".to_owned(),
                        r_type: crate::ast::DataType::UserDefined(UserDefinedType::Unresolved(
                            "Rabbit".to_owned()
                        ))
                    }],
                    commands: vec![Command {
                        name: "count_platypus".to_owned(),
                        r_type: crate::ast::DataType::Uint32,
                        args: vec![crate::ast::DataType::UserDefined(
                            UserDefinedType::Unresolved("Field".to_owned())
                        )]
                    }],
                    enums: vec![Enum {
                        comments: vec![],
                        name: "LifeState".to_owned(),
                        variants: vec![
                            Variant {
                                comments: vec![],
                                name: "ALIVE".to_owned(),
                                id: 0
                            },
                            Variant {
                                comments: vec![],
                                name: "DEAD".to_owned(),
                                id: 1
                            }
                        ]
                    }],
                    types: vec![Type {
                        comments: vec![],
                        name: "Rabbit".to_owned(),
                        members: vec![
                            Member {
                                comments: vec![],
                                m_type: crate::ast::DataType::String,
                                name: "name".to_owned(),
                                id: 1
                            },
                            Member {
                                comments: vec![],
                                m_type: crate::ast::DataType::UserDefined(
                                    UserDefinedType::Unresolved("LifeState".to_owned())
                                ),
                                name: "life_state".to_owned(),
                                id: 2
                            },
                            Member {
                                comments: vec![],
                                m_type: crate::ast::DataType::UserDefined(
                                    UserDefinedType::Unresolved("Gender".to_owned())
                                ),
                                name: "gender".to_owned(),
                                id: 3
                            }
                        ],
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
                        }],
                        types: vec![]
                    }]
                }
            ))
        );
    }
}
