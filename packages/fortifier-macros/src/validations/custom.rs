use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Ident, LitBool, LitInt, Path, Result, Type, TypePath, meta::ParseNestedMeta};

use crate::{
    generics::{Generic, generic_arguments},
    util::{count_options, upper_camel_ident},
    validation::{Execution, Validation},
};

pub struct Custom {
    r#type: Type,
    name: Ident,
    error_type: TypePath,
    function_path: Path,
    execution: Execution,
    context: bool,
    max_options: usize,
}

impl Validation for Custom {
    fn parse(r#type: &Type, meta: &ParseNestedMeta<'_>) -> Result<Self> {
        let mut name = None;
        let mut error_type: Option<TypePath> = None;
        let mut function_path: Option<Path> = None;
        let mut execution = Execution::Sync;
        let mut context = false;
        let mut max_options = 0;

        meta.parse_nested_meta(|meta| {
            if meta.path.is_ident("async") {
                if let Ok(value) = meta.value() {
                    let lit: LitBool = value.parse()?;
                    execution = if lit.value {
                        Execution::Async
                    } else {
                        Execution::Sync
                    };
                } else {
                    execution = Execution::Async;
                }

                Ok(())
            } else if meta.path.is_ident("context") {
                if let Ok(value) = meta.value() {
                    let lit: LitBool = value.parse()?;
                    context = lit.value;
                } else {
                    context = true;
                }

                Ok(())
            } else if meta.path.is_ident("error") {
                error_type = Some(meta.value()?.parse()?);

                Ok(())
            } else if meta.path.is_ident("function") {
                function_path = Some(meta.value()?.parse()?);

                Ok(())
            } else if meta.path.is_ident("options") {
                if let Ok(value) = meta.value() {
                    let lit: LitInt = value.parse()?;
                    max_options = lit.base10_parse::<usize>()?;
                } else {
                    max_options = usize::MAX;
                }

                Ok(())
            } else if meta.path.is_ident("name") {
                let ident = meta.value()?.parse()?;
                name = Some(upper_camel_ident(&ident));

                Ok(())
            } else {
                Err(meta.error("unknown parameter"))
            }
        })?;

        let Some(error_type) = error_type else {
            return Err(meta.error("missing error parameter"));
        };
        let Some(function_path) = function_path else {
            return Err(meta.error("missing function parameter"));
        };

        let name = name.unwrap_or_else(|| {
            // TODO: Determine default ident from function or error type.
            format_ident!("Custom")
        });

        Ok(Custom {
            r#type: r#type.clone(),
            name,
            error_type,
            function_path,
            execution,
            context,
            max_options,
        })
    }

    fn ident(&self) -> Ident {
        self.name.clone()
    }

    fn error_type(&self) -> TokenStream {
        self.error_type.to_token_stream()
    }

    fn error_generics(&self) -> Vec<Generic> {
        generic_arguments(&self.error_type)
    }

    fn error_where_predicates(&self) -> Vec<TokenStream> {
        vec![]
    }

    fn expr(&self, execution: Execution, expr: &TokenStream) -> Option<TokenStream> {
        match (execution, self.execution) {
            (Execution::Sync, Execution::Sync) => Some(wrapper(
                &self.r#type,
                &self.function_path,
                expr,
                self.context,
                self.max_options,
                None,
            )),
            (Execution::Async, Execution::Async) => Some(wrapper(
                &self.r#type,
                &self.function_path,
                expr,
                self.context,
                self.max_options,
                Some(quote!(.await)),
            )),
            _ => None,
        }
    }
}

fn wrapper(
    r#type: &Type,
    function_path: &Path,
    expr: &TokenStream,
    context: bool,
    max_options: usize,
    suffix: Option<TokenStream>,
) -> TokenStream {
    let context_expr = context.then(|| quote!(, &context));

    let count = count_options(r#type);
    let remove_count = count.saturating_sub(max_options);

    if remove_count > 0 {
        let mut wrapper = quote!(value);
        for _ in 0..remove_count {
            wrapper = quote!(Some(#wrapper));
        }

        quote! {
            { if let #wrapper = &#expr { #function_path(value #context_expr) #suffix} else { Ok(())} }
        }
    } else {
        quote! {
            #function_path(&#expr #context_expr) #suffix
        }
    }
}
