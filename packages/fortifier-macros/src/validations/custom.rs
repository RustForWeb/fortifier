use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{LitBool, Path, Result, Type, meta::ParseNestedMeta};

use crate::validation::Validation;

pub struct Custom {
    is_async: bool,
    error_type: Type,
    function_path: Path,
}

impl Validation for Custom {
    fn parse(meta: &ParseNestedMeta<'_>) -> Result<Self> {
        let mut is_async = false;
        let mut error_type: Option<Type> = None;
        let mut function_path: Option<Path> = None;

        meta.parse_nested_meta(|meta| {
            if meta.path.is_ident("async") {
                if let Ok(value) = meta.value() {
                    let lit: LitBool = value.parse()?;
                    is_async = lit.value;
                } else {
                    is_async = true;
                }

                Ok(())
            } else if meta.path.is_ident("error") {
                error_type = Some(meta.value()?.parse()?);

                Ok(())
            } else if meta.path.is_ident("function") {
                function_path = Some(meta.value()?.parse()?);

                Ok(())
            } else {
                Err(meta.error("unknown parameter"))
            }
        })?;

        let Some(error_type) = error_type else {
            return Err(meta.error("missing error parameter"));
        };
        let Some(function_path) = function_path else {
            return Err(meta.error("missing function parameter"));
        };

        Ok(Custom {
            is_async,
            error_type,
            function_path,
        })
    }

    fn is_async(&self) -> bool {
        self.is_async
    }

    fn error_type(&self) -> TokenStream {
        self.error_type.to_token_stream()
    }

    fn tokens(&self, expr: &TokenStream) -> TokenStream {
        let function_path = &self.function_path;

        if self.is_async {
            quote! {
                #function_path(&#expr).await
            }
        } else {
            quote! {
                #function_path(&#expr)
            }
        }
    }
}
