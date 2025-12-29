use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{DataEnum, DeriveInput, Generics, Ident, Result, Variant, Visibility};

use crate::{
    validate::{
        error::{ErrorType, combined_error_type, format_error_ident},
        field::{LiteralOrIdent, ValidateFieldPrefix},
        fields::ValidateFields,
    },
    validation::{Execution, Validation},
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
                &input.generics,
                result.ident,
                result.error_ident.clone(),
                variant,
            )?);
        }

        Ok(result)
    }

    pub fn error_type(&self, root_error_type: Option<&ErrorType>) -> Option<ErrorType> {
        if self.variants.is_empty() {
            return None;
        }

        let variant_error_types = self
            .variants
            .iter()
            .flat_map(|variant| variant.error_type(root_error_type))
            .collect::<Vec<_>>();

        if variant_error_types.is_empty() {
            return None;
        }

        let variant_idents = variant_error_types
            .iter()
            .map(|ErrorType { variant_ident, .. }| variant_ident);
        let variant_types = variant_error_types
            .iter()
            .map(|ErrorType { r#type, .. }| r#type);
        let variant_definitions = variant_error_types
            .iter()
            .flat_map(|ErrorType { definition, .. }| definition);

        Some(combined_error_type(
            self.visibility,
            self.ident,
            &self.error_ident,
            variant_idents,
            variant_types,
            variant_definitions,
            None,
        ))
    }

    pub fn validations(
        &self,
        execution: Execution,
        root_type_prefix: &Ident,
        root_error_ident: &Ident,
        root_validations: &[Box<dyn Validation>],
    ) -> TokenStream {
        let variant_match_arms = self.variants.iter().map(|variant| {
            variant.match_arm(
                execution,
                root_type_prefix,
                root_error_ident,
                root_validations,
            )
        });

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
        generics: &'a Generics,
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
                generics,
                format_ident!("{}{}", enum_ident, variant.ident),
                &variant.fields,
            )?,
        };

        Ok(result)
    }

    fn error_type(&self, root_error_type: Option<&ErrorType>) -> Option<ErrorType> {
        self.fields.error_type(Some(self.ident), root_error_type)
    }

    fn match_arm(
        &self,
        exeuction: Execution,
        root_type_prefix: &Ident,
        root_error_ident: &Ident,
        root_validations: &[Box<dyn Validation>],
    ) -> TokenStream {
        let enum_ident = &self.enum_ident;
        let enum_error_ident = &self.enum_error_ident;
        let ident = &self.ident;

        let error_wrapper = |tokens| quote!(#enum_error_ident::#ident(#tokens));

        match &self.fields {
            ValidateFields::Named(fields) => {
                let field_idents = fields.idents();
                let validations = fields.validations(
                    exeuction,
                    ValidateFieldPrefix::None,
                    &error_wrapper,
                    root_type_prefix,
                    root_error_ident,
                    root_validations,
                );

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
                let validations = fields.validations(
                    exeuction,
                    ValidateFieldPrefix::F,
                    &error_wrapper,
                    root_type_prefix,
                    root_error_ident,
                    root_validations,
                );

                quote! {
                    #enum_ident::#ident(
                        #( #field_idents ),*
                    ) => {
                        #validations
                    }
                }
            }
            ValidateFields::Unit(fields) => {
                let validations = fields.validations(
                    exeuction,
                    ValidateFieldPrefix::None,
                    &error_wrapper,
                    root_type_prefix,
                    root_error_ident,
                    root_validations,
                );

                quote! {
                    #enum_ident::#ident => {
                        #validations
                    }
                }
            }
        }
    }
}
