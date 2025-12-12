use convert_case::{Case, Casing};
use proc_macro2::{Literal, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::{Field, Ident, Result, Visibility};

use crate::{
    validate::{attributes::enum_attributes, r#type::should_validate_type},
    validation::{Execution, Validation},
    validations::{Custom, Email, Length, Regex, Url},
};

pub enum LiteralOrIdent {
    Literal(Literal),
    Ident(Ident),
}

impl ToTokens for LiteralOrIdent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            LiteralOrIdent::Literal(literal) => literal.to_tokens(tokens),
            LiteralOrIdent::Ident(ident) => ident.to_tokens(tokens),
        }
    }
}

#[derive(Clone, Copy)]
pub enum ValidateFieldPrefix {
    None,
    SelfKeyword,
    F,
}

pub struct ValidateField<'a> {
    visibility: &'a Visibility,
    ident: LiteralOrIdent,
    error_ident: Ident,
    error_type_ident: Ident,
    validations: Vec<Box<dyn Validation>>,
}

impl<'a> ValidateField<'a> {
    pub fn parse(
        visibility: &'a Visibility,
        type_prefix: &Ident,
        ident: LiteralOrIdent,
        field: &Field,
    ) -> Result<Self> {
        let error_ident = match &ident {
            LiteralOrIdent::Literal(literal) => format_ident!("F{literal}"),
            LiteralOrIdent::Ident(ident) => {
                format_ident!("{}", ident.to_string().to_case(Case::UpperCamel))
            }
        };
        let error_type_ident = format_ident!("{type_prefix}{error_ident}ValidationError");

        let mut result = Self {
            visibility,
            ident,
            error_ident,
            error_type_ident,
            validations: vec![],
        };
        let mut skip = false;

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
                    } else if meta.path.is_ident("skip") {
                        skip = true;

                        Ok(())
                    } else {
                        Err(meta.error("unknown parameter"))
                    }
                })?;
            }
        }

        if !skip && should_validate_type(&field.ty) {
            // TODO: Nested validation
        }

        Ok(result)
    }

    pub fn ident(&self) -> &LiteralOrIdent {
        &self.ident
    }

    pub fn error_ident(&self) -> &Ident {
        &self.error_ident
    }

    pub fn error_type(&self, ident: &Ident) -> (TokenStream, Option<TokenStream>) {
        if self.validations.len() > 1 {
            let attributes = enum_attributes();
            let visibility = &self.visibility;
            let ident = format_ident!("{}{}ValidationError", ident, self.error_ident);
            let variant_ident = self.validations.iter().map(|validation| validation.ident());
            let variant_type = self
                .validations
                .iter()
                .map(|validation| validation.error_type());

            (
                ident.to_token_stream(),
                Some(quote! {
                    #[derive(Debug, PartialEq)]
                    #attributes
                    #visibility enum #ident {
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

    pub fn validations(
        &self,
        execution: Execution,
        field_prefix: ValidateFieldPrefix,
    ) -> Vec<TokenStream> {
        let error_type_ident = &self.error_type_ident;
        let ident = &self.ident;

        let field_expr = match field_prefix {
            ValidateFieldPrefix::None => self.ident.to_token_stream(),
            ValidateFieldPrefix::SelfKeyword => quote!(self.#ident),
            ValidateFieldPrefix::F => match &self.ident {
                LiteralOrIdent::Literal(literal) => format_ident!("f{literal}").to_token_stream(),
                LiteralOrIdent::Ident(ident) => ident.to_token_stream(),
            },
        };

        self.validations
            .iter()
            .flat_map(|validation| {
                let validation_ident = validation.ident();
                let expr = validation.expr(execution, &field_expr);

                expr.map(|expr| {
                    if self.validations.len() > 1 {
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
}
