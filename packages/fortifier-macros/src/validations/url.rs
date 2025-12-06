use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Result, meta::ParseNestedMeta};

use crate::validation::Validation;

#[derive(Default)]
pub struct Url {}

impl Validation for Url {
    fn parse(_meta: &ParseNestedMeta<'_>) -> Result<Self> {
        Ok(Url::default())
    }

    fn is_async(&self) -> bool {
        false
    }

    fn ident(&self) -> Ident {
        format_ident!("Url")
    }

    fn error_type(&self) -> TokenStream {
        quote!(UrlError)
    }

    fn tokens(&self, expr: &TokenStream) -> TokenStream {
        quote! {
            #expr.validate_url()
        }
    }
}
