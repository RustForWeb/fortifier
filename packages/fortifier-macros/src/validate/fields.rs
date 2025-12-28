use std::iter::empty;

use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::{Fields, FieldsNamed, FieldsUnnamed, Ident, Result, Visibility};

use crate::{
    validate::{
        error::{
            ErrorType, combined_error_type, format_error_ident, format_error_ident_with_prefix,
        },
        field::{LiteralOrIdent, ValidateField, ValidateFieldPrefix},
    },
    validation::{Execution, Validation},
    validations::{combine_validations, combine_wrapped_validations, wrap_validations},
};

pub enum ValidateFields<'a> {
    Named(ValidateNamedFields<'a>),
    Unnamed(ValidateUnnamedFields<'a>),
    Unit(ValidateUnitFields<'a>),
}

impl<'a> ValidateFields<'a> {
    pub fn parse(visibility: &'a Visibility, ident: Ident, fields: &'a Fields) -> Result<Self> {
        Ok(match fields {
            Fields::Named(fields) => {
                Self::Named(ValidateNamedFields::parse(visibility, ident, fields)?)
            }
            Fields::Unnamed(fields) => {
                Self::Unnamed(ValidateUnnamedFields::parse(visibility, ident, fields)?)
            }
            Fields::Unit => Self::Unit(ValidateUnitFields::parse(visibility, ident)?),
        })
    }

    pub fn error_type(
        &self,
        error_variant_ident: Option<&Ident>,
        root_error_type: Option<&ErrorType>,
    ) -> Option<ErrorType> {
        match self {
            ValidateFields::Named(named) => named.error_type(error_variant_ident, root_error_type),
            ValidateFields::Unnamed(unnamed) => {
                unnamed.error_type(error_variant_ident, root_error_type)
            }
            ValidateFields::Unit(unit) => unit.error_type(error_variant_ident, root_error_type),
        }
    }

    pub fn validations(
        &self,
        execution: Execution,
        field_prefix: ValidateFieldPrefix,
        error_wrapper: &impl Fn(TokenStream) -> TokenStream,
        root_type_prefix: &Ident,
        root_error_ident: &Ident,
        root_validations: &[Box<dyn Validation>],
    ) -> TokenStream {
        match self {
            ValidateFields::Named(named) => named.validations(
                execution,
                field_prefix,
                error_wrapper,
                root_type_prefix,
                root_error_ident,
                root_validations,
            ),
            ValidateFields::Unnamed(unnamed) => unnamed.validations(
                execution,
                field_prefix,
                error_wrapper,
                root_type_prefix,
                root_error_ident,
                root_validations,
            ),
            ValidateFields::Unit(unit) => unit.validations(
                execution,
                field_prefix,
                error_wrapper,
                root_type_prefix,
                root_error_ident,
                root_validations,
            ),
        }
    }
}

pub struct ValidateNamedFields<'a> {
    visibility: &'a Visibility,
    ident: Ident,
    error_ident: Ident,
    fields: Vec<ValidateField<'a>>,
}

impl<'a> ValidateNamedFields<'a> {
    fn parse(visibility: &'a Visibility, ident: Ident, fields: &'a FieldsNamed) -> Result<Self> {
        let error_ident = format_error_ident(&ident);

        let mut result = Self {
            visibility,
            ident,
            error_ident,
            fields: Vec::with_capacity(fields.named.len()),
        };

        for field in &fields.named {
            let Some(field_ident) = &field.ident else {
                continue;
            };

            result.fields.push(ValidateField::parse(
                visibility,
                &result.ident,
                LiteralOrIdent::Ident(field_ident.clone()),
                field,
            )?);
        }

        Ok(result)
    }

    pub fn idents(&self) -> impl Iterator<Item = &LiteralOrIdent> {
        self.fields.iter().map(|field| field.ident())
    }

    fn error_type(
        &self,
        error_variant_ident: Option<&Ident>,
        root_error_type: Option<&ErrorType>,
    ) -> Option<ErrorType> {
        if self.fields.is_empty() {
            None
        } else {
            Some(error_type(
                self.visibility,
                &self.ident,
                error_variant_ident,
                &self.error_ident,
                self.fields.iter(),
                root_error_type,
            ))
        }
    }

    pub fn validations(
        &self,
        execution: Execution,
        field_prefix: ValidateFieldPrefix,
        error_wrapper: &impl Fn(TokenStream) -> TokenStream,
        root_type_prefix: &Ident,
        root_error_ident: &Ident,
        root_validations: &[Box<dyn Validation>],
    ) -> TokenStream {
        validations(
            execution,
            field_prefix,
            &self.error_ident,
            error_wrapper,
            self.fields.iter(),
            root_type_prefix,
            root_error_ident,
            root_validations,
        )
    }
}

pub struct ValidateUnnamedFields<'a> {
    visibility: &'a Visibility,
    ident: Ident,
    error_ident: Ident,
    fields: Vec<ValidateField<'a>>,
}

