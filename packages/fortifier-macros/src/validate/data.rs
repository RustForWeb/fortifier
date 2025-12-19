use proc_macro2::TokenStream;
use syn::{Data, DeriveInput, Result};

use crate::{
    validate::{r#enum::ValidateEnum, r#struct::ValidateStruct, union::ValidateUnion},
    validation::Execution,
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

    pub fn error_type(&self) -> Option<(TokenStream, TokenStream)> {
        match self {
            ValidateData::Struct(r#struct) => r#struct.error_type(),
            ValidateData::Enum(r#enum) => r#enum.error_type(),
            ValidateData::Union(r#union) => r#union.error_type(),
        }
    }

    pub fn validations(&self, execution: Execution) -> TokenStream {
        match self {
            ValidateData::Struct(r#struct) => r#struct.validations(execution),
            ValidateData::Enum(r#enum) => r#enum.validations(execution),
            ValidateData::Union(r#union) => r#union.validations(execution),
        }
    }
}
