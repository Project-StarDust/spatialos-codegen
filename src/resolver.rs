use std::collections::HashMap;

use crate::ast::{
    ASTNode, Component, DataType, Member, PackageNode, ResolvedTypeKind, SchemaFile, Type,
    UserDefinedType, AST,
};

type Context = HashMap<String, (String, ResolvedTypeKind)>;

fn register_type<S: AsRef<str>>(path: S, ty: &Type) -> Vec<(String, (String, ResolvedTypeKind))> {
    let mut types = vec![(
        ty.name.clone(),
        (
            path.as_ref().to_owned() + "::" + &ty.name,
            ResolvedTypeKind::Type,
        ),
    )];
    types.extend(
        ty.types
            .iter()
            .map(|ty| register_type(path.as_ref(), ty))
            .flatten(),
    );
    types.extend(ty.enums.iter().map(|en| {
        (
            en.name.to_owned(),
            (
                path.as_ref().to_string() + "::" + &en.name,
                ResolvedTypeKind::Enum,
            ),
        )
    }));
    types
}

fn register_component<S: AsRef<str>>(
    path: S,
    comp: &Component,
) -> Vec<(String, (String, ResolvedTypeKind))> {
    let mut types = vec![(
        comp.name.clone(),
        (
            path.as_ref().to_owned() + "::" + &comp.name,
            ResolvedTypeKind::Component,
        ),
    )];
    types.extend(
        comp.types
            .iter()
            .map(|ty| register_type(path.as_ref(), ty))
            .flatten(),
    );
    types.extend(comp.enums.iter().map(|en| {
        (
            en.name.to_owned(),
            (
                path.as_ref().to_string() + "::" + &en.name,
                ResolvedTypeKind::Enum,
            ),
        )
    }));
    types
}

fn register_schemas<S: AsRef<str>>(
    path: S,
    schema: &SchemaFile,
) -> Vec<(String, (String, ResolvedTypeKind))> {
    let mut types = schema
        .components
        .iter()
        .map(|comp| register_component(path.as_ref(), comp))
        .flatten()
        .collect::<Vec<_>>();
    types.extend(
        schema
            .types
            .iter()
            .map(|ty| register_type(path.as_ref(), ty))
            .flatten(),
    );
    types.extend(schema.enums.iter().map(|en| {
        (
            en.name.to_owned(),
            (
                path.as_ref().to_string() + "::" + &en.name,
                ResolvedTypeKind::Enum,
            ),
        )
    }));
    types
}

fn register_node<S: AsRef<str>>(
    path: S,
    node: &ASTNode,
) -> Vec<(String, (String, ResolvedTypeKind))> {
    match node {
        ASTNode::PackageNode(package) => package
            .inner
            .iter()
            .map(|node| register_node(path.as_ref().to_owned() + "::" + &package.name, node))
            .flatten()
            .collect(),
        ASTNode::SchemaNode(schema) => register_schemas(path, schema),
    }
}

fn resolve_date_type(ctx: &Context, data_type: DataType) -> DataType {
    match data_type {
        DataType::UserDefined(UserDefinedType::Unresolved(unresolved)) => {
            DataType::UserDefined(UserDefinedType::from(
                ctx.get(&unresolved)
                    .unwrap_or_else(|| panic!("Unable to resolve: {}", unresolved)),
            ))
        }
        DataType::Map(ty1, ty2) => DataType::Map(
            Box::new(resolve_date_type(ctx, *ty1)),
            Box::new(resolve_date_type(ctx, *ty2)),
        ),
        DataType::List(ty) => DataType::List(Box::new(resolve_date_type(ctx, *ty))),
        DataType::Option(ty) => DataType::Option(Box::new(resolve_date_type(ctx, *ty))),
        _ => data_type,
    }
}

fn resolve_member(ctx: &Context, mut member: Member) -> Member {
    member.m_type = resolve_date_type(ctx, member.m_type);
    member
}

fn resolve_component(ctx: &Context, mut comp: Component) -> Component {
    comp.members = comp
        .members
        .into_iter()
        .map(|member| resolve_member(ctx, member))
        .collect();
    comp.types = comp
        .types
        .into_iter()
        .map(|ty| resolve_type(ctx, ty))
        .collect();
    comp
}

fn resolve_type(ctx: &Context, mut ty: Type) -> Type {
    ty.members = ty
        .members
        .into_iter()
        .map(|member| resolve_member(ctx, member))
        .collect();
    ty.types = ty
        .types
        .into_iter()
        .map(|ty| resolve_type(ctx, ty))
        .collect();
    ty
}

fn resolve_schema(ctx: &Context, mut schema: SchemaFile) -> SchemaFile {
    schema.components = schema
        .components
        .into_iter()
        .map(|component| resolve_component(ctx, component))
        .collect();
    schema.types = schema
        .types
        .into_iter()
        .map(|t| resolve_type(ctx, t))
        .collect();
    schema
}

fn resolve_package(ctx: &Context, mut package: PackageNode) -> PackageNode {
    package.inner = package
        .inner
        .into_iter()
        .map(|n| resolve_node(ctx, n))
        .collect();
    package
}

fn resolve_node(ctx: &Context, node: ASTNode) -> ASTNode {
    match node {
        ASTNode::PackageNode(package) => ASTNode::PackageNode(resolve_package(ctx, package)),
        ASTNode::SchemaNode(schema) => ASTNode::SchemaNode(resolve_schema(ctx, schema)),
    }
}

pub fn resolve_types<S: AsRef<str>>(mut ast: AST, module: S) -> AST {
    let ctx = ast
        .inner
        .iter()
        .map(|node| register_node("crate::".to_string() + module.as_ref() , node))
        .flatten()
        .collect::<HashMap<_, _>>();

    ast.inner = ast
        .inner
        .into_iter()
        .map(|node| resolve_node(&ctx, node))
        .collect();
    ast
}
