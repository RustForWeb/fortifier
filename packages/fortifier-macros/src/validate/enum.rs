use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{DataEnum, DeriveInput, Ident, Result, Variant, Visibility};

use crate::{
    attributes::enum_attributes,
    validate::{
        field::{LiteralOrIdent, ValidateFieldPrefix, format_error_ident},
        fields::ValidateFields,
    },
    validation::Execution,
};

pub struct ValidateEnum<'a> {
    visibility: &'a Visibility,
    ident: &'a Ident,
    error_ident: Ident,
    variants: Vec<ValidateEnumVariant<'a>>,
}

impl<'a> ValidateEnum<'a> {
    pub fn parse(input: &'a DeriveInput, data: &'a DataEnum) -> Result<Self> {
        let mut result = ValidateEnum {
            visibility: &input.vis,
            ident: &input.ident,
            error_ident: format_error_ident(&input.ident),
            variants: Vec::with_capacity(data.variants.len()),
        };

        for variant in &data.variants {
            result.variants.push(ValidateEnumVariant::parse(
                &input.vis,
                result.ident,
                result.error_ident.clone(),
                variant,
            )?);
        }

        Ok(result)
    }

    pub fn error_type(&self) -> Option<(TokenStream, TokenStream)> {
        if self.variants.is_empty() {
            return None;
        }

        let visibility = &self.visibility;
        let error_ident = &self.error_ident;

        let attributes = enum_attributes();
        let error_variant_idents = self
            .variants
            .iter()
            .flat_map(|variant| variant.error_type().map(|_| &variant.ident))
            .collect::<Vec<_>>();
        let (error_variant_types, variant_error_types): (Vec<_>, Vec<_>) = self
            .variants
            .iter()
            .flat_map(|variant| variant.error_type())
            .unzip();

        if error_variant_types.is_empty() {
            return None;
        }

        Some((
            error_ident.to_token_stream(),
            quote! {
                #[allow(dead_code)]
                #[derive(Debug, PartialEq)]
                #attributes
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

                #( #variant_error_types )*
            },
        ))
    }

    pub fn validations(&self, execution: Execution) -> TokenStream {
        let variant_match_arms = self
            .variants
            .iter()
            .map(|variant| variant.match_arm(execution));

        quote! {
            match &self {
                #( #variant_match_arms ),*
            }
        }
    }
}

pub struct ValidateEnumVariant<'a> {
    enum_ident: &'a Ident,
    enum_error_ident: Ident,
    ident: &'a Ident,
    fields: ValidateFields<'a>,
}

impl<'a> ValidateEnumVariant<'a> {
    pub fn parse(
        visibility: &'a Visibility,
        enum_ident: &'a Ident,
        enum_error_ident: Ident,
        variant: &'a Variant,
    ) -> Result<Self> {
        let result = ValidateEnumVariant {
            enum_ident,
            enum_error_ident,
            ident: &variant.ident,
            fields: ValidateFields::parse(
                visibility,
                format_ident!("{}{}", enum_ident, variant.ident),
                &variant.fields,
            )?,
        };

        Ok(result)
    }

    fn error_type(&self) -> Option<(TokenStream, TokenStream)> {
        self.fields.error_type()
    }

    fn match_arm(&self, exeuction: Execution) -> TokenStream {
        let enum_ident = &self.enum_ident;
        let enum_error_ident = &self.enum_error_ident;
        let ident = &self.ident;

        let error_wrapper = |tokens| quote!(#enum_error_ident::#ident(#tokens));

        match &self.fields {
            ValidateFields::Named(fields) => {
                let field_idents = fields.idents();
                let validations =
                    fields.validations(exeuction, ValidateFieldPrefix::None, &error_wrapper);

                // TODO: Only destructure fields required for validation.
                quote! {
                    #[allow(unused_variables)]
                    #enum_ident::#ident {
                        #( #field_idents ),*
                    } => {
                        #validations
                    }
                }
            }
            ValidateFields::Unnamed(fields) => {
                let field_idents = fields.idents().map(|ident| match ident {
                    LiteralOrIdent::Literal(literal) => format_ident!("f{literal}"),
                    LiteralOrIdent::Ident(ident) => ident.clone(),
                });
                let validations =
                    fields.validations(exeuction, ValidateFieldPrefix::F, &error_wrapper);

                quote! {
                    #enum_ident::#ident(
                        #( #field_idents ),*
                    ) => {
                        #validations
                    }
                }
            }
            ValidateFields::Unit(fields) => {
                let validations = fields.validations();

                quote! {
                    #enum_ident::#ident => {
                        #validations
                    }
                }
            }
        }
    }
}
