use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Result, Type, meta::ParseNestedMeta};

use crate::{
    generics::Generic,
    validation::{Execution, Validation},
};

#[derive(Default)]
pub struct Url {}

impl Validation for Url {
    fn parse(_type: &Type, _meta: &ParseNestedMeta<'_>) -> Result<Self> {
        Ok(Url::default())
    }

    fn ident(&self) -> Ident {
        format_ident!("Url")
    }

    fn error_type(&self) -> TokenStream {
        quote!(::fortifier::UrlError)
    }

    fn error_generics(&self) -> Vec<Generic> {
        vec![]
    }

    fn error_where_predicates(&self) -> Vec<TokenStream> {
        vec![]
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
