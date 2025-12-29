use proc_macro2::TokenStream;
use syn::{DataUnion, DeriveInput, Ident, Result};

use crate::{
    validate::error::ErrorType,
    validation::{Execution, Validation},
};

pub struct ValidateUnion {}

impl ValidateUnion {
    pub fn parse(input: &DeriveInput, _data: &DataUnion) -> Result<Self> {
        Err(syn::Error::new_spanned(input, "union is not supported"))
    }

    pub fn error_type(&self) -> Option<ErrorType> {
        todo!()
    }

    pub fn validations(
        &self,
        _execution: Execution,
        _root_type_prefix: &Ident,
        _root_error_ident: &Ident,
        _root_validations: &[Box<dyn Validation>],
    ) -> TokenStream {
        todo!()
    }
}
