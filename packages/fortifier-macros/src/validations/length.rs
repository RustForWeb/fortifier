use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, Result, meta::ParseNestedMeta};

#[derive(Default)]
pub struct Length {
    pub equal: Option<Expr>,
    pub min: Option<Expr>,
    pub max: Option<Expr>,
}

impl Length {
    pub fn parse(meta: &ParseNestedMeta<'_>) -> Result<Length> {
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
                Err(meta.error("unknown length parameter"))
            }
        })?;

        Ok(result)
    }

    pub fn tokens(&self, expr: &TokenStream) -> TokenStream {
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

        quote! {
            #expr.validate_length(#equal, #min, #max)
        }
    }
}
