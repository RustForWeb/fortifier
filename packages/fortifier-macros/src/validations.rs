mod custom;
mod email_address;
mod length;
mod nested;
mod phone_number;
mod range;
mod regex;
mod url;

pub use custom::*;
pub use email_address::*;
pub use length::*;
pub use nested::*;
pub use phone_number::*;
pub use range::*;
pub use regex::*;
pub use url::*;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::validation::{Execution, Validation};

pub fn combine_validations(
    execution: Execution,
    error_type_ident: &Ident,
    expr: &TokenStream,
    validations: &[Box<dyn Validation>],
) -> Vec<TokenStream> {
    validations
        .iter()
        .flat_map(|validation| {
            let validation_ident = validation.ident();
            let expr = validation.expr(execution, expr);

            expr.map(|expr| {
                if validations.len() > 1 {
                    quote! {
                        #expr.map_err(#error_type_ident::#validation_ident)
                    }
                } else {
                    expr
                }
            })
        })
        .collect()
}

pub fn wrap_validations(
    error_ident: &Ident,
    error_field_ident: &Ident,
    error_wrapper: &impl Fn(TokenStream) -> TokenStream,
    validations: Vec<TokenStream>,
) -> Vec<TokenStream> {
    validations
        .iter()
        .map(|validation| {
            let error = error_wrapper(quote!(#error_ident::#error_field_ident(err)));

            quote! {
                if let Err(err) = #validation {
                    errors.push(#error);
                }
            }
        })
        .collect()
}

pub fn combine_wrapped_validations(validations: Vec<TokenStream>) -> TokenStream {
    if validations.is_empty() {
        quote! {
            Ok(())
        }
    } else {
        quote! {
            let mut errors = vec![];

            #(#validations)*

            if !errors.is_empty() {
                Err(errors.into())
            } else {
                Ok(())
            }
        }
    }
}
