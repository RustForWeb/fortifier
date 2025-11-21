use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{DataStruct, Field, Fields, Ident, Result};

use crate::validations::{email_tokens, length_tokens, parse_email, parse_length};

pub fn validate_struct_tokens(
    ident: Ident,
    error_ident: Ident,
    data: DataStruct,
) -> Result<TokenStream> {
    match data.fields {
        Fields::Named(fields_named) => {
            validate_named_struct_tokens(ident, error_ident, fields_named.named.into_iter())
        }
        Fields::Unnamed(_fields_unnamed) => todo!("fields unamed"),
        Fields::Unit => todo!("fields unit"),
    }
}

fn validate_named_struct_tokens(
    ident: Ident,
    error_ident: Ident,
    fields: impl Iterator<Item = Field>,
) -> Result<TokenStream> {
    let mut field_names = vec![];
    let mut field_types = vec![];
    let mut sync_validations = vec![];
    // let async_validations = vec![];

    for field in fields {
        let Some(field_ident) = field.ident else {
            continue;
        };

        let field_error_ident =
            format_ident!("{}", &field_ident.to_string().to_case(Case::UpperCamel));

        for attr in field.attrs {
            if attr.path().is_ident("validate") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("email") {
                        let email = parse_email(&meta)?;

                        field_names.push(field_error_ident.clone());
                        field_types.push(quote!(::fortifier::EmailError));

                        sync_validations.push(email_tokens(
                            email,
                            &error_ident,
                            &field_ident,
                            &field_error_ident,
                        ));

                        Ok(())
                    } else if meta.path.is_ident("length") {
                        let length = parse_length(&meta)?;

                        field_names.push(field_error_ident.clone());
                        field_types.push(quote!(::fortifier::LengthError<usize>));

                        sync_validations.push(length_tokens(
                            length,
                            &error_ident,
                            &field_ident,
                            &field_error_ident,
                        ));

                        Ok(())
                    } else {
                        Err(meta.error("unknown validate parameter"))
                    }
                })?;
            }
        }
    }

    Ok(quote! {
        use fortifier::*;

        #[derive(Debug)]
        enum #error_ident {
            #( #field_names(#field_types) ),*
        }

        impl ::std::fmt::Display for #error_ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{self:#?}")
            }
        }

        impl ::std::error::Error for #error_ident {}

        impl Validate for #ident {
            type Error = #error_ident;

            fn validate_sync(&self) -> Result<(), ValidationErrors<Self::Error>> {
                use ::fortifier::*;

                let mut errors = vec![];

                #(#sync_validations)*

                if !errors.is_empty() {
                    Err(errors.into())
                } else {
                    Ok(())
                }
            }

            fn validate_async(&self) -> ::std::pin::Pin<Box<impl Future<Output = Result<(), ValidationErrors<Self::Error>>>>> {
                use ::fortifier::*;

                Box::pin(async {

                    let mut errors = vec![];

                    // #(#async_validations)*

                    if !errors.is_empty() {
                        Err(errors.into())
                    } else {
                        Ok(())
                    }
                })
            }
        }
    })
}
