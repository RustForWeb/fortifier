use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, Field, Ident, Result, meta::ParseNestedMeta};

use crate::validation::{Execution, Validation};

#[derive(Default)]
pub struct Length {
    equal: Option<Expr>,
    min: Option<Expr>,
    max: Option<Expr>,
}

impl Validation for Length {
    fn parse(_field: &Field, meta: &ParseNestedMeta<'_>) -> Result<Self> {
        let mut result = Length::default();

        meta.parse_nested_meta(|meta| {
            if meta.path.is_ident("equal") {
                let expr: Expr = meta.value()?.parse()?;
                result.equal = Some(expr);

                Ok(())
            } else if meta.path.is_ident("min") {
                let expr: Expr = meta.value()?.parse()?;
                result.min = Some(expr);

                Ok(())
            } else if meta.path.is_ident("max") {
                let expr: Expr = meta.value()?.parse()?;
                result.max = Some(expr);

                Ok(())
            } else {
                Err(meta.error("unknown parameter"))
            }
        })?;

        if result.equal.is_some() {
            if result.min.is_some() {
                return Err(meta.error("`equal` and `min` are conflicting parameters"));
            } else if result.max.is_some() {
                return Err(meta.error("`equal` and `max` are conflicting parameters"));
            }
        }

        Ok(result)
    }

    fn ident(&self) -> Ident {
        format_ident!("Length")
    }

    fn error_type(&self) -> TokenStream {
        quote!(::fortifier::LengthError<usize>)
    }

    fn expr(&self, exeuction: Execution, expr: &TokenStream) -> Option<TokenStream> {
        match exeuction {
            Execution::Sync => {
                let equal = if let Some(equal) = &self.equal {
                    quote!(Some(#equal))
                } else {
                    quote!(None)
                };
                let min = if let Some(min) = &self.min {
                    quote!(Some(#min))
                } else {
                    quote!(None)
                };
                let max = if let Some(max) = &self.max {
                    quote!(Some(#max))
                } else {
                    quote!(None)
                };

                Some(quote! {
                    ::fortifier::ValidateLength::validate_length(&#expr, #equal, #min, #max)
                })
            }
            Execution::Async => None,
        }
    }
}
