use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Field, Ident, Result};

use crate::{
    validation::Validation,
    validations::{Custom, Email, Length, Regex, Url},
};

pub struct ValidateField {
    error_type_ident: Ident,
    expr: TokenStream,
    validations: Vec<Box<dyn Validation>>,
}

impl ValidateField {
    pub fn parse(
        type_prefix: &Ident,
        ident: Ident,
        expr: TokenStream,
        field: &Field,
    ) -> Result<Self> {
        let error_ident = format_ident!("{}", ident.to_string().to_case(Case::UpperCamel));
        let error_type_ident = format_ident!("{type_prefix}{error_ident}ValidationError");

        let mut result = Self {
            error_type_ident,
            expr,
            validations: vec![],
        };

        for attr in &field.attrs {
            if attr.path().is_ident("validate") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("custom") {
                        result.validations.push(Box::new(Custom::parse(&meta)?));

                        Ok(())
                    } else if meta.path.is_ident("email") {
                        result.validations.push(Box::new(Email::parse(&meta)?));

                        Ok(())
                    } else if meta.path.is_ident("length") {
                        result.validations.push(Box::new(Length::parse(&meta)?));

                        Ok(())
                    } else if meta.path.is_ident("regex") {
                        result.validations.push(Box::new(Regex::parse(&meta)?));

                        Ok(())
                    } else if meta.path.is_ident("url") {
                        result.validations.push(Box::new(Url::parse(&meta)?));

                        Ok(())
                    } else {
                        Err(meta.error("unknown parameter"))
                    }
                })?;
            }
        }

        Ok(result)
    }

    pub fn error_type(
        &self,
        ident: &Ident,
        field_error_ident: &Ident,
    ) -> (TokenStream, Option<TokenStream>) {
        if self.validations.len() > 1 {
            let ident = format_ident!("{}{}ValidationError", ident, field_error_ident);
            let variant_ident = self.validations.iter().map(|validation| validation.ident());
            let variant_type = self
                .validations
                .iter()
                .map(|validation| validation.error_type());

            (
                ident.to_token_stream(),
                Some(quote! {
                    #[derive(Debug)]
                    enum #ident {
                        #( #variant_ident(#variant_type) ),*
                    }
                }),
            )
        } else if let Some(validation) = self.validations.first() {
            (validation.error_type(), None)
        } else {
            (quote!(()), None)
        }
    }

    pub fn sync_validations(&self) -> Vec<TokenStream> {
        let error_type_ident = &self.error_type_ident;

        self.validations
            .iter()
            .filter(|validation| !validation.is_async())
            .map(|validation| {
                let validation_ident = validation.ident();
                let tokens = validation.tokens(&self.expr);

                if self.validations.len() > 1 {
                    quote! {
                        #tokens.map_err(#error_type_ident::#validation_ident)
                    }
                } else {
                    tokens
                }
            })
            .collect()
    }

    pub fn async_validations(&self) -> Vec<TokenStream> {
        let error_type_ident = &self.error_type_ident;

        self.validations
            .iter()
            .filter(|validation| validation.is_async())
            .map(|validation| {
                let validation_ident = validation.ident();
                let tokens = validation.tokens(&self.expr);

                if self.validations.len() > 1 {
                    quote! {
                        #tokens.map_err(#error_type_ident::#validation_ident)
                    }
                } else {
                    tokens
                }
            })
            .collect()
    }
}
