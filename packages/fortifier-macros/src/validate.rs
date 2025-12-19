mod data;
mod r#enum;
mod field;
mod fields;
mod r#struct;
mod r#type;
mod r#union;

use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote};
use syn::{DeriveInput, Generics, Ident, Result, Type, TypeTuple, punctuated::Punctuated};

use crate::{validate::data::ValidateData, validation::Execution};

pub struct Validate<'a> {
    ident: &'a Ident,
    generics: &'a Generics,
    context_type: Option<Type>,
    data: ValidateData<'a>,
}

impl<'a> Validate<'a> {
    pub fn parse(input: &'a DeriveInput) -> Result<Self> {
        let mut result = Validate {
            ident: &input.ident,
            generics: &input.generics,
            context_type: None,
            data: ValidateData::parse(input)?,
        };

        for attribute in &input.attrs {
            if !attribute.path().is_ident("validate") {
                continue;
            }

            attribute.parse_nested_meta(|meta| {
                if meta.path.is_ident("context") {
                    result.context_type = Some(meta.value()?.parse()?);

                    Ok(())
                } else {
                    Err(meta.error("unknown parameter"))
                }
            })?;
        }

        Ok(result)
    }
}

impl<'a> ToTokens for Validate<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

        let context_type = match &self.context_type {
            Some(context_type) => context_type,
            None => &Type::Tuple(TypeTuple {
                paren_token: Default::default(),
                elems: Punctuated::new(),
            }),
        };

        let (error_type, error_definition) = self
            .data
            .error_type()
            .unwrap_or_else(|| (quote!(::std::convert::Infallible), TokenStream::new()));

        let sync_validations = self.data.validations(Execution::Sync);
        let async_validations = self.data.validations(Execution::Async);

        let no_context_impl = self.context_type.is_none().then(|| {
            quote! {
                #[automatically_derived]
                impl #impl_generics ::fortifier::Validate for #ident #type_generics #where_clause {}
            }
        });

        tokens.append_all(quote! {
            #error_definition

            #[automatically_derived]
            impl #impl_generics ::fortifier::ValidateWithContext for #ident #type_generics #where_clause {
                type Context = #context_type;
                type Error = #error_type;

                fn validate_sync_with_context(&self, context: &Self::Context) -> Result<(), ::fortifier::ValidationErrors<Self::Error>> {
                    #sync_validations
                }

                fn validate_async_with_context(&self, context: &Self::Context) -> ::std::pin::Pin<Box<impl Future<Output = Result<(), ::fortifier::ValidationErrors<Self::Error>>>>> {
                    Box::pin(async move {
                        #async_validations
                    })
                }
            }

            #no_context_impl
        })
    }
}
