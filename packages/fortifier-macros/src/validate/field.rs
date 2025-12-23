use convert_case::{Case, Casing};
use proc_macro2::{Literal, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::{Error, Field, Ident, Result, Visibility};

use crate::{
    attributes::enum_attributes,
    validate::r#type::{KnownOrUnknown, should_validate_type},
    validation::{Execution, Validation},
    validations::{Custom, EmailAddress, Length, Nested, PhoneNumber, Range, Regex, Url},
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
            LiteralOrIdent::Ident(ident) => upper_camel_ident(ident),
        };
        let error_type_ident = format_error_ident_with_prefix(type_prefix, &error_ident);

        let mut result = Self {
            visibility,
            ident,
            error_ident,
            error_type_ident,
            validations: vec![],
        };
        let mut skip_nested = false;

        for attr in &field.attrs {
            if attr.path().is_ident("validate") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("custom") {
                        result
                            .validations
                            .push(Box::new(Custom::parse(field, &meta)?));

                        Ok(())
                    } else if meta.path.is_ident("email_address") {
                        result
                            .validations
                            .push(Box::new(EmailAddress::parse(field, &meta)?));

                        Ok(())
                    } else if meta.path.is_ident("length") {
                        result
                            .validations
                            .push(Box::new(Length::parse(field, &meta)?));

                        Ok(())
                    } else if meta.path.is_ident("nested") {
                        result
                            .validations
                            .push(Box::new(Nested::parse(field, &meta)?));
                        skip_nested = true;

                        Ok(())
                    } else if meta.path.is_ident("phone_number") {
                        result
                            .validations
                            .push(Box::new(PhoneNumber::parse(field, &meta)?));

                        Ok(())
                    } else if meta.path.is_ident("range") {
                        result
                            .validations
                            .push(Box::new(Range::parse(field, &meta)?));

                        Ok(())
                    } else if meta.path.is_ident("regex") {
                        result
                            .validations
                            .push(Box::new(Regex::parse(field, &meta)?));

                        Ok(())
                    } else if meta.path.is_ident("url") {
                        result.validations.push(Box::new(Url::parse(field, &meta)?));

                        Ok(())
                    } else if meta.path.is_ident("skip") {
                        skip_nested = true;

                        Ok(())
                    } else {
                        Err(meta.error("unknown parameter"))
                    }
                })?;
            }
        }

        // TODO: Use enum/struct generics to determine if a generic field type supports nested validation.
        // TODO: Remove the validations empty check after resolving the issue above.
        if !skip_nested
            && result.validations.is_empty()
            && let Some(nested_type) = should_validate_type(&field.ty)
        {
            if let KnownOrUnknown::Known(nested_type) = nested_type {
                result.validations.push(Box::new(Nested::new(nested_type)));
            } else {
                return Err(Error::new_spanned(
                    field,
                    "error type must be specified using `#[validate(nested(error_type = MyErrorType))]`",
                ));
            }
        }

        Ok(result)
    }

    pub fn ident(&self) -> &LiteralOrIdent {
        &self.ident
    }

    pub fn error_ident(&self) -> &Ident {
        &self.error_ident
    }

    pub fn error_type(&self, ident: &Ident) -> Option<(TokenStream, Option<TokenStream>)> {
        if self.validations.len() > 1 {
            let attributes = enum_attributes();
            let visibility = &self.visibility;
            let ident = format_error_ident_with_prefix(ident, &self.error_ident);
            let variant_ident = self.validations.iter().map(|validation| validation.ident());
            let variant_type = self
                .validations
                .iter()
                .map(|validation| validation.error_type());

            Some((
                ident.to_token_stream(),
                Some(quote! {
                    #[derive(Debug, PartialEq)]
                    #attributes
                    #visibility enum #ident {
                        #( #variant_ident(#variant_type) ),*
                    }
                }),
            ))
        } else {
            self.validations
                .first()
                .map(|validation| (validation.error_type(), None))
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

fn upper_camel_ident(ident: &Ident) -> Ident {
    let s = ident.to_string();

    if s.starts_with("r#") {
        format_ident!("{}", (&s[2..]).to_case(Case::UpperCamel))
    } else {
        format_ident!("{}", s.to_case(Case::UpperCamel))
    }
}

pub fn format_error_ident(ident: &Ident) -> Ident {
    format_ident!("{}ValidationError", ident)
}

pub fn format_error_ident_with_prefix(prefix: &Ident, ident: &Ident) -> Ident {
    format_ident!("{}{}ValidationError", prefix, ident)
}
