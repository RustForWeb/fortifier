use proc_macro2::TokenStream;
use syn::{Field, Ident, Result, meta::ParseNestedMeta};

#[derive(Clone, Copy)]
pub enum Execution {
    Sync,
    Async,
}

pub trait Validation {
    fn parse(_field: &Field, _meta: &ParseNestedMeta<'_>) -> Result<Self>
    where
        Self: Sized;

    fn ident(&self) -> Ident;

    fn error_type(&self) -> TokenStream;

    fn expr(&self, execution: Execution, expr: &TokenStream) -> Option<TokenStream>;
}
