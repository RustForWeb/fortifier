mod r#enum;
mod r#struct;
mod r#union;

use proc_macro2::TokenStream;
use quote::format_ident;
use syn::{Data, DeriveInput, Result};

use crate::validate::{
    r#enum::validate_enum, r#struct::validate_struct_tokens, union::validate_union,
};

pub fn validate_tokens(input: DeriveInput) -> Result<TokenStream> {
    let ident = input.ident;
    let error_ident = format_ident!("{ident}ValidationError");

    match input.data {
        Data::Struct(data) => validate_struct_tokens(ident, error_ident, data),
        Data::Enum(data) => validate_enum(ident, error_ident, data),
        Data::Union(data) => validate_union(ident, error_ident, data),
    }
}
