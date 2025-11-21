use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{DataEnum, DeriveInput, Result};

pub struct ValidateEnum {}

impl ValidateEnum {
    pub fn parse(_input: &DeriveInput, _data: &DataEnum) -> Result<Self> {
        todo!("enum")
    }
}

impl ToTokens for ValidateEnum {
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        // TODO
    }
}
