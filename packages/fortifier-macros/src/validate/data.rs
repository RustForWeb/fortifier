use proc_macro2::TokenStream;
use syn::{Data, DeriveInput, Ident, Result};

use crate::{
    validate::{
        r#enum::ValidateEnum, error::ErrorType, r#struct::ValidateStruct, union::ValidateUnion,
    },
    validation::{Execution, Validation},
};

pub enum ValidateData<'a> {
    Struct(ValidateStruct<'a>),
    Enum(ValidateEnum<'a>),
    Union(ValidateUnion),
}

impl<'a> ValidateData<'a> {
    pub fn parse(input: &'a DeriveInput) -> Result<Self> {
        Ok(match &input.data {
            Data::Struct(data) => Self::Struct(ValidateStruct::parse(input, data)?),
            Data::Enum(data) => Self::Enum(ValidateEnum::parse(input, data)?),
            Data::Union(data) => Self::Union(ValidateUnion::parse(input, data)?),
        })
    }

    pub fn error_type(&self, root_error_type: Option<&ErrorType>) -> Option<ErrorType> {
        match self {
            ValidateData::Struct(r#struct) => r#struct.error_type(root_error_type),
            ValidateData::Enum(r#enum) => r#enum.error_type(root_error_type),
            ValidateData::Union(r#union) => r#union.error_type(),
        }
    }

    pub fn validations(
        &self,
        execution: Execution,
        root_type_prefix: &Ident,
        root_error_ident: &Ident,
        root_validations: &[Box<dyn Validation>],
    ) -> TokenStream {
        match self {
            ValidateData::Struct(r#struct) => r#struct.validations(
                execution,
                root_type_prefix,
                root_error_ident,
                root_validations,
            ),
            ValidateData::Enum(r#enum) => r#enum.validations(
                execution,
                root_type_prefix,
                root_error_ident,
                root_validations,
            ),
            ValidateData::Union(r#union) => r#union.validations(
                execution,
                root_type_prefix,
                root_error_ident,
                root_validations,
            ),
        }
    }
}
