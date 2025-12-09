use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote};
use syn::{DataStruct, DeriveInput, Generics, Ident, Result};

use crate::{
    validate::{field::ValidateFieldPrefix, fields::ValidateFields},
    validation::Execution,
};

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
