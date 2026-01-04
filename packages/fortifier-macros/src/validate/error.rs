use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Generics, Ident, Visibility};

use crate::{
    generics::{Generic, filter_generics},
    integrations::enum_attributes,
    validation::Validation,
};

pub fn format_error_ident(ident: &Ident) -> Ident {
    format_ident!("{}ValidationError", ident)
}

pub fn format_error_ident_with_prefix(prefix: &Ident, ident: &Ident) -> Ident {
    format_ident!("{}{}ValidationError", prefix, ident)
}

pub struct ErrorType {
    pub variant_ident: Ident,
    pub r#type: TokenStream,
    pub generics: Vec<Generic>,
    pub where_predicates: Vec<TokenStream>,
    pub definition: Option<TokenStream>,
}

pub fn error_type(
    visibility: &Visibility,
    prefix: &Ident,
    error_ident: &Ident,
    validations: &[Box<dyn Validation>],
) -> Option<ErrorType> {
    if validations.len() > 1 {
        let attributes = enum_attributes();
        let ident = format_error_ident_with_prefix(prefix, error_ident);
        let variant_ident = validations.iter().map(|validation| validation.ident());
        let variant_type = validations.iter().map(|validation| validation.error_type());
        let generics = validations
            .iter()
            .flat_map(|validation| validation.error_generics())
            .collect();
        let where_predicates = validations
            .iter()
            .flat_map(|validation| validation.error_where_predicates())
            .collect();

        Some(ErrorType {
            variant_ident: error_ident.clone(),
            r#type: ident.to_token_stream(),
            generics,
            where_predicates,
            definition: Some(quote! {
                #[derive(Debug, PartialEq)]
                #attributes
                #visibility enum #ident {
                    #( #variant_ident(#variant_type) ),*
                }
            }),
        })
    } else {
        validations.first().map(|validation| ErrorType {
            variant_ident: error_ident.clone(),
            r#type: validation.error_type(),
            generics: validation.error_generics(),
            where_predicates: validation.error_where_predicates(),
            definition: None,
        })
    }
}

pub fn combined_error_type(
    visibility: &Visibility,
    generics: &Generics,
    variant_ident: &Ident,
    ident: &Ident,
    error_types: Vec<ErrorType>,
    root_error_type: Option<&ErrorType>,
) -> Option<ErrorType> {
    if error_types.is_empty() && root_error_type.is_none() {
        return None;
    }

    let attributes = enum_attributes();

    let variant_idents = root_error_type
        .into_iter()
        .map(|error_type| &error_type.variant_ident)
        .chain(
            error_types
                .iter()
                .map(|error_type| &error_type.variant_ident),
        );

    let variant_types = root_error_type
        .into_iter()
        .map(|error_type| &error_type.r#type)
        .chain(error_types.iter().map(|error_type| &error_type.r#type));

    let definitions = root_error_type
        .into_iter()
        .flat_map(|error_type| &error_type.definition)
        .chain(
            error_types
                .iter()
                .flat_map(|error_type| &error_type.definition),
        );

    let generic_arguments_or_params = root_error_type
        .into_iter()
        .flat_map(|error_type| &error_type.generics)
        .chain(
            error_types
                .iter()
                .flat_map(|error_type| &error_type.generics),
        )
        .cloned()
        .collect::<Vec<_>>();

    let where_predicates = root_error_type
        .into_iter()
        .flat_map(|error_type| &error_type.where_predicates)
        .chain(
            error_types
                .iter()
                .flat_map(|error_type| &error_type.where_predicates),
        )
        .cloned()
        .collect::<Vec<_>>();

    let generics = filter_generics(generics, &generic_arguments_or_params);
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let where_clause = if let Some(where_clause) = where_clause {
        if where_clause.predicates.trailing_punct() {
            quote! {
                #where_clause #( #where_predicates ),*
            }
        } else {
            quote! {
                #where_clause, #( #where_predicates ),*
            }
        }
    } else {
        quote! {
            where #( #where_predicates ),*
        }
    };

    Some(ErrorType {
        variant_ident: variant_ident.clone(),
        r#type: quote!(#ident #type_generics),
        generics: generic_arguments_or_params,
        where_predicates,
        definition: Some(quote! {
            #[allow(dead_code)]
            #[derive(Debug, PartialEq)]
            #attributes
            #visibility enum #ident #type_generics #where_clause {
                #( #variant_idents(#variant_types) ),*
            }

            #[automatically_derived]
            impl #impl_generics ::std::fmt::Display for #ident #type_generics #where_clause {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, "{self:#?}")
                }
            }

            #[automatically_derived]
            impl #impl_generics ::std::error::Error for #ident #type_generics #where_clause {}

            #( #definitions )*
        }),
    })
}
