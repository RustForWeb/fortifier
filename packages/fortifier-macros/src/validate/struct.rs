use proc_macro2::TokenStream;
use syn::{DataStruct, DeriveInput, Ident, Result};

use crate::{
    validate::{error::ErrorType, field::ValidateFieldPrefix, fields::ValidateFields},
    validation::{Execution, Validation},
};

pub struct ValidateStruct<'a> {
    fields: ValidateFields<'a>,
}

impl<'a> ValidateStruct<'a> {
    pub fn parse(input: &'a DeriveInput, data: &'a DataStruct) -> Result<Self> {
        Ok(ValidateStruct {
            fields: ValidateFields::parse(
                &input.vis,
                &input.generics,
                input.ident.clone(),
                &data.fields,
            )?,
        })
    }

    pub fn error_type(&self, root_error_type: Option<&ErrorType>) -> Option<ErrorType> {
        self.fields.error_type(None, root_error_type)
    }

    pub fn validations(
        &self,
        execution: Execution,
        root_type_prefix: &Ident,
        root_error_ident: &Ident,
        root_validations: &[Box<dyn Validation>],
    ) -> TokenStream {
        let error_wrapper = |tokens| tokens;

        self.fields.validations(
            execution,
            ValidateFieldPrefix::SelfKeyword,
            &error_wrapper,
            root_type_prefix,
            root_error_ident,
            root_validations,
        )
    }
}
