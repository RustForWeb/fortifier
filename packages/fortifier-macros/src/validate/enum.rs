use std::{collections::HashSet, str::FromStr};

use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, format_ident, quote};
use syn::{DataEnum, DeriveInput, Generics, Ident, Result, Variant, Visibility};

use crate::validate::{
    field::{LiteralOrIdent, ValidateFieldPrefix},
    fields::ValidateFields,
};

pub struct ValidateEnum {
    visibility: Visibility,
    ident: Ident,
    error_ident: Ident,
    generics: Generics,
    variants: Vec<ValidateEnumVariant>,
}

impl ValidateEnum {
    pub fn parse(input: &DeriveInput, data: &DataEnum) -> Result<Self> {
        let mut result = ValidateEnum {
            visibility: input.vis.clone(),
            ident: input.ident.clone(),
            error_ident: format_ident!("{}ValidationError", input.ident),
            generics: input.generics.clone(),
            variants: Vec::with_capacity(data.variants.len()),
        };

        for variant in &data.variants {
            result.variants.push(ValidateEnumVariant::parse(
                &input.vis,
                &result.ident,
                &result.error_ident,
                variant,
            )?);
        }

        Ok(result)
    }

    fn uses(&self) -> HashSet<String> {
        self.variants
            .iter()
            .flat_map(|variant| variant.uses())
            .collect()
    }

    fn error_type(&self) -> (Ident, TokenStream) {
        let visibility = &self.visibility;
        let error_ident = &self.error_ident;

        let error_variant_idents = self
            .variants
            .iter()
            .map(|variant| &variant.ident)
            .collect::<Vec<_>>();
        let error_variant_types = self
            .variants
            .iter()
            .map(|variant| variant.error_type().0)
            .collect::<Vec<_>>();

        (
            error_ident.clone(),
            quote! {
                #[allow(dead_code)]
                #[derive(Debug)]
                #visibility enum #error_ident {
                    #( #error_variant_idents(#error_variant_types) ),*
                }

                #[automatically_derived]
                impl ::std::fmt::Display for #error_ident {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        write!(f, "{self:#?}")
                    }
                }

                #[automatically_derived]
                impl ::std::error::Error for #error_ident {}
            },
        )
    }
}

impl ToTokens for ValidateEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let (impl_generics, type_generics, where_clause) = &self.generics.split_for_impl();

        let uses = self.uses().into_iter().map(|r#use| {
            let tokens = TokenStream::from_str(&r#use).expect("Tokens should be valid.");
            quote!(use #tokens;)
        });
        let (error_ident, error_type) = self.error_type();
        let variant_error_types = self.variants.iter().map(|variant| variant.error_type().1);
        let sync_variant_match_arms = self
            .variants
            .iter()
            .map(|variant| variant.sync_match_arm_tokens());
        let async_variant_match_arms = self
            .variants
            .iter()
            .map(|variant| variant.async_match_arm_tokens());

        tokens.append_all(quote! {
            #( #uses )*

            // TODO: Replace with granular uses.
            use fortifier::*;

            #error_type

            #( #variant_error_types )*

            #[automatically_derived]
            impl #impl_generics Validate for #ident #type_generics #where_clause {
                type Error = #error_ident;

                fn validate_sync(&self) -> Result<(), ValidationErrors<Self::Error>> {
                    match &self {
                        #( #sync_variant_match_arms ),*
                    }
                }

                fn validate_async(&self) -> ::std::pin::Pin<Box<impl Future<Output = Result<(), ValidationErrors<Self::Error>>>>> {
                    Box::pin(async move {
                        match &self {
                            #( #async_variant_match_arms ),*
                        }
                    })
                }
            }
        })
    }
}

pub struct ValidateEnumVariant {
    enum_ident: Ident,
    enum_error_ident: Ident,
    ident: Ident,
    fields: ValidateFields,
}

impl ValidateEnumVariant {
    pub fn parse(
        visibility: &Visibility,
        enum_ident: &Ident,
        enum_error_ident: &Ident,
        variant: &Variant,
    ) -> Result<Self> {
        let result = ValidateEnumVariant {
            enum_ident: enum_ident.clone(),
            enum_error_ident: enum_error_ident.clone(),
            ident: variant.ident.clone(),
            fields: ValidateFields::parse(
                visibility,
                &format_ident!("{}{}", enum_ident, variant.ident),
                &variant.fields,
            )?,
        };

        Ok(result)
    }

    fn uses(&self) -> HashSet<String> {
        self.fields.uses()
    }

    fn error_type(&self) -> (Ident, TokenStream) {
        self.fields.error_type()
    }

    fn sync_match_arm_tokens(&self) -> TokenStream {
        let enum_ident = &self.enum_ident;
        let enum_error_ident = &self.enum_error_ident;
        let ident = &self.ident;

        let error_wrapper = |tokens| quote!(#enum_error_ident::#ident(#tokens));

        match &self.fields {
            ValidateFields::Named(fields) => {
                let field_idents = fields.idents();
                let sync_validations =
                    fields.sync_validations(ValidateFieldPrefix::None, &error_wrapper);

                // TODO: Only destructure fields required for validation.
                quote! {
                    #[allow(unused_variables)]
                    #enum_ident::#ident {
                        #( #field_idents ),*
                    } => {
                        #sync_validations
                    }
                }
            }
            ValidateFields::Unnamed(fields) => {
                let field_idents = fields.idents().map(|ident| match ident {
                    LiteralOrIdent::Literal(literal) => format_ident!("f{literal}"),
                    LiteralOrIdent::Ident(ident) => ident.clone(),
                });
                let sync_validations =
                    fields.sync_validations(ValidateFieldPrefix::F, &error_wrapper);

                quote! {
                    #enum_ident::#ident(
                        #( #field_idents ),*
                    ) => {
                        #sync_validations
                    }
                }
            }
            ValidateFields::Unit(fields) => {
                let sync_validations = fields.sync_validations();

                quote! {
                    #enum_ident::#ident => {
                        #sync_validations
                    }
                }
            }
        }
    }

    fn async_match_arm_tokens(&self) -> TokenStream {
        let enum_ident = &self.enum_ident;
        let enum_error_ident = &self.enum_error_ident;
        let ident = &self.ident;

        let error_wrapper = |tokens| quote!(#enum_error_ident::#ident(#tokens));

        match &self.fields {
            ValidateFields::Named(fields) => {
                let field_idents = fields.idents();
                let async_validations =
                    fields.async_validations(ValidateFieldPrefix::None, &error_wrapper);

                // TODO: Only destructure fields required for validation.
                quote! {
                    #[allow(unused_variables)]
                    #enum_ident::#ident {
                        #( #field_idents ),*
                    } => {
                        #async_validations
                    }
                }
            }
            ValidateFields::Unnamed(fields) => {
                let field_idents = fields.idents().map(|ident| match ident {
                    LiteralOrIdent::Literal(literal) => format_ident!("f{literal}"),
                    LiteralOrIdent::Ident(ident) => ident.clone(),
                });
                let async_validations =
                    fields.async_validations(ValidateFieldPrefix::F, &error_wrapper);

                quote! {
                    #enum_ident::#ident(
                        #( #field_idents ),*
                    ) => {
                        #async_validations
                    }
                }
            }
            ValidateFields::Unit(fields) => {
                let async_validations = fields.async_validations();

                quote! {
                    #enum_ident::#ident => {
                        #async_validations
                    }
                }
            }
        }
    }
}
