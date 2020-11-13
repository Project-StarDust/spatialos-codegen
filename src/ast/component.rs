use crate::ast::Command;
use crate::ast::Event;
use crate::ast::Member;
use std::convert::identity;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Component {
    pub name: String,
    pub id: usize,
    pub members: Vec<Member>,
    pub events: Vec<Event>,
    pub commands: Vec<Command>,
    pub comments: Vec<String>,
}

impl Component {
    pub fn generate_one(&self) -> String {
        format!(
            "{}\n{}{}\n{}\npub struct {} {{{}}}",
            format!("#[allow(dead_code)]"),
            self.comments.iter().fold(String::new(), |acc, val| {
                if !acc.is_empty() {
                    acc + &format!("#[doc = \"{}\"]\n", val)
                } else {
                    format!("#[doc = \"{}\"]\n", val)
                }
            }),
            format!("#[derive(SpatialComponent)]"),
            format!("#[id({})]", self.id),
            self.name,
            Member::generate_multiple(&self.members)
        )
    }

    pub fn generate_multiple(data: &[Self]) -> String {
        data.iter()
            .map(Component::generate_one)
            .fold(String::new(), |acc, val| acc + "\n\n" + &val)
    }

    pub fn get_export(&self) -> Option<String> {
        Some(self.name.clone())
    }

    pub fn get_exports(data: &[Self]) -> Vec<String> {
        data.iter()
            .map(Self::get_export)
            .filter_map(identity)
            .collect()
    }
}
