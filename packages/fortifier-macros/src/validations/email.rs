use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Result, meta::ParseNestedMeta};

#[derive(Default)]
pub struct Email {}

pub fn parse_email(_meta: &ParseNestedMeta<'_>) -> Result<Email> {
    Ok(Email::default())
}

pub fn email_tokens(
    _email: Email,
    error_ident: &Ident,
    field_ident: &Ident,
    field_error_ident: &Ident,
) -> TokenStream {
    quote! {
        if let Err(err) = self.#field_ident.validate_email() {
            errors.push(#error_ident::#field_error_ident(err));
        }
    }
}
