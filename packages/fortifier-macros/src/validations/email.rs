use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, LitBool, LitInt, Result, meta::ParseNestedMeta};

use crate::validation::Validation;

pub struct Email {
    allow_display_text: bool,
    allow_domain_literal: bool,
    minimum_sub_domains: usize,
}

impl Default for Email {
    fn default() -> Self {
        Self {
            allow_display_text: false,
            allow_domain_literal: true,
            minimum_sub_domains: 0,
        }
    }
}

impl Validation for Email {
    fn parse(meta: &ParseNestedMeta<'_>) -> Result<Self> {
        let mut result = Email::default();

        if !meta.input.is_empty() {
            meta.parse_nested_meta(|meta| {
                if meta.path.is_ident("allow_display_text") {
                    let lit: LitBool = meta.value()?.parse()?;
                    result.allow_display_text = lit.value;

                    Ok(())
                } else if meta.path.is_ident("allow_domain_literal") {
                    let lit: LitBool = meta.value()?.parse()?;
                    result.allow_domain_literal = lit.value;

                    Ok(())
                } else if meta.path.is_ident("minimum_sub_domains") {
                    let lit: LitInt = meta.value()?.parse()?;
                    result.minimum_sub_domains = lit.base10_parse()?;

                    Ok(())
                } else {
                    Err(meta.error("unknown parameter"))
                }
            })?;
        }

        Ok(result)
    }

    fn is_async(&self) -> bool {
        false
    }

    fn ident(&self) -> Ident {
        format_ident!("Email")
    }

    fn error_type(&self) -> TokenStream {
        quote!(EmailError)
    }

    fn tokens(&self, expr: &TokenStream) -> TokenStream {
        let allow_display_text = self.allow_display_text;
        let allow_domain_literal = self.allow_domain_literal;
        let minimum_sub_domains = self.minimum_sub_domains;

        quote! {
            {
                const EMAIL_ADDRESS_OPTIONS: EmailOptions = EmailOptions {
                    allow_display_text: #allow_display_text,
                    allow_domain_literal: #allow_domain_literal,
                    minimum_sub_domains: #minimum_sub_domains,
                };

                #expr.validate_email(EMAIL_ADDRESS_OPTIONS)
            }
        }
    }
}
