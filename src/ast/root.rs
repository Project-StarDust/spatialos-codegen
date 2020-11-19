use crate::ast::ASTNode;
use crate::ast::SchemaFile;
use std::convert::TryFrom;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug, Eq, PartialEq, Default)]
pub struct ASTBuilder {
    directories: Vec<PathBuf>,
}

#[allow(dead_code)]
impl ASTBuilder {
    pub fn build(self) -> AST {
        self.directories
            .into_iter()
            .map(|d| {
                WalkDir::new(&d)
                    .follow_links(true)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .map(|e| {
                        e.path()
                            .to_str()
                            .map(|s| s.to_string())
                            .ok_or("Can't tranform into &str")
                    })
                    .filter_map(Result::ok)
                    .map(PathBuf::from)
                    .filter(|p| p.extension() == Some(OsStr::new("schema")))
                    .map(|p| (SchemaFile::try_from(p.clone()), p))
                    .map(|(schemas, buf)| match schemas {
                        Ok(data) => Ok(data),
                        Err(e) => {
                            eprintln!("{}: {:?}", e, buf);
                            Err(())
                        }
                    })
                    .filter_map(Result::ok)
                    .collect::<Vec<_>>()
            })
            .flatten()
            .fold(AST::default(), |acc, val| {
                acc.merge_schema(&val, &val.package_name)
            })
    }

    pub fn with_directory<P: AsRef<Path>>(mut self, path: P) -> Self {
        let path = path.as_ref().to_path_buf();
        self.directories.push(path);
        self
    }
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct AST {
    pub inner: Vec<ASTNode>,
}

impl AST {
    pub fn generate<P: AsRef<Path> + Clone>(&self, path: P) -> Result<(), std::io::Error> {
        let path_clone = path.clone();
        if path_clone.as_ref().exists() {
            std::fs::remove_dir_all(path)?;
        }
        for node in &self.inner {
            node.generate_node(path_clone.clone())?;
        }
        ASTNode::generate_mod_rs(&self.inner, path_clone)
    }

    fn merge_schema<T: AsRef<str>>(self, schema: &SchemaFile, path: &[T]) -> Self {
        if !path.is_empty() {
            let is_path_present = self
                .inner
                .iter()
                .map(|n| match n {
                    ASTNode::SchemaNode(_) => panic!("SchemaFile shouldn't be at the root of AST"),
                    ASTNode::PackageNode(pn) => pn.name == *path[0].as_ref(),
                })
                .fold(false, |acc, val| acc | val);
            if is_path_present {
                AST {
                    inner: self
                        .inner
                        .into_iter()
                        .map(|n| n.merge_schema(schema, path))
                        .collect::<Vec<ASTNode>>(),
                }
            } else {
                let mut inner = self.inner;
                inner.push(ASTNode::package_schema(schema, path));
                AST { inner }
            }
        } else {
            panic!("SchemaFile does not have a package name");
        }
    }
}
