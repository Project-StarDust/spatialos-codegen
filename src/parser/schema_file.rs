use crate::{
    ast::{Component, Enum, SchemaFile, Type},
    parser::{
        component::parse_component, package_name::parse_package_name, r#enum::parse_enum,
        r#type::parse_type, utils::ws0,
    },
};

use nom::{branch::alt, combinator::map, multi::many0, sequence::pair, IResult};

#[derive(Default)]
pub struct SchemaFileBuilder {
    pub package_name: Option<Vec<String>>,
    pub name: Option<String>,
    pub types: Vec<Type>,
    pub enums: Vec<Enum>,
    pub components: Vec<Component>,
}

#[derive(Debug)]
pub enum SchemaModel {
    Type(Type),
    Component(Component),
    Enum(Enum),
}

impl SchemaFileBuilder {
    pub fn with_model(mut self, model: SchemaModel) -> Self {
        match model {
            SchemaModel::Type(t) => self.types.push(t),
            SchemaModel::Component(c) => self.components.push(c),
            SchemaModel::Enum(e) => self.enums.push(e),
        };
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_package_name(mut self, package_name: Vec<String>) -> Self {
        self.package_name = Some(package_name);
        self
    }

    pub fn build(self) -> Result<SchemaFile, &'static str> {
        let name = self.name.ok_or("Name could not be found")?;
        let package_name = self.package_name.ok_or("Package Name could not be found")?;
        Ok(SchemaFile {
            package_name,
            name,
            components: self.components,
            types: self.types,
            enums: self.enums,
        })
    }
}

fn parse_model(input: &[u8]) -> IResult<&[u8], SchemaModel> {
    alt((
        map(parse_type, SchemaModel::Type),
        map(parse_component, SchemaModel::Component),
        map(parse_enum, SchemaModel::Enum),
    ))(input)
}

fn parse_models(input: &[u8]) -> IResult<&[u8], Vec<SchemaModel>> {
    many0(ws0(parse_model))(input)
}

pub fn parse_schema(input: &[u8]) -> IResult<&[u8], SchemaFileBuilder> {
    map(
        ws0(pair(parse_package_name, ws0(parse_models))),
        |(package_name, models)| {
            models
                .into_iter()
                .fold(SchemaFileBuilder::default(), |acc, val| acc.with_model(val))
                .with_package_name(package_name)
        },
    )(input)
}

#[cfg(test)]
mod tests {

    use crate::ast::*;

    use super::*;

    const SIMPLE_COMPONENT: &str = "
        package io.nebulis.player;

        enum LifeState {
            ALIVE = 0;
            DEAD = 1;
            RESPAWNING = 2;
        }
        
        
        type IsDead { }
        
        type Damage {
            uint32 points = 1;
        }
        
        type DamageResponse {}
        
        component Health {
            id = 601;
            uint32 hp = 1;
            uint32 max_hp = 2;
        
            event IsDead is_dead;
            event Damage took_damage;
            command DamageResponse damage(Damage);
        }
    ";

    #[test]
    fn test_parse_schema() {
        let schema_builder = parse_schema(SIMPLE_COMPONENT.as_bytes());
        assert!(schema_builder.is_ok());
        let (rest, schema_builder) = schema_builder.unwrap();
        let schema = schema_builder.with_name("test".to_owned()).build();
        assert!(schema.is_ok());
        println!("{:?}", schema);
        assert_eq!(
            schema,
            Ok(SchemaFile {
                package_name: vec!["io".to_owned(), "nebulis".to_owned(), "player".to_owned()],
                name: "test".to_owned(),
                types: vec![
                    Type {
                        name: "IsDead".to_owned(),
                        members: vec![],
                        comments: vec![],
                        types: vec![],
                        enums: vec![]
                    },
                    Type {
                        name: "Damage".to_owned(),
                        members: vec![Member {
                            comments: vec![],
                            name: "points".to_owned(),
                            m_type: DataType::Uint32,
                            id: 1
                        }],
                        comments: vec![],
                        types: vec![],
                        enums: vec![]
                    },
                    Type {
                        name: "DamageResponse".to_owned(),
                        members: vec![],
                        comments: vec![],
                        types: vec![],
                        enums: vec![]
                    },
                ],
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
                            id: 1,
                        },
                        Variant {
                            comments: vec![],
                            name: "RESPAWNING".to_owned(),
                            id: 2,
                        }
                    ]
                }],
                components: vec![Component {
                    id: 601,
                    name: "Health".to_owned(),
                    comments: vec![],
                    members: vec![
                        Member {
                            comments: vec![],
                            name: "hp".to_owned(),
                            id: 1,
                            m_type: DataType::Uint32
                        },
                        Member {
                            comments: vec![],
                            name: "max_hp".to_owned(),
                            id: 2,
                            m_type: DataType::Uint32
                        }
                    ],
                    events: vec![
                        Event {
                            name: "is_dead".to_owned(),
                            r_type: DataType::UserDefined("IsDead".to_owned())
                        },
                        Event {
                            name: "took_damage".to_owned(),
                            r_type: DataType::UserDefined("Damage".to_owned())
                        }
                    ],
                    commands: vec![Command {
                        name: "damage".to_owned(),
                        r_type: DataType::UserDefined("DamageResponse".to_owned()),
                        args: vec![DataType::UserDefined("Damage".to_owned())],
                    }],
                    enums: vec![],
                    types: vec![]
                }]
            })
        );
        assert_eq!(rest, &b""[..]);
    }
}
