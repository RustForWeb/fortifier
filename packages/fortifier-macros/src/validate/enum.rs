use proc_macro2::TokenStream;
use syn::{DataEnum, Ident, Result};

pub fn validate_enum(_ident: Ident, _error_ident: Ident, _data: DataEnum) -> Result<TokenStream> {
    todo!("enum")
}
