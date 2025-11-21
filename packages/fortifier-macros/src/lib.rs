#![warn(missing_docs)]

//! Fortifier macros.

mod validate;
mod validations;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{DeriveInput, Error, parse_macro_input};

use crate::validate::Validate;

/// Validate derive macro.
#[proc_macro_derive(Validate, attributes(validate))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    Validate::parse(input)
        .map(|validate| validate.to_token_stream())
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
