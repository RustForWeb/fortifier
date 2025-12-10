use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote};
use syn::{DataStruct, DeriveInput, Generics, Ident, Result};

use crate::{
    validate::{field::ValidateFieldPrefix, fields::ValidateFields},
    validation::Execution,
};

pub struct ValidateStruct<'a> {
    ident: &'a Ident,
    generics: &'a Generics,
    fields: ValidateFields<'a>,
}

impl<'a> ValidateStruct<'a> {
    pub fn parse(input: &'a DeriveInput, data: &'a DataStruct) -> Result<Self> {
        Ok(ValidateStruct {
            ident: &input.ident,
            generics: &input.generics,
            fields: ValidateFields::parse(&input.vis, input.ident.clone(), &data.fields)?,
        })
    }
}

impl<'a> ToTokens for ValidateStruct<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

        let error_wrapper = |tokens| tokens;

        let (error_ident, error_type) = self.fields.error_type();
        let sync_validations = self.fields.validations(
            Execution::Sync,
            ValidateFieldPrefix::SelfKeyword,
            &error_wrapper,
        );
        let async_validations = self.fields.validations(
            Execution::Async,
            ValidateFieldPrefix::SelfKeyword,
            &error_wrapper,
        );

        tokens.append_all(quote! {
            #error_type

            #[automatically_derived]
            impl #impl_generics ::fortifier::Validate for #ident #type_generics #where_clause {
                type Error = #error_ident;

                fn validate_sync(&self) -> Result<(), ::fortifier::ValidationErrors<Self::Error>> {
                    #sync_validations
                }

                fn validate_async(&self) -> ::std::pin::Pin<Box<impl Future<Output = Result<(), ::fortifier::ValidationErrors<Self::Error>>>>> {
                    Box::pin(async {
                        #async_validations
                    })
                }
            }
        })
    }
}
