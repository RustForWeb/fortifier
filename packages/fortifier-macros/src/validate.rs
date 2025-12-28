mod data;
mod r#enum;
mod error;
mod field;
mod fields;
mod r#struct;
mod r#type;
mod r#union;

use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, format_ident, quote};
use syn::{
    DeriveInput, Generics, Ident, Result, Type, TypeNever, TypeTuple, Visibility,
    punctuated::Punctuated,
};

use crate::{
    validate::{
        data::ValidateData,
        error::{ErrorType, error_type, format_error_ident},
    },
    validation::{Execution, Validation},
    validations::Custom,
};

pub struct Validate<'a> {
    visibility: &'a Visibility,
    ident: &'a Ident,
    generics: &'a Generics,
    root_error_ident: Ident,
    context_type: Option<Type>,
    data: ValidateData<'a>,
    validations: Vec<Box<dyn Validation>>,
}

impl<'a> Validate<'a> {
    pub fn parse(input: &'a DeriveInput) -> Result<Self> {
        let mut result = Validate {
            visibility: &input.vis,
            ident: &input.ident,
            generics: &input.generics,
            // TODO: Make `Root` ident configurable to prevent collisions.
            root_error_ident: format_ident!("Root"),
            context_type: None,
            data: ValidateData::parse(input)?,
            validations: vec![],
        };

        for attribute in &input.attrs {
            if !attribute.path().is_ident("validate") {
                continue;
            }

            attribute.parse_nested_meta(|meta| {
                if meta.path.is_ident("context") {
                    result.context_type = Some(meta.value()?.parse()?);

                    Ok(())
                } else if meta.path.is_ident("custom") {
                    result.validations.push(Box::new(Custom::parse(
                        // Type is never used in the custom validation, so pass an arbitrary value.
                        &Type::Never(TypeNever {
                            bang_token: Default::default(),
                        }),
                        &meta,
                    )?));

                    Ok(())
                } else {
                    Err(meta.error("unknown parameter"))
                }
            })?;
        }

        Ok(result)
    }

    fn error_type(&self) -> Option<ErrorType> {
        let root_error_type = error_type(
            self.visibility,
            self.ident,
            &self.root_error_ident,
            &self.validations,
        );

        self.data.error_type(root_error_type.as_ref())
    }

    fn validations(&self, execution: Execution) -> TokenStream {
        self.data.validations(
            execution,
            &format_error_ident(self.ident),
            &self.root_error_ident,
            &self.validations,
        )
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

        let (error_type, error_definition) = if let Some(ErrorType {
            r#type, definition, ..
        }) = self.error_type()
        {
            (r#type, definition)
        } else {
            (quote!(::std::convert::Infallible), None)
        };

        let sync_validations = self.validations(Execution::Sync);
        let async_validations = self.validations(Execution::Async);

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
