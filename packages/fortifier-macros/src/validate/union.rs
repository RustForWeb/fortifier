use proc_macro2::TokenStream;
use syn::{DataUnion, DeriveInput, Result};

use crate::validation::Execution;

pub struct ValidateUnion {}

impl ValidateUnion {
    pub fn parse(input: &DeriveInput, _data: &DataUnion) -> Result<Self> {
        Err(syn::Error::new_spanned(input, "union is not supported"))
    }

    pub fn error_type(&self) -> Option<(TokenStream, TokenStream)> {
        todo!()
    }

    pub fn validations(&self, _execution: Execution) -> TokenStream {
        todo!()
    }
}
