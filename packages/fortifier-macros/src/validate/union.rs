use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{DataUnion, DeriveInput, Result};

pub struct ValidateUnion {}

impl ValidateUnion {
    pub fn parse(_input: &DeriveInput, _data: &DataUnion) -> Result<Self> {
        todo!("union")
    }
}

impl ToTokens for ValidateUnion {
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        // TODO
    }
}
