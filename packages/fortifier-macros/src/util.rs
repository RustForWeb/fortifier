use convert_case::{Case, Casing};
use quote::format_ident;
use syn::Ident;

pub fn upper_camel_ident(ident: &Ident) -> Ident {
    let s = ident.to_string();

    if s.starts_with("r#") {
        format_ident!("{}", (&s[2..]).to_case(Case::UpperCamel))
    } else {
        format_ident!("{}", s.to_case(Case::UpperCamel))
    }
}
