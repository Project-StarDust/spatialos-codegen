use quote::__private::TokenStream;

use crate::ast::{Component, Enum, Member, Type, Variant};

impl Generator for Component {
    fn generate_one(&self) -> TokenStream {
        let enums = <Enum as Generator>::generate_multiple(&self.enums);
        let types = <Type as Generator>::generate_multiple(&self.types);
        let members = <Member as Generator>::generate_multiple(&self.members);
        let comments = &self.comments;
        let id = &self.id;
        let name = format_ident!("{}", &self.name);
        quote! {
            #enums

            #types

            #[allow_dead_code]
            #(#[doc = #comments])*
            #[derive(SpatialComponent)]
            #[id(#id)]
            pub struct #name {
                #members
            }
        }
    }

    fn generate_multiple(data: &[Self]) -> TokenStream {
        let ones: Vec<_> = data.iter().map(Self::generate_one).collect();
        quote! {
            #(#ones)*
        }
    }
}

impl Generator for Enum {
    fn generate_one(&self) -> TokenStream {
        let comments = &self.comments;
        let name = format_ident!("{}", &self.name);
        let variants = <Variant as Generator>::generate_multiple(&self.variants);
        quote! {
            #(#[doc = #comments])*
            #[derive(SpatialEnum)]
            pub enum #name {
                #variants
            }

        }
    }

    fn generate_multiple(data: &[Self]) -> TokenStream {
        let ones: Vec<_> = data.iter().map(Self::generate_one).collect();
        quote! {
            #(#ones)*
        }
    }
}

impl Generator for Member {
    fn generate_one(&self) -> TokenStream {
        let docs = &self.comments;
        let id = self.id;
        let spatial_type = self.m_type.spatial_type();
        let rust_type =
            syn::parse_str::<syn::Type>(&self.m_type.rust_type()).expect("Can't parse type");
        let name = format_ident!("{}", &self.name);
        quote! {
            #(#[doc = #docs])*
            #[field_id(#id)]
            #[spatial_type(#spatial_type)]
            #name: #rust_type
        }
    }
}

pub(crate) trait Generator: Sized {
    fn generate_one(&self) -> TokenStream;
    fn generate_multiple(data: &[Self]) -> TokenStream {
        let ones: Vec<_> = data.iter().map(Self::generate_one).collect();
        quote! {
            #(#ones),*
        }
    }
}

impl Generator for Type {
    fn generate_one(&self) -> TokenStream {
        let enums = <Enum as Generator>::generate_multiple(&self.enums);
        let types = <Type as Generator>::generate_multiple(&self.types);
        let members = <Member as Generator>::generate_multiple(&self.members);
        let comments = &self.comments;
        let name = format_ident!("{}", &self.name);
        quote! {
            #enums

            #types

            #[allow_dead_code]
            #(#[doc = #comments])*
            #[derive(SpatialType)]
            pub struct #name {
                #members
            }

        }
    }

    fn generate_multiple(data: &[Self]) -> TokenStream {
        let ones: Vec<_> = data.iter().map(Self::generate_one).collect();
        quote! {
            #(#ones)*
        }
    }
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    format!(
        "{}{}",
        chars.next().unwrap().to_uppercase().collect::<String>(),
        chars.collect::<String>()
    )
}

fn to_upper_camel_case<S: AsRef<str>>(s: S) -> String {
    s.as_ref()
        .to_lowercase()
        .split("_")
        .map(capitalize)
        .fold(String::new(), |acc, val| acc + &val)
}

impl Generator for Variant {
    fn generate_one(&self) -> TokenStream {
        let comments = &self.comments;
        let id = &self.id;
        let name = format_ident!("{}", to_upper_camel_case(&self.name));
        quote! {
            #(#[doc = #comments])*
            #[value(#id)]
            #name
        }
    }
}
