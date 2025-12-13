use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Result, meta::ParseNestedMeta};

use crate::validation::{Execution, Validation};

#[derive(Default)]
pub struct Nested {}

impl Nested {
    pub fn new() -> Self {
        Self {}
    }
}

impl Validation for Nested {
    fn parse(_meta: &ParseNestedMeta<'_>) -> Result<Self> {
        unimplemented!()
    }

    fn ident(&self) -> Ident {
        format_ident!("Nested")
    }

    fn error_type(&self) -> TokenStream {
        // TODO
        quote!(::fortifier::NestedError)
    }

    fn expr(&self, execution: Execution, expr: &TokenStream) -> Option<TokenStream> {
        match execution {
            Execution::Sync => Some(quote! {
                ::fortifier::ValidateWithContext::validate_sync_with_context(&#expr, context)
            }),
            Execution::Async => Some(quote! {
                ::fortifier::ValidateWithContext::validate_async_with_context(&#expr, context).await
            }),
        }
    }
}
