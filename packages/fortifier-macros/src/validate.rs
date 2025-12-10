mod attributes;
mod r#enum;
mod field;
mod fields;
mod r#struct;
mod r#union;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Data, DeriveInput, Result};

use crate::validate::{r#enum::ValidateEnum, r#struct::ValidateStruct, union::ValidateUnion};

pub enum Validate {
    Struct(ValidateStruct),
    Enum(ValidateEnum),
    Union(ValidateUnion),
}

impl Validate {
    pub fn parse(input: DeriveInput) -> Result<Self> {
        Ok(match &input.data {
            Data::Struct(data) => Self::Struct(ValidateStruct::parse(&input, data)?),
            Data::Enum(data) => Self::Enum(ValidateEnum::parse(&input, data)?),
            Data::Union(data) => Self::Union(ValidateUnion::parse(&input, data)?),
        })
    }
}

impl ToTokens for Validate {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Validate::Struct(r#struct) => r#struct.to_tokens(tokens),
            Validate::Enum(r#enum) => r#enum.to_tokens(tokens),
            Validate::Union(r#union) => r#union.to_tokens(tokens),
        }
    }
}
