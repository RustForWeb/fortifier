use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, Ident, Result, meta::ParseNestedMeta};

use crate::validation::Validation;

pub struct Regex {
    pub expr: Expr,
}

impl Validation for Regex {
    fn parse(meta: &ParseNestedMeta<'_>) -> Result<Self> {
        let mut expr: Option<Expr> = None;

        meta.parse_nested_meta(|meta| {
            if meta.path.is_ident("expr") {
                expr = Some(meta.value()?.parse()?);

                Ok(())
            } else {
                Err(meta.error("unknown parameter"))
            }
        })?;

        let Some(expr) = expr else {
            return Err(meta.error("missing expr parameter"));
        };

        Ok(Regex { expr })
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
        let regex_expr = &self.expr;

        quote! {
            #expr.validate_regex(#regex_expr)
        }
    }
}
