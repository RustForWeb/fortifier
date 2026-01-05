use proc_macro2::TokenStream;
use quote::quote;

pub fn enum_attributes() -> TokenStream {
    #[allow(unused_mut)]
    let mut attributes: Vec<TokenStream> = vec![];

    #[cfg(feature = "serde")]
    {
        use proc_macro_crate::crate_name;

        if crate_name("serde").is_ok() {
            attributes.push(quote! {
                #[derive(serde::Deserialize, serde::Serialize)]
                #[serde(
                    tag = "path",
                    rename_all = "camelCase",
                    rename_all_fields = "camelCase"
                )]
            });
        }
    }

    #[cfg(feature = "utoipa")]
    {
        use proc_macro_crate::crate_name;

        if crate_name("utoipa").is_ok() {
            attributes.push(quote! {
                #[derive(utoipa::ToSchema)]
            });
        }
    }

    quote! {
        #( #attributes )*
    }
}

pub fn enum_field_attributes() -> TokenStream {
    #[allow(unused_mut)]
    let mut attributes: Vec<TokenStream> = vec![];

    #[cfg(feature = "serde")]
    {
        use proc_macro_crate::crate_name;

        if crate_name("serde").is_ok() {
            attributes.push(quote! {
                #[serde(with = "::fortifier::serde::errors")]
            });
        }
    }

    quote! {
        #( #attributes )*
    }
}

pub fn where_predicate(error_type: TokenStream) -> TokenStream {
    #[allow(unused_mut)]
    let mut lifetime = TokenStream::new();
    #[allow(unused_mut)]
    let mut traits = TokenStream::new();

    #[cfg(feature = "serde")]
    {
        use proc_macro_crate::crate_name;

        if crate_name("serde").is_ok() {
            lifetime = quote!(for<'fde>);
            traits = quote!(#traits + ::serde::Deserialize<'fde> + ::serde::Serialize);
        }
    }
    #[cfg(feature = "utoipa")]
    {
        use proc_macro_crate::crate_name;

        if crate_name("utoipa").is_ok() {
            traits = quote!(#traits + ::utoipa::ToSchema);
        }
    }

    quote! {
        #lifetime #error_type: ::std::fmt::Debug + PartialEq #traits
    }
}
