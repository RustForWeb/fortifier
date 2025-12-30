use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{GenericArgument, Ident, Result, Type, meta::ParseNestedMeta};

use crate::validation::{Execution, Validation};

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

    fn error_generic_arguments(&self) -> Vec<GenericArgument> {
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
