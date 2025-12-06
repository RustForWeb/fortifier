use std::collections::HashMap;

use convert_case::{Case, Casing};
use proc_macro2::{Literal, TokenStream};
use quote::{ToTokens, TokenStreamExt, format_ident, quote};
use syn::{DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Ident, Result};

use crate::validate::field::ValidateField;

pub enum ValidateStruct {
    Named(ValidateNamedStruct),
    Unnamed(ValidateUnnamedStruct),
    Unit(ValidateUnitStruct),
}

impl ValidateStruct {
    pub fn parse(input: &DeriveInput, data: &DataStruct) -> Result<Self> {
        Ok(match &data.fields {
            Fields::Named(fields) => Self::Named(ValidateNamedStruct::parse(input, data, fields)?),
            Fields::Unnamed(fields) => {
                Self::Unnamed(ValidateUnnamedStruct::parse(input, data, fields)?)
            }
            Fields::Unit => Self::Unit(ValidateUnitStruct::parse(input)?),
        })
    }
}

impl ToTokens for ValidateStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ValidateStruct::Named(named) => named.to_tokens(tokens),
            ValidateStruct::Unnamed(unnamed) => unnamed.to_tokens(tokens),
            ValidateStruct::Unit(unit) => unit.to_tokens(tokens),
        }
    }
}

pub struct ValidateNamedStruct {
    ident: Ident,
    error_ident: Ident,
    fields: HashMap<Ident, ValidateField>,
}

impl ValidateNamedStruct {
    fn parse(input: &DeriveInput, _data: &DataStruct, fields: &FieldsNamed) -> Result<Self> {
        let mut result = Self {
            ident: input.ident.clone(),
            error_ident: format_ident!("{}ValidationError", input.ident),
            fields: HashMap::default(),
        };

        for field in &fields.named {
            let Some(field_ident) = &field.ident else {
                continue;
            };

            let expr = quote!(self.#field_ident);

            result.fields.insert(
                field_ident.clone(),
                ValidateField::parse(&input.ident, field_ident.clone(), expr, field)?,
            );
        }

        Ok(result)
    }
}

impl ToTokens for ValidateNamedStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let error_ident = &self.error_ident;
        let mut error_field_idents = vec![];
        let mut error_field_types = vec![];
        let mut error_field_enums = vec![];
        let mut sync_validations = vec![];
        let mut async_validations = vec![];

        for (field_ident, field) in &self.fields {
            let field_error_ident =
                format_ident!("{}", &field_ident.to_string().to_case(Case::UpperCamel));

            let (error_type, error_enum) = field.error_type(ident, &field_error_ident);

            error_field_idents.push(field_error_ident.clone());
            error_field_types.push(error_type);
            if let Some(error_enum) = error_enum {
                error_field_enums.push(error_enum);
            }

            for validation in field.sync_validations() {
                sync_validations.push(quote! {
                    if let Err(err) = #validation {
                        errors.push(#error_ident::#field_error_ident(err));
                    }
                });
            }

            for validation in field.async_validations() {
                async_validations.push(quote! {
                    if let Err(err) = #validation {
                        errors.push(#error_ident::#field_error_ident(err));
                    }
                });
            }
        }

        tokens.append_all(quote! {
            use fortifier::*;

            #[allow(dead_code)]
            #[derive(Debug)]
            enum #error_ident {
                #( #error_field_idents(#error_field_types) ),*
            }

            #[automatically_derived]
            impl ::std::fmt::Display for #error_ident {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, "{self:#?}")
                }
            }

            #[automatically_derived]
            impl ::std::error::Error for #error_ident {}

            #(#error_field_enums)*

            #[automatically_derived]
            impl Validate for #ident {
                type Error = #error_ident;

                fn validate_sync(&self) -> Result<(), ValidationErrors<Self::Error>> {
                    let mut errors = vec![];

                    #(#sync_validations)*

                    if !errors.is_empty() {
                        Err(errors.into())
                    } else {
                        Ok(())
                    }
                }

                fn validate_async(&self) -> ::std::pin::Pin<Box<impl Future<Output = Result<(), ValidationErrors<Self::Error>>>>> {
                    Box::pin(async {
                        let mut errors = vec![];

                        #(#async_validations)*

                        if !errors.is_empty() {
                            Err(errors.into())
                        } else {
                            Ok(())
                        }
                    })
                }
            }
        })
    }
}

