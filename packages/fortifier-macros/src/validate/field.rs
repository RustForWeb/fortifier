use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, Result};

use crate::validations::{Email, Length};

pub struct ValidateField {
    expr: TokenStream,
    // TODO: Consider using a trait for validations.
    email: Option<Email>,
    length: Option<Length>,
}

impl ValidateField {
    pub fn parse(expr: TokenStream, field: &Field) -> Result<Self> {
        let mut result = Self {
            expr,
            email: None,
            length: None,
        };

        for attr in &field.attrs {
            if attr.path().is_ident("validate") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("email") {
                        result.email = Some(Email::parse(&meta)?);

                        Ok(())
                    } else if meta.path.is_ident("length") {
                        result.length = Some(Length::parse(&meta)?);

                        Ok(())
                    } else {
                        Err(meta.error("unknown validate parameter"))
                    }
                })?;
            }
        }

        Ok(result)
    }

    pub fn error_type(&self) -> TokenStream {
        // TODO: Merge error types

        if self.email.is_some() {
            quote!(EmailError)
        } else if self.length.is_some() {
            quote!(LengthError<usize>)
        } else {
            quote!(())
        }
    }

    pub fn sync_validations(&self) -> Vec<TokenStream> {
        let email = self.email.as_ref().map(|email| email.tokens(&self.expr));
        let length = self.length.as_ref().map(|length| length.tokens(&self.expr));

        [email, length].into_iter().flatten().collect()
    }

    pub fn async_validations(&self) -> Vec<TokenStream> {
        vec![]
    }
}
