use proc_macro2::TokenStream;
use quote::quote;
use syn::{Result, meta::ParseNestedMeta};

#[derive(Default)]
pub struct Url {}

impl Url {
    pub fn parse(_meta: &ParseNestedMeta<'_>) -> Result<Url> {
        Ok(Url::default())
    }

    pub fn tokens(&self, expr: &TokenStream) -> TokenStream {
        quote! {
            #expr.validate_url()
        }
    }
}
