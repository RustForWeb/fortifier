use proc_macro2::TokenStream;
use quote::quote;

pub fn enum_attributes() -> TokenStream {
    let mut attributes: Vec<TokenStream> = vec![];

    #[cfg(feature = "serde")]
    {
        use proc_macro_crate::crate_name;

        if crate_name("serde").is_ok() {
            attributes.push(quote! {
                #[derive(serde::Deserialize, serde::Serialize)]
                #[serde(
                    // TODO: Tag?
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
