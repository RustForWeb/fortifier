use proc_macro2::TokenStream;
use quote::quote;
use syn::{Result, meta::ParseNestedMeta};

use crate::validation::Validation;

#[derive(Default)]
pub struct Email {}

impl Validation for Email {
    fn parse(_meta: &ParseNestedMeta<'_>) -> Result<Self> {
        Ok(Email::default())
    }

    fn is_async(&self) -> bool {
        false
    }

    fn error_type(&self) -> TokenStream {
        quote!(EmailError)
    }

    fn tokens(&self, expr: &TokenStream) -> TokenStream {
        quote! {
            #expr.validate_email()
        }
    }
}
