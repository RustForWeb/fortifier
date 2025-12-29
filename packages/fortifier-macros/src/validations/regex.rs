use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, Ident, Result, Type, meta::ParseNestedMeta};

use crate::validation::{Execution, Validation};

pub struct Regex {
    expression: Expr,
}

impl Validation for Regex {
    fn parse(_type: &Type, meta: &ParseNestedMeta<'_>) -> Result<Self> {
        let mut expression: Option<Expr> = None;

        if let Ok(value) = meta.value() {
            expression = Some(value.parse()?);
        } else {
            meta.parse_nested_meta(|meta| {
                if meta.path.is_ident("expression") {
                    expression = Some(meta.value()?.parse()?);

                    Ok(())
                } else {
                    Err(meta.error("unknown parameter"))
                }
            })?;
        }

        let Some(expression) = expression else {
            return Err(meta.error("missing expression parameter"));
        };

        Ok(Regex { expression })
    }

    fn ident(&self) -> Ident {
        format_ident!("Regex")
    }

    fn error_type(&self) -> TokenStream {
        quote!(::fortifier::RegexError)
    }

    fn expr(&self, execution: Execution, expr: &TokenStream) -> Option<TokenStream> {
        match execution {
            Execution::Sync => {
                let expression = &self.expression;

                Some(quote! {
                    ::fortifier::ValidateRegex::validate_regex(&#expr, #expression)
                })
            }
            Execution::Async => None,
        }
    }
}
