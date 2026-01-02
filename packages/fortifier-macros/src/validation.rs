use proc_macro2::TokenStream;
use syn::{GenericArgument, Ident, Result, Type, meta::ParseNestedMeta};

#[derive(Clone, Copy)]
pub enum Execution {
    Sync,
    Async,
}

pub trait Validation {
    fn parse(_type: &Type, _meta: &ParseNestedMeta<'_>) -> Result<Self>
    where
        Self: Sized;

    fn ident(&self) -> Ident;

    fn error_type(&self) -> TokenStream;

    fn error_generic_arguments(&self) -> Vec<GenericArgument>;

    fn expr(&self, execution: Execution, expr: &TokenStream) -> Option<TokenStream>;
}
