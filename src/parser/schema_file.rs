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
        pair(parse_package_name, ws0(parse_models)),
        |(package_name, models)| {
            models
                .into_iter()
                .fold(SchemaFileBuilder::default(), |acc, val| acc.with_model(val))
                .with_package_name(package_name)
        },
    )(input)
}
