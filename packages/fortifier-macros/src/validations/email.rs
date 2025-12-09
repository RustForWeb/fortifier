use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, LitBool, LitInt, Result, meta::ParseNestedMeta};

use crate::validation::{Execution, Validation};

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

    fn ident(&self) -> Ident {
        format_ident!("Email")
    }
    fn error_type(&self) -> TokenStream {
        quote!(::fortifier::EmailError)
    }

    fn expr(&self, execution: Execution, expr: &TokenStream) -> Option<TokenStream> {
        match execution {
            Execution::Sync => {
                let allow_display_text = self.allow_display_text;
                let allow_domain_literal = self.allow_domain_literal;
                let minimum_sub_domains = self.minimum_sub_domains;

                Some(quote! {
                    {
                        const EMAIL_ADDRESS_OPTIONS: ::fortifier::EmailOptions = ::fortifier::EmailOptions {
                            allow_display_text: #allow_display_text,
                            allow_domain_literal: #allow_domain_literal,
                            minimum_sub_domains: #minimum_sub_domains,
                        };

                        ::fortifier::ValidateEmail::validate_email(&#expr, EMAIL_ADDRESS_OPTIONS)
                    }
                })
            }
            Execution::Async => None,
        }
    }
}
