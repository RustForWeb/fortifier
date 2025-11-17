use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields};

pub fn validate_tokens(input: DeriveInput) -> TokenStream {
    match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(_fields_named) => {
                // TODO
            }
            Fields::Unnamed(_fields_unnamed) => todo!("fields unamed"),
            Fields::Unit => todo!("fields unit"),
        },
        Data::Enum(_data_enum) => todo!("data enum"),
        Data::Union(_data_union) => todo!("data union"),
    }

    let ident = input.ident;
    let error_ident = format_ident!("{ident}ValidationError");

    quote! {
        #[derive(Debug)]
        struct #error_ident {}

        impl ::std::fmt::Display for #error_ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "")
            }
        }

        impl ::std::error::Error for #error_ident {}

        impl Validate for #ident {
            type Error = #error_ident;

            fn validate_sync(&self) -> Result<(), Self::Error> {
                Ok(())
            }

            fn validate_async(&self) -> ::std::pin::Pin<Box<impl Future<Output = Result<(), Self::Error>>>> {
                Box::pin(async {
                    Ok(())
                })
            }
        }
    }
}