impl<'a> ValidateUnnamedFields<'a> {
    fn parse(visibility: &'a Visibility, ident: Ident, fields: &'a FieldsUnnamed) -> Result<Self> {
        let error_ident = format_error_ident(&ident);

        let mut result = Self {
            visibility,
            ident,
            error_ident,
            fields: Vec::with_capacity(fields.unnamed.len()),
        };

        for (index, field) in fields.unnamed.iter().enumerate() {
            result.fields.push(ValidateField::parse(
                visibility,
                &result.ident,
                LiteralOrIdent::Literal(Literal::usize_unsuffixed(index)),
                field,
            )?);
        }

        Ok(result)
    }

    pub fn idents(&self) -> impl Iterator<Item = &LiteralOrIdent> {
        self.fields.iter().map(|field| field.ident())
    }

    fn error_type(
        &self,
        error_variant_ident: Option<&Ident>,
        root_error_type: Option<&ErrorType>,
    ) -> Option<ErrorType> {
        if self.fields.is_empty() {
            None
        } else {
            Some(error_type(
                self.visibility,
                &self.ident,
                error_variant_ident,
                &self.error_ident,
                self.fields.iter(),
                root_error_type,
            ))
        }
    }

    pub fn validations(
        &self,
        execution: Execution,
        field_prefix: ValidateFieldPrefix,
        error_wrapper: &impl Fn(TokenStream) -> TokenStream,
        root_type_prefix: &Ident,
        root_error_ident: &Ident,
        root_validations: &[Box<dyn Validation>],
    ) -> TokenStream {
        validations(
            execution,
            field_prefix,
            &self.error_ident,
            error_wrapper,
            self.fields.iter(),
            root_type_prefix,
            root_error_ident,
            root_validations,
        )
    }
}

pub struct ValidateUnitFields<'a> {
    visibility: &'a Visibility,
    ident: Ident,
    error_ident: Ident,
}

impl<'a> ValidateUnitFields<'a> {
    fn parse(visibility: &'a Visibility, ident: Ident) -> Result<Self> {
        let error_ident = format_error_ident(&ident);

        Ok(Self {
            visibility,
            ident,
            error_ident,
        })
    }

    fn error_type(
        &self,
        error_variant_ident: Option<&Ident>,
        root_error_type: Option<&ErrorType>,
    ) -> Option<ErrorType> {
        if root_error_type.is_some() {
            Some(error_type(
                self.visibility,
                &self.ident,
                error_variant_ident,
                &self.error_ident,
                empty(),
                root_error_type,
            ))
        } else {
            None
        }
    }

    pub fn validations(
        &self,
        execution: Execution,
        field_prefix: ValidateFieldPrefix,
        error_wrapper: &impl Fn(TokenStream) -> TokenStream,
        root_type_prefix: &Ident,
        root_error_ident: &Ident,
        root_validations: &[Box<dyn Validation>],
    ) -> TokenStream {
        if root_validations.is_empty() {
            quote!(Ok(()))
        } else {
            validations(
                execution,
                field_prefix,
                &self.error_ident,
                error_wrapper,
                empty(),
                root_type_prefix,
                root_error_ident,
                root_validations,
            )
        }
    }
}

fn error_type<'a>(
    visibility: &Visibility,
    ident: &Ident,
    error_variant_ident: Option<&Ident>,
    error_ident: &Ident,
    fields: impl Iterator<Item = &'a ValidateField<'a>>,
    root_error_type: Option<&ErrorType>,
) -> ErrorType {
    let mut variant_idents = vec![];
    let mut variant_types = vec![];
    let mut definitions = vec![];

    for field in fields {
        if let Some(ErrorType {
            variant_ident,
            r#type,
            definition,
        }) = field.error_type(ident)
        {
            variant_idents.push(variant_ident);
            variant_types.push(r#type);
            if let Some(definition) = definition {
                definitions.push(definition);
            }
        }
    }

    combined_error_type(
        visibility,
        error_variant_ident.unwrap_or(ident),
        error_ident,
        variant_idents.iter(),
        variant_types.iter(),
        definitions.iter(),
        root_error_type,
    )
}

#[expect(clippy::too_many_arguments)]
fn validations<'a>(
    execution: Execution,
    field_prefix: ValidateFieldPrefix,
    error_ident: &Ident,
    error_wrapper: &impl Fn(TokenStream) -> TokenStream,
    fields: impl Iterator<Item = &'a ValidateField<'a>>,
    root_type_prefix: &Ident,
    root_error_ident: &Ident,
    root_validations: &[Box<dyn Validation>],
) -> TokenStream {
    let root_validations = wrap_validations(
        error_ident,
        root_error_ident,
        error_wrapper,
        combine_validations(
            execution,
            &format_error_ident_with_prefix(root_type_prefix, root_error_ident),
            &quote!(self),
            root_validations,
        ),
    );

    let validations = fields.flat_map(|field| {
        let field_error_ident = field.error_ident();
        let validations = field.validations(execution, field_prefix);

        wrap_validations(error_ident, field_error_ident, error_wrapper, validations)
    });

    combine_wrapped_validations(validations.chain(root_validations).collect::<Vec<_>>())
}
