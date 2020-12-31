use quote::__private::TokenStream;

use crate::ast::{Command, Component, Enum, Member, Type, Variant};

impl Command {
    fn get_member<S: AsRef<str>>(&self, component_name: S) -> TokenStream {
        let name = format_ident!("{}", self.name);
        let command_name = format_ident!(
            "{}{}",
            component_name.as_ref(),
            to_upper_camel_case(&self.name)
        );
        quote! {
            #name: PhantomData<#command_name>
        }
    }

    fn get_types<S: AsRef<str>>(&self, component_name: S) -> TokenStream {
        let command_name = format_ident!(
            "{}{}",
            component_name.as_ref(),
            to_upper_camel_case(&self.name)
        );
        let request = format_ident!("{}Request", command_name);
        let response = format_ident!("{}Response", command_name);
        let request_args = if self.args.len() > 1 {
            let arg_types = self
                .args
                .iter()
                .map(|arg| arg.rust_type())
                .map(|s| syn::parse_str::<syn::Type>(&s).expect("Can't parse type"))
                .collect::<Vec<_>>();
            quote! {
                (#(#arg_types),*)
            }
        } else {
            let resolved = self
                .args
                .first()
                .expect("Command must have at least one argument");
            let arg_type =
                syn::parse_str::<syn::Type>(&resolved.rust_type()).expect("Can't parse type");
            quote! {
                #arg_type
            }
        };
        let response_args =
            syn::parse_str::<syn::Type>(&self.r_type.rust_type()).expect("Can't parse type");
        quote! {
            type #request = #request_args;
            type #response = #response_args;
            type #command_name = Fn(#request) -> #response;
        }
    }
}

impl Generator for Component {
    fn generate_one(&self) -> TokenStream {
        let enums = <Enum as Generator>::generate_multiple(&self.enums);
        let types = <Type as Generator>::generate_multiple(&self.types);
        let members = <Member as Generator>::generate_multiple(&self.members);
        let commands = self
            .commands
            .iter()
            .map(|c| c.get_member(&self.name))
            .collect::<Vec<_>>();

        let commands_types = self
            .commands
            .iter()
            .map(|c| c.get_types(&self.name))
            .collect::<Vec<_>>();
        let comments = &self.comments;
        let id = &self.id;
        let name = format_ident!("{}", &self.name);

        let comma = if commands.len() > 0 && members.to_string().len() > 0 {
            quote! { , }
        } else {
            quote! {}
        };
        quote! {
            #enums

            #types

            #[allow(dead_code)]
            #(#[doc = #comments])*
            #[derive(SpatialComponent)]
            #[id(#id)]
            pub struct #name {
                #members

                #comma

                #(#commands),*
            }

            #(#commands_types)*
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
            #[derive(SpatialEnum, Debug, Clone)]
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

            #[allow(dead_code)]
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
