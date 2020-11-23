use crate::ast::{Enum, Member};

use std::convert::identity;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Type {
    pub name: String,
    pub members: Vec<Member>,
    pub comments: Vec<String>,
    pub types: Vec<Type>,
    pub enums: Vec<Enum>,
}

impl Type {
    pub fn generate_one(&self) -> String {
        format!(
            "{}{}{}\n{}{}\npub struct {} {{{}}}",
            if !self.enums.is_empty() {
                Enum::generate_multiple(&self.enums) + "\n"
            } else {
                "".to_owned()
            },
            if !self.types.is_empty() {
                Type::generate_multiple(&self.types) + "\n"
            } else {
                "".to_owned()
            },
            "#[allow(dead_code)]",
            self.comments.iter().fold(String::new(), |acc, val| {
                if !acc.is_empty() {
                    acc + &format!("#[doc = \"{}\"]\n", val)
                } else {
                    format!("#[doc = \"{}\"]\n", val)
                }
            }),
            "#[derive(SpatialType)]",
            self.name,
            Member::generate_multiple(&self.members)
        )
    }

    pub fn generate_multiple(data: &[Self]) -> String {
        data.iter()
            .map(Type::generate_one)
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
