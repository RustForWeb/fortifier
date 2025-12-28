use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Ident, LitBool, Path, Result, Type, meta::ParseNestedMeta};

use crate::validation::{Execution, Validation};

pub struct Custom {
    execution: Execution,
    error_type: Type,
    function_path: Path,
    context: bool,
}

impl Validation for Custom {
    fn parse(_type: &Type, meta: &ParseNestedMeta<'_>) -> Result<Self> {
        let mut execution = Execution::Sync;
        let mut error_type: Option<Type> = None;
        let mut function_path: Option<Path> = None;
        let mut context = false;

        meta.parse_nested_meta(|meta| {
            if meta.path.is_ident("async") {
                if let Ok(value) = meta.value() {
                    let lit: LitBool = value.parse()?;
                    execution = if lit.value {
                        Execution::Async
                    } else {
                        Execution::Sync
                    };
                } else {
                    execution = Execution::Async;
                }

                Ok(())
            } else if meta.path.is_ident("context") {
                if let Ok(value) = meta.value() {
                    let lit: LitBool = value.parse()?;
                    context = lit.value;
                } else {
                    context = true;
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
            execution,
            error_type,
            function_path,
            context,
        })
    }

    fn ident(&self) -> Ident {
        // TODO: Determine ident from function or error type.
        format_ident!("Custom")
    }

    fn error_type(&self) -> TokenStream {
        self.error_type.to_token_stream()
    }

    fn expr(&self, execution: Execution, expr: &TokenStream) -> Option<TokenStream> {
        let context_expr = self.context.then(|| quote!(, &context));

        match (execution, self.execution) {
            (Execution::Sync, Execution::Sync) => {
                let function_path = &self.function_path;

                Some(quote! {
                    #function_path(&#expr #context_expr)
                })
            }
            (Execution::Async, Execution::Async) => {
                let function_path = &self.function_path;

                Some(quote! {
                    #function_path(&#expr #context_expr).await
                })
            }
            _ => None,
        }
    }
}
