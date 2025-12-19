use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Ident, Path, Result, meta::ParseNestedMeta};

use crate::validation::{Execution, Validation};

#[derive(Default)]
pub struct Nested {
    error_type: TokenStream,
}

impl Nested {
    pub fn new(error_type: TokenStream) -> Self {
        Self { error_type }
    }
}

impl Validation for Nested {
    fn parse(meta: &ParseNestedMeta<'_>) -> Result<Self> {
        let mut error_type: Option<Path> = None;

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

        Ok(Nested {
            error_type: error_type.to_token_stream(),
        })
    }

    fn ident(&self) -> Ident {
        format_ident!("Nested")
    }

    fn error_type(&self) -> TokenStream {
        self.error_type.clone()
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
