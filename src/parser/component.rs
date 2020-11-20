use crate::{
    ast::{Command, Component, Event, Member},
    parser::{
        command::parse_command,
        event::parse_event,
        member::parse_member,
        utils::camel_case as parse_component_name,
        utils::{parse_comments, parse_usize, ws0, ws1},
    },
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

enum ComponentProperty {
    ID(usize),
    Member(Member),
    Command(Command),
    Event(Event),
}

#[derive(Default)]
struct ComponentBuilder {
    pub name: Option<String>,
    pub id: Option<usize>,
    pub members: Vec<Member>,
    pub commands: Vec<Command>,
    pub events: Vec<Event>,
    pub comments: Vec<String>,
}

impl ComponentBuilder {
    pub fn with_property(mut self, property: ComponentProperty) -> Self {
        match property {
            ComponentProperty::ID(id) => self.id = Some(id),
            ComponentProperty::Member(m) => self.members.push(m),
            ComponentProperty::Event(e) => self.events.push(e),
            ComponentProperty::Command(c) => self.commands.push(c),
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
        })
    }
}

fn parse_id(input: &[u8]) -> IResult<&[u8], usize> {
    preceded(tag("id"), preceded(ws0(char('=')), parse_usize))(input)
}

fn parse_property(input: &[u8]) -> IResult<&[u8], ComponentProperty> {
    alt((
        map(parse_id, ComponentProperty::ID),
        map(parse_member, ComponentProperty::Member),
        map(parse_command, ComponentProperty::Command),
        map(parse_event, ComponentProperty::Event),
    ))(input)
}

fn parse_properties(input: &[u8]) -> IResult<&[u8], Vec<ComponentProperty>> {
    separated_list1(
        multispace0,
        terminated(parse_property, pair(multispace0, char(';'))),
    )(input)
}

fn parse_component_body(input: &[u8]) -> IResult<&[u8], Vec<ComponentProperty>> {
    delimited(char('{'), ws0(parse_properties), char('}'))(input)
}

pub fn parse_component(input: &[u8]) -> IResult<&[u8], Component> {
    map_res(
        map(
            tuple((
                parse_comments,
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
