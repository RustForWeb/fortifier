use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Field, Ident, Result, meta::ParseNestedMeta};

use crate::validation::{Execution, Validation};

#[derive(Default)]
pub struct Url {}

impl Validation for Url {
    fn parse(_field: &Field, _meta: &ParseNestedMeta<'_>) -> Result<Self> {
        Ok(Url::default())
    }

    fn ident(&self) -> Ident {
        format_ident!("Url")
    }

    fn error_type(&self) -> TokenStream {
        quote!(::fortifier::UrlError)
    }

    fn expr(&self, execution: Execution, expr: &TokenStream) -> Option<TokenStream> {
        match execution {
            Execution::Sync => Some(quote! {
                ::fortifier::ValidateUrl::validate_url(&#expr)
            }),
            Execution::Async => None,
        }
    }
}
