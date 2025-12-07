use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, Ident, Result, meta::ParseNestedMeta};

use crate::validation::Validation;

pub struct Regex {
    pub expression: Expr,
}

impl Validation for Regex {
    fn parse(meta: &ParseNestedMeta<'_>) -> Result<Self> {
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

    fn is_async(&self) -> bool {
        false
    }

    fn ident(&self) -> Ident {
        format_ident!("Regex")
    }

    fn error_type(&self) -> TokenStream {
        quote!(RegexError)
    }

    fn tokens(&self, expr: &TokenStream) -> TokenStream {
        let expression = &self.expression;

        quote! {
            #expr.validate_regex(#expression)
        }
    }
}
