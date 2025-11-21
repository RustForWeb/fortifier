mod validate;
mod validations;

use proc_macro::TokenStream;
use syn::{DeriveInput, Error, parse_macro_input};

use crate::validate::validate_tokens;

#[proc_macro_derive(Validate, attributes(validate))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    validate_tokens(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
