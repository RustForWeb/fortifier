use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, Result};

use crate::{
    validation::Validation,
    validations::{Email, Length, Url},
};

pub struct ValidateField {
    expr: TokenStream,
    validations: Vec<Box<dyn Validation>>,
}

impl ValidateField {
    pub fn parse(expr: TokenStream, field: &Field) -> Result<Self> {
        let mut result = Self {
            expr,
            validations: vec![],
        };

        for attr in &field.attrs {
            if attr.path().is_ident("validate") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("email") {
                        result.validations.push(Box::new(Email::parse(&meta)?));

                        Ok(())
                    } else if meta.path.is_ident("length") {
                        result.validations.push(Box::new(Length::parse(&meta)?));

                        Ok(())
                    } else if meta.path.is_ident("url") {
                        result.validations.push(Box::new(Url::parse(&meta)?));

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

        self.validations
            .first()
            .map(|validation| validation.error_type())
            .unwrap_or_else(|| quote!(()))
    }

    pub fn sync_validations(&self) -> Vec<TokenStream> {
        self.validations
            .iter()
            .filter(|validation| !validation.is_async())
            .map(|validation| validation.tokens(&self.expr))
            .collect()
    }

    pub fn async_validations(&self) -> Vec<TokenStream> {
        self.validations
            .iter()
            .filter(|validation| validation.is_async())
            .map(|validation| validation.tokens(&self.expr))
            .collect()
    }
}
