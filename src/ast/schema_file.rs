use quote::__private::TokenStream;

use crate::{
    ast::{Component, Enum, Type},
    codegen::Generator,
    parser::schema_file::parse_schema,
};
use std::{
    convert::TryFrom,
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SchemaFile {
    pub package_name: Vec<String>,
    pub name: String,
    pub types: Vec<Type>,
    pub enums: Vec<Enum>,
    pub components: Vec<Component>,
}

impl SchemaFile {
    fn generate(&self) -> TokenStream {
        let enums = Enum::generate_multiple(&self.enums);
        let types = Type::generate_multiple(&self.types);
        let components = Component::generate_multiple(&self.components);
        quote! {
            #[allow(unused_imports)]
            use std::collections::HashMap;

            #[allow(unused_imports)]
            use std::marker::PhantomData;

            #enums

            #types

            #components
        }
    }

    pub fn get_exports(&self) -> Vec<String> {
        let mut exports = vec![];
        exports.extend(Enum::get_exports(&self.enums));
        exports.extend(Type::get_exports(&self.types));
        exports.extend(Component::get_exports(&self.components));
        exports
    }

    pub fn generate_schema<P: AsRef<Path> + Clone>(&self, path: P) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(path.clone()).map(|_| {
            let mut file = File::create(path.clone().as_ref().join(self.name.clone() + ".rs"))?;
            write!(&mut file, "{}", self.generate())?;
            Command::new("rustfmt")
                .arg(path.clone().as_ref().join(self.name.clone() + ".rs"))
                .output()
                .expect("Failed to execute rustfmt");
            Ok(())
        })?
    }
}

impl TryFrom<PathBuf> for SchemaFile {
    type Error = String;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let filename = path
            .file_stem()
            .ok_or("Unable to get file stem")
            .map(|s| s.to_str())?
            .ok_or("Can't convert file stem to UTF-8")
            .map(|s| s.to_string())?;
        let mut file = File::open(path).map_err(|e| format!("Unable to open file: {}", e))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| format!("Unable to read file: {}", e))?;
        parse_schema(contents.as_bytes())
            .map(|r| r.1)
            .map_err(|e| format!("Unable to parse data: {}", e))
            .map(|sb| sb.with_name(filename).build())?
            .map_err(|e| format!("Cannot convert SchemaFile: {}", e))
    }
}
