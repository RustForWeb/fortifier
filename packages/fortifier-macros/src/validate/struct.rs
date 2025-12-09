use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote};
use syn::{DataStruct, DeriveInput, Generics, Ident, Result};

use crate::validate::{field::ValidateFieldPrefix, fields::ValidateFields};

pub struct ValidateStruct {
    ident: Ident,
    generics: Generics,
    fields: ValidateFields,
}

impl ValidateStruct {
    pub fn parse(input: &DeriveInput, data: &DataStruct) -> Result<Self> {
        Ok(ValidateStruct {
            ident: input.ident.clone(),
            generics: input.generics.clone(),
            fields: ValidateFields::parse(&input.vis, &input.ident, &data.fields)?,
        })
    }
}

impl ToTokens for ValidateStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

        let error_wrapper = |tokens| tokens;

        let uses = self.fields.uses().into_iter().map(|r#use| {
            let tokens = TokenStream::from_str(&r#use).expect("Tokens should be valid.");
            quote!(use #tokens;)
        });
        let (error_ident, error_type) = self.fields.error_type();
        let sync_validations = self
            .fields
            .sync_validations(ValidateFieldPrefix::SelfKeyword, &error_wrapper);
        let async_validations = self
            .fields
            .async_validations(ValidateFieldPrefix::SelfKeyword, &error_wrapper);

        tokens.append_all(quote! {
            #( #uses )*

            // TODO: Replace with granular uses.
            use fortifier::*;

            #error_type

            #[automatically_derived]
            impl #impl_generics Validate for #ident #type_generics #where_clause {
                type Error = #error_ident;

                fn validate_sync(&self) -> Result<(), ValidationErrors<Self::Error>> {
                    #sync_validations
                }

                fn validate_async(&self) -> ::std::pin::Pin<Box<impl Future<Output = Result<(), ValidationErrors<Self::Error>>>>> {
                    Box::pin(async {
                        #async_validations
                    })
                }
            }
        })
    }
}
