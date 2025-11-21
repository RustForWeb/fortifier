use proc_macro2::TokenStream;
use quote::quote;
use syn::{Result, meta::ParseNestedMeta};

#[derive(Default)]
pub struct Email {}

impl Email {
    pub fn parse(_meta: &ParseNestedMeta<'_>) -> Result<Email> {
        Ok(Email::default())
    }

    pub fn tokens(&self, expr: &TokenStream) -> TokenStream {
        quote! {
            #expr.validate_email()
        }
    }
}
