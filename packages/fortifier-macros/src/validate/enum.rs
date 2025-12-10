use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, format_ident, quote};
use syn::{DataEnum, DeriveInput, Generics, Ident, Result, Variant, Visibility};

use crate::{
    validate::{
        attributes::enum_attributes,
        field::{LiteralOrIdent, ValidateFieldPrefix},
        fields::ValidateFields,
    },
    validation::Execution,
};

pub struct ValidateEnum<'a> {
    visibility: &'a Visibility,
    ident: &'a Ident,
    error_ident: Ident,
    generics: &'a Generics,
    variants: Vec<ValidateEnumVariant<'a>>,
}

impl<'a> ValidateEnum<'a> {
    pub fn parse(input: &'a DeriveInput, data: &'a DataEnum) -> Result<Self> {
        let mut result = ValidateEnum {
            visibility: &input.vis,
            ident: &input.ident,
            error_ident: format_ident!("{}ValidationError", input.ident),
            generics: &input.generics,
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

    fn error_type(&self) -> (&Ident, TokenStream) {
        let visibility = &self.visibility;
        let error_ident = &self.error_ident;

        let attributes = enum_attributes();
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
            error_ident,
            quote! {
                #[allow(dead_code)]
                #[derive(Debug)]
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
            },
        )
    }
}

impl<'a> ToTokens for ValidateEnum<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let (impl_generics, type_generics, where_clause) = &self.generics.split_for_impl();

        let (error_ident, error_type) = self.error_type();
        let variant_error_types = self.variants.iter().map(|variant| variant.error_type().1);
        let sync_variant_match_arms = self
            .variants
            .iter()
            .map(|variant| variant.match_arm(Execution::Sync));
        let async_variant_match_arms = self
            .variants
            .iter()
            .map(|variant| variant.match_arm(Execution::Async));

        tokens.append_all(quote! {
            #error_type

            #( #variant_error_types )*

            #[automatically_derived]
            impl #impl_generics ::fortifier::Validate for #ident #type_generics #where_clause {
                type Error = #error_ident;

                fn validate_sync(&self) -> Result<(), ::fortifier::ValidationErrors<Self::Error>> {
                    match &self {
                        #( #sync_variant_match_arms ),*
                    }
                }

                fn validate_async(&self) -> ::std::pin::Pin<Box<impl Future<Output = Result<(), ::fortifier::ValidationErrors<Self::Error>>>>> {
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

    fn error_type(&self) -> (TokenStream, TokenStream) {
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
