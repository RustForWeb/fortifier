use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{GenericArgument, Ident, Result, Type, TypePath, meta::ParseNestedMeta};

use crate::{
    attributes::enum_field_attributes,
    util::generic_arguments,
    validation::{Execution, Validation},
};

pub struct Nested {
    error_type: TypePath,
}

impl Nested {
    pub fn new(error_type: TypePath) -> Self {
        Self { error_type }
    }
}

impl Validation for Nested {
    fn parse(_type: &Type, meta: &ParseNestedMeta<'_>) -> Result<Self> {
        let mut error_type: Option<TypePath> = None;

        meta.parse_nested_meta(|meta| {
            if meta.path.is_ident("error_type") {
                error_type = Some(meta.value()?.parse()?);

                Ok(())
            } else {
                Err(meta.error("unknown parameter"))
            }
        })?;

        let Some(error_type) = error_type else {
            return Err(meta.error("missing `error_type` parameter"));
        };

        Ok(Nested { error_type })
    }

    fn ident(&self) -> Ident {
        format_ident!("Nested")
    }

    fn error_type(&self) -> TokenStream {
        let error_type = &self.error_type;
        let attributes = enum_field_attributes();

        quote!(#attributes ::fortifier::ValidationErrors<#error_type>)
    }

    fn error_generic_arguments(&self) -> Vec<GenericArgument> {
        generic_arguments(&self.error_type)
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
