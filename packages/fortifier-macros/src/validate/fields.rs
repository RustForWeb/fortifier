use proc_macro2::{Literal, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::{Fields, FieldsNamed, FieldsUnnamed, Ident, Result, Visibility};

use crate::{
    validate::{
        attributes::enum_attributes,
        field::{LiteralOrIdent, ValidateField, ValidateFieldPrefix},
    },
    validation::Execution,
};

pub enum ValidateFields {
    Named(ValidateNamedFields),
    Unnamed(ValidateUnnamedFields),
    Unit(ValidateUnitFields),
}

impl ValidateFields {
    pub fn parse(visibility: &Visibility, ident: &Ident, fields: &Fields) -> Result<Self> {
        Ok(match fields {
            Fields::Named(fields) => {
                Self::Named(ValidateNamedFields::parse(visibility, ident, fields)?)
            }
            Fields::Unnamed(fields) => {
                Self::Unnamed(ValidateUnnamedFields::parse(visibility, ident, fields)?)
            }
            Fields::Unit => Self::Unit(ValidateUnitFields::parse()?),
        })
    }

    pub fn error_type(&self) -> (TokenStream, TokenStream) {
        match self {
            ValidateFields::Named(named) => named.error_type(),
            ValidateFields::Unnamed(unnamed) => unnamed.error_type(),
            ValidateFields::Unit(unit) => unit.error_type(),
        }
    }

    pub fn validations(
        &self,
        execution: Execution,
        field_prefix: ValidateFieldPrefix,
        error_wrapper: &impl Fn(TokenStream) -> TokenStream,
    ) -> TokenStream {
        match self {
            ValidateFields::Named(named) => {
                named.validations(execution, field_prefix, error_wrapper)
            }
            ValidateFields::Unnamed(unnamed) => {
                unnamed.validations(execution, field_prefix, error_wrapper)
            }
            ValidateFields::Unit(unit) => unit.validations(),
        }
    }
}

pub struct ValidateNamedFields {
    visibility: Visibility,
    ident: Ident,
    error_ident: Ident,
    fields: Vec<ValidateField>,
}

impl ValidateNamedFields {
    fn parse(visibility: &Visibility, ident: &Ident, fields: &FieldsNamed) -> Result<Self> {
        let mut result = Self {
            visibility: visibility.clone(),
            ident: ident.clone(),
            error_ident: format_ident!("{}ValidationError", ident),
            fields: Vec::with_capacity(fields.named.len()),
        };

        for field in &fields.named {
            let Some(field_ident) = &field.ident else {
                continue;
            };

            result.fields.push(ValidateField::parse(
                visibility,
                ident,
                LiteralOrIdent::Ident(field_ident.clone()),
                field,
            )?);
        }

        Ok(result)
    }

    pub fn idents(&self) -> impl Iterator<Item = &LiteralOrIdent> {
        self.fields.iter().map(|field| field.ident())
    }

    fn error_type(&self) -> (TokenStream, TokenStream) {
        error_type(
            &self.visibility,
            &self.ident,
            &self.error_ident,
            self.fields.iter(),
        )
    }

    pub fn validations(
        &self,
        execution: Execution,
        field_prefix: ValidateFieldPrefix,
        error_wrapper: &impl Fn(TokenStream) -> TokenStream,
    ) -> TokenStream {
        validations(
            execution,
            field_prefix,
            &self.error_ident,
            error_wrapper,
            self.fields.iter(),
        )
    }
}

pub struct ValidateUnnamedFields {
    visibility: Visibility,
    ident: Ident,
    error_ident: Ident,
    fields: Vec<ValidateField>,
}

impl ValidateUnnamedFields {
    fn parse(visibility: &Visibility, ident: &Ident, fields: &FieldsUnnamed) -> Result<Self> {
        let mut result = Self {
            visibility: visibility.clone(),
            ident: ident.clone(),
            error_ident: format_ident!("{}ValidationError", ident),
            fields: Vec::with_capacity(fields.unnamed.len()),
        };

        for (index, field) in fields.unnamed.iter().enumerate() {
            result.fields.push(ValidateField::parse(
                visibility,
                ident,
                LiteralOrIdent::Literal(Literal::usize_unsuffixed(index)),
                field,
            )?);
        }

        Ok(result)
    }

    pub fn idents(&self) -> impl Iterator<Item = &LiteralOrIdent> {
        self.fields.iter().map(|field| field.ident())
    }

    fn error_type(&self) -> (TokenStream, TokenStream) {
        error_type(
            &self.visibility,
            &self.ident,
            &self.error_ident,
            self.fields.iter(),
        )
    }

    pub fn validations(
        &self,
        execution: Execution,
        field_prefix: ValidateFieldPrefix,
        error_wrapper: &impl Fn(TokenStream) -> TokenStream,
    ) -> TokenStream {
        validations(
            execution,
            field_prefix,
            &self.error_ident,
            error_wrapper,
            self.fields.iter(),
        )
    }
}

pub struct ValidateUnitFields {}

impl ValidateUnitFields {
    fn parse() -> Result<Self> {
        Ok(Self {})
    }

    fn error_type(&self) -> (TokenStream, TokenStream) {
        (quote!(::std::convert::Infallible), TokenStream::new())
    }

    pub fn validations(&self) -> TokenStream {
        quote! {
            Ok(())
        }
    }
}

fn error_type<'a>(
    visibility: &Visibility,
    ident: &Ident,
    error_ident: &Ident,
    fields: impl Iterator<Item = &'a ValidateField>,
) -> (TokenStream, TokenStream) {
    let attributes = enum_attributes();

    let mut error_field_idents = vec![];
    let mut error_field_types = vec![];
    let mut error_field_enums = vec![];

    for field in fields {
        let field_error_ident = field.error_ident();
        let (field_error_type, field_error_enum) = field.error_type(ident);

        error_field_idents.push(field_error_ident.clone());
        error_field_types.push(field_error_type);
        if let Some(error_enum) = field_error_enum {
            error_field_enums.push(error_enum);
        }
    }

    (
        error_ident.to_token_stream(),
        quote! {
            #[allow(dead_code)]
            #[derive(Debug)]
            #attributes
            #visibility enum #error_ident {
                #( #error_field_idents(#error_field_types) ),*
            }

            #[automatically_derived]
            impl ::std::fmt::Display for #error_ident {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, "{self:#?}")
                }
            }

            #[automatically_derived]
            impl ::std::error::Error for #error_ident {}

            #( #error_field_enums )*
        },
    )
}

fn validations<'a>(
    execution: Execution,
    field_prefix: ValidateFieldPrefix,
    error_ident: &Ident,
    error_wrapper: &impl Fn(TokenStream) -> TokenStream,
    fields: impl Iterator<Item = &'a ValidateField>,
) -> TokenStream {
    let error_ident = &error_ident;

    let validations = fields
        .flat_map(|field| {
            let field_error_ident = field.error_ident();
            let validations = field.validations(execution, field_prefix);

            validations
                .iter()
                .map(|validation| {
                    let error = error_wrapper(quote!(#error_ident::#field_error_ident(err)));

                    quote! {
                        if let Err(err) = #validation {
                            errors.push(#error);
                        }
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

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
