use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, GenericArgument, Ident, Result, Type, meta::ParseNestedMeta};

use crate::validation::{Execution, Validation};

#[derive(Default)]
pub struct PhoneNumber {
    allowed_countries: Option<Expr>,
    default_country: Option<Expr>,
}

impl Validation for PhoneNumber {
    fn parse(_type: &Type, meta: &ParseNestedMeta<'_>) -> Result<Self> {
        let mut result = PhoneNumber::default();

        if !meta.input.is_empty() {
            meta.parse_nested_meta(|meta| {
                if meta.path.is_ident("allowed_countries") {
                    let expr: Expr = meta.value()?.parse()?;
                    result.allowed_countries = Some(expr);

                    Ok(())
                } else if meta.path.is_ident("default_country") {
                    let expr: Expr = meta.value()?.parse()?;
                    result.default_country = Some(expr);

                    Ok(())
                } else {
                    Err(meta.error("unknown parameter"))
                }
            })?;
        }

        Ok(result)
    }

    fn ident(&self) -> Ident {
        format_ident!("PhoneNumber")
    }
    fn error_type(&self) -> TokenStream {
        quote!(::fortifier::PhoneNumberError)
    }

    fn error_generic_arguments(&self) -> Vec<GenericArgument> {
        vec![]
    }

    fn expr(&self, execution: Execution, expr: &TokenStream) -> Option<TokenStream> {
        match execution {
            Execution::Sync => {
                let allowed_countries = match &self.allowed_countries {
                    Some(allowed_countries) => quote!(Some(#allowed_countries)),
                    None => quote!(None),
                };
                let default_country = match &self.default_country {
                    Some(default_country) => quote!(Some(#default_country)),
                    None => quote!(None),
                };

                Some(quote! {
                    ::fortifier::ValidatePhoneNumber::validate_phone_number(&#expr, #default_country, #allowed_countries)
                })
            }
            Execution::Async => None,
        }
    }
}
