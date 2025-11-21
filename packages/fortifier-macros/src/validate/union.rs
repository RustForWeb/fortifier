use proc_macro2::TokenStream;
use syn::{DataUnion, Ident, Result};

pub fn validate_union(_ident: Ident, _error_ident: Ident, _data: DataUnion) -> Result<TokenStream> {
    todo!("union")
}
