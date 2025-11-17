mod derive;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

use crate::derive::validate_tokens;

#[proc_macro_derive(Validate, attributes(validate))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    validate_tokens(input).into()
}
