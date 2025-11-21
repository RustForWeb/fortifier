use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, Ident, Result, meta::ParseNestedMeta};

#[derive(Default)]
pub struct Length {
    pub equal: Option<Expr>,
    pub min: Option<Expr>,
    pub max: Option<Expr>,
}

pub fn parse_length(meta: &ParseNestedMeta<'_>) -> Result<Length> {
    let mut length = Length::default();

    meta.parse_nested_meta(|meta| {
        if meta.path.is_ident("equal") {
            let expr: Expr = meta.value()?.parse()?;
            length.equal = Some(expr);

            Ok(())
        } else if meta.path.is_ident("min") {
            let expr: Expr = meta.value()?.parse()?;
            length.min = Some(expr);

            Ok(())
        } else if meta.path.is_ident("max") {
            let expr: Expr = meta.value()?.parse()?;
            length.max = Some(expr);

            Ok(())
        } else {
            Err(meta.error("unknown length parameter"))
        }
    })?;

    Ok(length)
}

pub fn length_tokens(
    length: Length,
    error_ident: &Ident,
    field_ident: &Ident,
    field_error_ident: &Ident,
) -> TokenStream {
    let equal = if let Some(equal) = length.equal {
        quote!(Some(#equal))
    } else {
        quote!(None)
    };
    let min = if let Some(min) = length.min {
        quote!(Some(#min))
    } else {
        quote!(None)
    };
    let max = if let Some(max) = length.max {
        quote!(Some(#max))
    } else {
        quote!(None)
    };

    quote! {
        if let Err(err) = self.#field_ident.validate_length(#equal, #min, #max) {
            errors.push(#error_ident::#field_error_ident(err));
        }
    }
}
