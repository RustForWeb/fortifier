use convert_case::{Case, Casing};
use quote::format_ident;
use syn::{GenericArgument, Ident, PathArguments, TypePath};

pub fn upper_camel_ident(ident: &Ident) -> Ident {
    let s = ident.to_string();

    if s.starts_with("r#") {
        format_ident!("{}", (&s[2..]).to_case(Case::UpperCamel))
    } else {
        format_ident!("{}", s.to_case(Case::UpperCamel))
    }
}

pub fn generic_arguments(r#type: &TypePath) -> Vec<GenericArgument> {
    if let Some(segment) = r#type.path.segments.last()
        && let PathArguments::AngleBracketed(arguments) = &segment.arguments
    {
        arguments.args.iter().cloned().collect()
    } else {
        vec![]
    }
}
