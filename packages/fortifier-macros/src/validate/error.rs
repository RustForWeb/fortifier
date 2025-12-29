use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Ident, Visibility};

use crate::{attributes::enum_attributes, validation::Validation};

pub fn format_error_ident(ident: &Ident) -> Ident {
    format_ident!("{}ValidationError", ident)
}

pub fn format_error_ident_with_prefix(prefix: &Ident, ident: &Ident) -> Ident {
    format_ident!("{}{}ValidationError", prefix, ident)
}

pub struct ErrorType {
    pub variant_ident: Ident,
    pub r#type: TokenStream,
    pub definition: Option<TokenStream>,
}

pub fn error_type(
    visibility: &Visibility,
    prefix: &Ident,
    error_ident: &Ident,
    validations: &[Box<dyn Validation>],
) -> Option<ErrorType> {
    if validations.len() > 1 {
        let attributes = enum_attributes();
        let ident = format_error_ident_with_prefix(prefix, error_ident);
        let variant_ident = validations.iter().map(|validation| validation.ident());
        let variant_type = validations.iter().map(|validation| validation.error_type());

        Some(ErrorType {
            variant_ident: error_ident.clone(),
            r#type: ident.to_token_stream(),
            definition: Some(quote! {
                #[derive(Debug, PartialEq)]
                #attributes
                #visibility enum #ident {
                    #( #variant_ident(#variant_type) ),*
                }
            }),
        })
    } else {
        validations.first().map(|validation| ErrorType {
            variant_ident: error_ident.clone(),
            r#type: validation.error_type(),
            definition: None,
        })
    }
}

pub fn combined_error_type<'a>(
    visibility: &Visibility,
    error_variant_ident: &Ident,
    error_ident: &Ident,
    error_field_idents: impl Iterator<Item = &'a Ident>,
    error_field_types: impl Iterator<Item = &'a TokenStream>,
    error_definitions: impl Iterator<Item = &'a TokenStream>,
    root_error_type: Option<&ErrorType>,
) -> ErrorType {
    let attributes = enum_attributes();
    let root_error_field = root_error_type.as_ref().map(
        |ErrorType {
             variant_ident,
             r#type,
             ..
         }| {
            quote! {
                #variant_ident(#r#type),
            }
        },
    );
    let root_error_definition =
        root_error_type.and_then(|ErrorType { definition, .. }| definition.as_ref());

    ErrorType {
        variant_ident: error_variant_ident.clone(),
        r#type: error_ident.to_token_stream(),
        definition: Some(quote! {
            #[allow(dead_code)]
            #[derive(Debug, PartialEq)]
            #attributes
            #visibility enum #error_ident {
                #root_error_field
                #( #error_field_idents(#error_field_types) ),*
            }

            #[automatically_derived]
            impl ::std::fmt::Display for #error_ident {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, "{self:#?}")
                }
            }

            #[automatically_derived]
            impl ::std::error::Error for #error_ident {}

            #root_error_definition

            #( #error_definitions )*
        }),
    }
}
