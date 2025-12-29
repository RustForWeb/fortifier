use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, Ident, Result, Type, meta::ParseNestedMeta};

use crate::validation::{Execution, Validation};

pub struct Range {
    r#type: Type,
    min: Option<Expr>,
    max: Option<Expr>,
    exclusive_min: Option<Expr>,
    exclusive_max: Option<Expr>,
}

impl Validation for Range {
    fn parse(r#type: &Type, meta: &ParseNestedMeta<'_>) -> Result<Self> {
        let mut result = Range {
            r#type: r#type.clone(),
            min: None,
            max: None,
            exclusive_min: None,
            exclusive_max: None,
        };

        meta.parse_nested_meta(|meta| {
            if meta.path.is_ident("min") {
                let expr: Expr = meta.value()?.parse()?;
                result.min = Some(expr);

                Ok(())
            } else if meta.path.is_ident("max") {
                let expr: Expr = meta.value()?.parse()?;
                result.max = Some(expr);

                Ok(())
            } else if meta.path.is_ident("exclusive_min") {
                let expr: Expr = meta.value()?.parse()?;
                result.exclusive_min = Some(expr);

                Ok(())
            } else if meta.path.is_ident("exclusive_max") {
                let expr: Expr = meta.value()?.parse()?;
                result.exclusive_max = Some(expr);

                Ok(())
            } else {
                Err(meta.error("unknown parameter"))
            }
        })?;

        if result.min.is_some() && result.exclusive_min.is_some() {
            return Err(meta.error("`exclusive_min` and `min` are conflicting parameters"));
        }
        if result.max.is_some() && result.exclusive_max.is_some() {
            return Err(meta.error("`exclusive_max` and `max` are conflicting parameters"));
        }

        Ok(result)
    }

    fn ident(&self) -> Ident {
        format_ident!("Range")
    }

    fn error_type(&self) -> TokenStream {
        let r#type = &self.r#type;

        quote!(::fortifier::RangeError<#r#type>)
    }

    fn expr(&self, exeuction: Execution, expr: &TokenStream) -> Option<TokenStream> {
        match exeuction {
            Execution::Sync => {
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
                let exclusive_min = if let Some(exclusive_min) = &self.exclusive_min {
                    quote!(Some(#exclusive_min))
                } else {
                    quote!(None)
                };
                let exclusive_max = if let Some(exclusive_max) = &self.exclusive_max {
                    quote!(Some(#exclusive_max))
                } else {
                    quote!(None)
                };

                Some(quote! {
                    ::fortifier::ValidateRange::validate_range(&#expr, #min, #max, #exclusive_min, #exclusive_max)
                })
            }
            Execution::Async => None,
        }
    }
}
