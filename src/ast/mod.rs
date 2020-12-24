pub mod ast_node;
pub mod root;
pub mod schema_file;

use std::convert::identity;

pub use ast_node::ASTNode;
pub use root::{ASTBuilder, AST};
pub use schema_file::SchemaFile;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Command {
    pub name: String,
    pub r_type: DataType,
    pub args: Vec<DataType>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Event {
    pub name: String,
    pub r_type: DataType,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Component {
    pub name: String,
    pub id: u32,
    pub members: Vec<Member>,
    pub events: Vec<Event>,
    pub commands: Vec<Command>,
    pub comments: Vec<String>,
    pub enums: Vec<Enum>,
    pub types: Vec<Type>,
}

impl Component {
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Member {
    pub name: String,
    pub m_type: DataType,
    pub id: u32,
    pub comments: Vec<String>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Variant {
    pub name: String,
    pub id: u32,
    pub comments: Vec<String>,
}
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Type {
    pub name: String,
    pub members: Vec<Member>,
    pub comments: Vec<String>,
    pub types: Vec<Type>,
    pub enums: Vec<Enum>,
}

impl Type {
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
#[derive(Debug, Eq, PartialEq)]
pub struct PackageNode {
    pub name: String,
    pub inner: Vec<ASTNode>,
}

impl PackageNode {
    pub fn add_node(self, node: ASTNode) -> Self {
        let mut inner = self.inner;
        inner.push(node);
        Self {
            name: self.name,
            inner,
        }
    }

    pub fn has_path<S: AsRef<str>>(&self, path: S) -> bool {
        self.inner
            .iter()
            .map(|node| match &node {
                ASTNode::SchemaNode(_) => false,
                ASTNode::PackageNode(pn) => pn.name == *path.as_ref(),
            })
            .fold(false, |acc, val| acc | val)
    }

    pub fn get_exports(&self) -> Vec<String> {
        vec![self.name.clone()]
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<Variant>,
    pub comments: Vec<String>,
}

impl Enum {
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum DataType {
    Bool,
    Uint32,
    Uint64,
    Int32,
    Int64,
    SInt32,
    SInt64,
    Fixed32,
    Fixed64,
    SFixed32,
    SFixed64,
    Float,
    Double,
    String,
    Bytes,
    EntityID,
    Entity,
    Map(Box<DataType>, Box<DataType>),
    List(Box<DataType>),
    Option(Box<DataType>),
    UserDefined(UserDefinedType),
}

impl DataType {
    pub fn spatial_type(&self) -> String {
        match self {
            Self::Bool => "bool".to_string(),
            Self::Float => "float".to_string(),
            Self::Bytes => "bytes".to_string(),
            Self::Int32 => "int32".to_string(),
            Self::Int64 => "int64".to_string(),
            Self::String => "string".to_string(),
            Self::Double => "double".to_string(),
            Self::Uint32 => "uint32".to_string(),
            Self::Uint64 => "uint64".to_string(),
            Self::SInt32 => "sint32".to_string(),
            Self::SInt64 => "sint64".to_string(),
            Self::Fixed32 => "fixed32".to_string(),
            Self::Fixed64 => "fixed64".to_string(),
            Self::SFixed32 => "sfixed32".to_string(),
            Self::SFixed64 => "sfixed64".to_string(),
            Self::EntityID => "EntityId".to_string(),
            Self::Entity => "Entity".to_string(),
            Self::Map(fst, snd) => format!("map<{},{}>", fst.spatial_type(), snd.spatial_type()),
            Self::List(fst) => format!("list<{}>", fst.spatial_type()),
            Self::Option(fst) => format!("option<{}>", fst.spatial_type()),
            Self::UserDefined(fst) => fst.spatial_type(),
        }
    }

    pub fn rust_type(&self) -> String {
        match self {
            Self::Bool => "bool".to_string(),
            Self::Uint32 => "u32".to_string(),
            Self::Uint64 => "u64".to_string(),
            Self::Int32 => "i32".to_string(),
            Self::Int64 => "i64".to_string(),
            Self::Float => "f32".to_string(),
            Self::Double => "f64".to_string(),
            Self::String => "String".to_string(),
            Self::Bytes => "Vec<u8>".to_string(),
            Self::Map(fst, snd) => {
                format!("HashMap<{}, {}>", (*fst).rust_type(), (*snd).rust_type())
            }
            Self::List(fst) => format!("Vec<{}>", (*fst).rust_type()),
            Self::Option(fst) => format!("Option<{}>", (*fst).rust_type()),
            Self::UserDefined(fst) => fst.rust_type(),
            _ => "uninmplemented()!".to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ResolvedTypeKind {
    Enum,
    Type,
    Component,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum UserDefinedType {
    Unresolved(String),
    Resolved(String, ResolvedTypeKind),
}

impl From<&(String, ResolvedTypeKind)> for UserDefinedType {
    fn from(data: &(String, ResolvedTypeKind)) -> Self {
        Self::Resolved(data.0.clone(), data.1.clone())
    }
}

impl UserDefinedType {
    pub fn spatial_type(&self) -> String {
        match self {
            Self::Unresolved(name) => panic!("{} is not resolved in the current schema", name),
            Self::Resolved(name, kind) => match kind {
                ResolvedTypeKind::Enum => "enum",
                ResolvedTypeKind::Type => "type",
                ResolvedTypeKind::Component => panic!("You can't reference component {}", name),
            }
            .to_string(),
        }
    }

    pub fn rust_type(&self) -> String {
        match self {
            Self::Unresolved(name) => name.to_owned(),
            Self::Resolved(name, _) => name.to_owned(),
        }
    }
}
