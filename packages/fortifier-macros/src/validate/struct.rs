use proc_macro2::TokenStream;
use syn::{DataStruct, DeriveInput, Result};

use crate::{
    validate::{field::ValidateFieldPrefix, fields::ValidateFields},
    validation::Execution,
};

pub struct ValidateStruct<'a> {
    fields: ValidateFields<'a>,
}

impl<'a> ValidateStruct<'a> {
    pub fn parse(input: &'a DeriveInput, data: &'a DataStruct) -> Result<Self> {
        Ok(ValidateStruct {
            fields: ValidateFields::parse(&input.vis, input.ident.clone(), &data.fields)?,
        })
    }

    pub fn error_type(&self) -> (TokenStream, TokenStream) {
        self.fields.error_type()
    }

    pub fn validations(&self, execution: Execution) -> TokenStream {
        let error_wrapper = |tokens| tokens;

        self.fields
            .validations(execution, ValidateFieldPrefix::SelfKeyword, &error_wrapper)
    }
}
