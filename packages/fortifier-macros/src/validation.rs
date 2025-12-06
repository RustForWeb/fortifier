use proc_macro2::TokenStream;
use syn::{Ident, Result, meta::ParseNestedMeta};

pub trait Validation {
    fn parse(_meta: &ParseNestedMeta<'_>) -> Result<Self>
    where
        Self: Sized;

    fn is_async(&self) -> bool;

    fn ident(&self) -> Ident;

    fn error_type(&self) -> TokenStream;

    fn tokens(&self, expr: &TokenStream) -> TokenStream;
}