pub struct ValidateUnnamedStruct {
    ident: Ident,
    error_ident: Ident,
    fields: Vec<ValidateField>,
}

impl ValidateUnnamedStruct {
    fn parse(input: &DeriveInput, _data: &DataStruct, fields: &FieldsUnnamed) -> Result<Self> {
        let mut result = Self {
            ident: input.ident.clone(),
            error_ident: format_ident!("{}ValidationError", input.ident),
            fields: Vec::default(),
        };

        for (index, field) in fields.unnamed.iter().enumerate() {
            let index = Literal::usize_unsuffixed(index);
            let field_ident = format_ident!("F{index}");
            let expr = quote!(self.#index);

            result.fields.push(ValidateField::parse(
                &input.ident,
                field_ident,
                expr,
                field,
            )?);
        }

        Ok(result)
    }
}

impl ToTokens for ValidateUnnamedStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let error_ident = &self.error_ident;
        let mut error_field_idents = vec![];
        let mut error_field_types = vec![];
        let mut error_field_enums = vec![];
        let mut sync_validations = vec![];
        let mut async_validations = vec![];

        for (index, field) in self.fields.iter().enumerate() {
            let field_error_ident = format_ident!("F{index}");

            let (error_type, error_enum) = field.error_type(ident, &field_error_ident);

            error_field_idents.push(field_error_ident.clone());
            error_field_types.push(error_type);
            if let Some(error_enum) = error_enum {
                error_field_enums.push(error_enum);
            }

            for validation in field.sync_validations() {
                sync_validations.push(quote! {
                    if let Err(err) = #validation {
                        errors.push(#error_ident::#field_error_ident(err));
                    }
                });
            }

            for validation in field.async_validations() {
                async_validations.push(quote! {
                    if let Err(err) = #validation {
                        errors.push(#error_ident::#field_error_ident(err));
                    }
                });
            }
        }

        tokens.append_all(quote! {
            use fortifier::*;

            #[allow(dead_code)]
            #[derive(Debug)]
            enum #error_ident {
                #( #error_field_idents(#error_field_types) ),*
            }

            #[automatically_derived]
            impl ::std::fmt::Display for #error_ident {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, "{self:#?}")
                }
            }

            #[automatically_derived]
            impl ::std::error::Error for #error_ident {}

            #(#error_field_enums)*

            #[automatically_derived]
            impl Validate for #ident {
                type Error = #error_ident;

                fn validate_sync(&self) -> Result<(), ValidationErrors<Self::Error>> {
                    let mut errors = vec![];

                    #(#sync_validations)*

                    if !errors.is_empty() {
                        Err(errors.into())
                    } else {
                        Ok(())
                    }
                }

                fn validate_async(&self) -> ::std::pin::Pin<Box<impl Future<Output = Result<(), ValidationErrors<Self::Error>>>>> {
                    Box::pin(async {
                        let mut errors = vec![];

                        #(#async_validations)*

                        if !errors.is_empty() {
                            Err(errors.into())
                        } else {
                            Ok(())
                        }
                    })
                }
            }
        })
    }
}

pub struct ValidateUnitStruct {
    ident: Ident,
}

impl ValidateUnitStruct {
    fn parse(input: &DeriveInput) -> Result<Self> {
        Ok(Self {
            ident: input.ident.clone(),
        })
    }
}

impl ToTokens for ValidateUnitStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;

        tokens.append_all(quote! {
            use fortifier::ValidationErrors;

            #[automatically_derived]
            impl Validate for #ident {
                type Error = ::std::convert::Infallible;

                fn validate_sync(&self) -> Result<(), ValidationErrors<Self::Error>> {
                    Ok(())
                }

                fn validate_async(&self) -> ::std::pin::Pin<Box<impl Future<Output = Result<(), ValidationErrors<Self::Error>>>>> {
                    Box::pin(async {
                        Ok(())
                    })
                }
            }
        });
    }
}
