use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    GenericArgument, Generics, Path, PathArguments, PathSegment, Type, TypeParamBound,
    WherePredicate, punctuated::Punctuated, token::PathSep,
};

use crate::validate::error::format_error_ident;

/// Primitive and built-in types.
///
/// Only single identifiers without generics.
const PRIMITIVE_AND_BUILT_IN_TYPES: [&str; 56] = [
    "bool",
    "i8",
    "i16",
    "i32",
    "i64",
    "i128",
    "isize",
    "u8",
    "u16",
    "u32",
    "u64",
    "u128",
    "usize",
    "f32",
    "f64",
    "char",
    "str",
    "String",
    "AtomicBool",
    "AtomicI16",
    "AtomicI32",
    "AtomicI64",
    "AtomicI8",
    "AtomicIsize",
    "AtomicPtr",
    "AtomicU16",
    "AtomicU32",
    "AtomicU64",
    "AtomicU8",
    "AtomicUsize",
    "CStr",
    "CString",
    "Duration",
    "Instant",
    "IpAddr",
    "Ipv4Addr",
    "Ipv6Adr",
    "NonZeroI128",
    "NonZeroI16",
    "NonZeroI32",
    "NonZeroI64",
    "NonZeroI8",
    "NonZeroIsize",
    "NonZeroU128",
    "NonZeroU16",
    "NonZeroU32",
    "NonZeroU64",
    "NonZeroU8",
    "NonZeroUsize",
    "OsStr",
    "OsString",
    "Path",
    "PathBuf",
    "SocketAddr",
    "SocketAddrV4",
    "SocketAddrV6",
];

/// Built-in types.
///
/// Only paths and single identifiers with generics.
const BUILT_IN_TYPES: [&str; 40] = [
    "PhantomData",
    "std::ffi::CStr",
    "std::ffi::CString",
    "std::ffi::OsStr",
    "std::ffi::OsString",
    "std::marker::PhantomData",
    "std::net::IpAddr",
    "std::net::Ipv4Addr",
    "std::net::Ipv6Adr",
    "std::net::SocketAddr",
    "std::net::SocketAddrV4",
    "std::net::SocketAddrV6",
    "std::num::NonZeroI128",
    "std::num::NonZeroI16",
    "std::num::NonZeroI32",
    "std::num::NonZeroI64",
    "std::num::NonZeroI8",
    "std::num::NonZeroIsize",
    "std::num::NonZeroU128",
    "std::num::NonZeroU16",
    "std::num::NonZeroU32",
    "std::num::NonZeroU64",
    "std::num::NonZeroU8",
    "std::num::NonZeroUsize",
    "std::path::Path",
    "std::path::PathBuf",
    "std::sync::atomic::AtomicBool",
    "std::sync::atomic::AtomicI16",
    "std::sync::atomic::AtomicI32",
    "std::sync::atomic::AtomicI64",
    "std::sync::atomic::AtomicI8",
    "std::sync::atomic::AtomicIsize",
    "std::sync::atomic::AtomicPtr",
    "std::sync::atomic::AtomicU16",
    "std::sync::atomic::AtomicU32",
    "std::sync::atomic::AtomicU64",
    "std::sync::atomic::AtomicU8",
    "std::sync::atomic::AtomicUsize",
    "std::time::Duration",
    "std::time::Instant",
];

/// Container types.
const CONTAINER_TYPES: [&str; 12] = [
    "Arc",
    "NonZero",
    "Option",
    "Rc",
    "Saturating",
    "Wrapping",
    "std::num::NonZero",
    "std::num::Saturating",
    "std::num::Wrapping",
    "std::option::Option",
    "std::rc::Rc",
    "std::sync::Arc",
];

/// Indexed container types.
const INDEXED_CONTAINER_TYPES: [&str; 12] = [
    "BTreeSet",
    "HashSet",
    "IndexSet",
    "LinkedList",
    "Vec",
    "VecDeque",
    "indexmap::set::IndexMap",
    "std::collections::BTreeSet",
    "std::collections::HashSet",
    "std::collections::LinkedList",
    "std::collections::VecDeque",
    "std::vec::Vec",
];

/// Keyed container types.
const KEYED_CONTAINER_TYPES: [&str; 6] = [
    "BTreeMap",
    "HashMap",
    "IndexMap",
    "indexmap::map::IndexMap",
    "std::collections::BTreeMap",
    "std::collections::HashMap",
];

fn path_to_string(path: &Path) -> String {
    path.segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

fn is_validate_path(path: &Path) -> bool {
    let path_string = path_to_string(path);
    path_string == "Validate"
        || path_string == "ValidateWithContext"
        || path_string == "fortifier::Validate"
        || path_string == "fortifier::ValidateWithContext"
}

fn should_validate_generic_argument(
    generics: &Generics,
    arg: &GenericArgument,
) -> Option<KnownOrUnknown<TokenStream>> {
    match arg {
        GenericArgument::Lifetime(_) => Some(KnownOrUnknown::Unknown),
        GenericArgument::Type(r#type) => should_validate_type(generics, r#type),
        // TODO: Const.
        GenericArgument::Const(_expr) => Some(KnownOrUnknown::Unknown),
        // TODO: Associated type.
        GenericArgument::AssocType(_assoc_type) => Some(KnownOrUnknown::Unknown),
        // TODO: Associated const.
        GenericArgument::AssocConst(_assoc_const) => Some(KnownOrUnknown::Unknown),
        // TODO: Constraint.
        GenericArgument::Constraint(_constraint) => Some(KnownOrUnknown::Unknown),
        _ => Some(KnownOrUnknown::Unknown),
    }
}

fn should_validate_type_param_bounds<'a>(
    mut bounds: impl Iterator<Item = &'a TypeParamBound>,
) -> Option<KnownOrUnknown<TokenStream>> {
    bounds
        .any(|bound| matches!(bound, TypeParamBound::Trait(bound) if is_validate_path(&bound.path)))
        .then_some(KnownOrUnknown::Unknown)
}

fn should_validate_path(generics: &Generics, path: &Path) -> Option<KnownOrUnknown<TokenStream>> {
    if let Some(ident) = path.get_ident() {
        if PRIMITIVE_AND_BUILT_IN_TYPES.contains(&ident.to_string().as_str()) {
            return None;
        }

        if let Some(param) = generics.type_params().find(|param| param.ident == *ident) {
            return should_validate_type_param_bounds(param.bounds.iter()).or_else(|| {
                generics.where_clause.as_ref().and_then(|where_clause| {
                    where_clause.predicates.iter().find_map(|predicate| {
                        if let WherePredicate::Type(predicate) = predicate
                            && let Type::Path(predicate_type) = &predicate.bounded_ty
                            && predicate_type.path.is_ident(ident)
                        {
                            should_validate_type_param_bounds(predicate.bounds.iter())
                        } else {
                            None
                        }
                    })
                })
            });
        }
    }

    let path_string = path_to_string(path);
    let path_string = path_string.as_str();

    if BUILT_IN_TYPES.contains(&path_string) {
        return None;
    }

    if CONTAINER_TYPES.contains(&path_string)
        && let Some(segment) = path.segments.last()
        && let PathArguments::AngleBracketed(arguments) = &segment.arguments
        && let Some(argument) = arguments.args.first()
    {
        return should_validate_generic_argument(generics, argument);
    }

    if INDEXED_CONTAINER_TYPES.contains(&path_string)
        && let Some(segment) = path.segments.last()
        && let PathArguments::AngleBracketed(arguments) = &segment.arguments
        && let Some(argument) = arguments.args.first()
    {
        return should_validate_generic_argument(generics, argument).map(|error_type| {
            match error_type {
                KnownOrUnknown::Known(error_type) => {
                    KnownOrUnknown::Known(quote!(::fortifier::IndexedValidationError<#error_type>))
                }
                KnownOrUnknown::Unknown => KnownOrUnknown::Unknown,
            }
        });
    }

    // TODO: Determine error type.
    if KEYED_CONTAINER_TYPES.contains(&path_string)
        && let Some(segment) = path.segments.last()
        && let PathArguments::AngleBracketed(arguments) = &segment.arguments
        && !arguments
            .args
            .iter()
            .all(|arg| should_validate_generic_argument(generics, arg).is_some())
    {
        return None;
    }

    let path = Punctuated::<PathSegment, PathSep>::from_iter(
        path.segments
            .iter()
            .take(path.segments.len() - 1)
            .cloned()
            .chain(
                path.segments
                    .iter()
                    .skip(path.segments.len() - 1)
                    .map(|segment| PathSegment {
                        ident: format_error_ident(&segment.ident),
                        arguments: PathArguments::None,
                    }),
            ),
    );

    Some(KnownOrUnknown::Known(path.to_token_stream()))
}

#[derive(Debug, PartialEq)]
pub enum KnownOrUnknown<T> {
    Known(T),
    Unknown,
}

impl<T> KnownOrUnknown<T> {
    pub fn map<F, U>(self, f: F) -> KnownOrUnknown<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            KnownOrUnknown::Known(value) => KnownOrUnknown::Known(f(value)),
            KnownOrUnknown::Unknown => KnownOrUnknown::Unknown,
        }
    }
}

pub fn should_validate_type(
    generics: &Generics,
    r#type: &Type,
) -> Option<KnownOrUnknown<TokenStream>> {
    match r#type {
        Type::Array(r#type) => {
            should_validate_type(generics, &r#type.elem).map(|error_type| {
                error_type.map(|error_type| quote!(::fortifier::IndexedValidationError<#error_type>))
            })
        },
        Type::BareFn(_) => None,
        Type::Group(r#type) => should_validate_type(generics, &r#type.elem),
        Type::ImplTrait(r#type) => {
            r#type.bounds
                .iter()
                .any(|bound| matches!(bound, TypeParamBound::Trait(bound) if is_validate_path(&bound.path)))
                .then_some(KnownOrUnknown::Unknown)
        },
        Type::Infer(_) => Some(KnownOrUnknown::Unknown),
        Type::Macro(_) => Some(KnownOrUnknown::Unknown),
        Type::Never(_) => None,
        Type::Paren(r#type) => should_validate_type(generics, &r#type.elem),
        Type::Path(r#type) => should_validate_path(generics, &r#type.path),
        Type::Ptr(r#type) => should_validate_type(generics, &r#type.elem),
        Type::Reference(r#type) => should_validate_type(generics,&r#type.elem),
        Type::Slice(r#type) => {
            should_validate_type(generics, &r#type.elem).map(|error_type| {
                error_type.map(|error_type| quote!(::fortifier::IndexedValidationError<#error_type>))
            })
        },
        Type::TraitObject(r#type) => should_validate_type_param_bounds(r#type.bounds.iter()),
        Type::Tuple(r#type) => {
            (!r#type.elems.is_empty() && r#type.elems.iter().all(|r#type| should_validate_type(generics, r#type).is_some())).then_some(KnownOrUnknown::Unknown)
        }
        Type::Verbatim(_) => None,
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{GenericParam, Generics, punctuated::Punctuated};

    use crate::validate::r#type::KnownOrUnknown;

    use super::should_validate_type;

    fn validate(tokens: TokenStream) -> Option<KnownOrUnknown<String>> {
        validate_with_generics(tokens, Generics::default())
    }

    fn validate_with_generics(
        tokens: TokenStream,
        generics: Generics,
    ) -> Option<KnownOrUnknown<String>> {
        should_validate_type(&generics, &syn::parse2(tokens).expect("valid type"))
            .map(|value| value.map(|value| value.to_string().replace(' ', "")))
    }

    #[test]
    fn should_validate() {
        // TODO: Keyed error types.

        assert_eq!(
            validate(quote!(&T)),
            Some(KnownOrUnknown::Known("TValidationError".to_owned()))
        );
        assert_eq!(
            validate(quote!(T)),
            Some(KnownOrUnknown::Known("TValidationError".to_owned()))
        );
        assert_eq!(
            validate(quote!(T<usize>)),
            Some(KnownOrUnknown::Known("TValidationError".to_owned()))
        );
        assert_eq!(
            validate(quote!(T<u8, u8>)),
            Some(KnownOrUnknown::Known("TValidationError".to_owned()))
        );

        assert_eq!(validate(quote!((T, T))), Some(KnownOrUnknown::Unknown));
        assert_eq!(validate(quote!((A, B, C))), Some(KnownOrUnknown::Unknown));

        assert_eq!(
            validate(quote!([T])),
            Some(KnownOrUnknown::Known(
                "::fortifier::IndexedValidationError<TValidationError>".to_owned()
            ))
        );
        assert_eq!(
            validate(quote!([T; 3])),
            Some(KnownOrUnknown::Known(
                "::fortifier::IndexedValidationError<TValidationError>".to_owned()
            ))
        );
        assert_eq!(
            validate(quote!([&T])),
            Some(KnownOrUnknown::Known(
                "::fortifier::IndexedValidationError<TValidationError>".to_owned()
            ))
        );
        assert_eq!(
            validate(quote!([&T; 3])),
            Some(KnownOrUnknown::Known(
                "::fortifier::IndexedValidationError<TValidationError>".to_owned()
            ))
        );
        assert_eq!(
            validate(quote!(&[T])),
            Some(KnownOrUnknown::Known(
                "::fortifier::IndexedValidationError<TValidationError>".to_owned()
            ))
        );
        assert_eq!(
            validate(quote!(&[T; 3])),
            Some(KnownOrUnknown::Known(
                "::fortifier::IndexedValidationError<TValidationError>".to_owned()
            ))
        );

        assert_eq!(
            validate(quote!(Arc<T>)),
            Some(KnownOrUnknown::Known("TValidationError".to_owned()))
        );
        assert_eq!(
            validate(quote!(BTreeSet<T>)),
            Some(KnownOrUnknown::Known(
                "::fortifier::IndexedValidationError<TValidationError>".to_owned()
            ))
        );
        // assert_eq!(
        //     validate(quote!(BTreeMap<K, V>)),
        //     Some(KnownOrUnknown::Known(
        //         "::fortifier::KeyedValidationError<K, TValidationError>".to_owned()
        //     ))
        // );
        assert_eq!(
            validate(quote!(IndexSet<T>)),
            Some(KnownOrUnknown::Known(
                "::fortifier::IndexedValidationError<TValidationError>".to_owned()
            ))
        );
        // assert_eq!(
        //     validate(quote!(IndexMap<K, V>)),
        //     Some(KnownOrUnknown::Known(
        //         "::fortifier::KeyedValidationError<K, TValidationError>".to_owned()
        //     ))
        // );
        assert_eq!(
            validate(quote!(HashSet<T>)),
            Some(KnownOrUnknown::Known(
                "::fortifier::IndexedValidationError<TValidationError>".to_owned()
            ))
        );
        // assert_eq!(
        //     validate(quote!(HashMap<K, V>)),
        //     Some(KnownOrUnknown::Known(
        //         "::fortifier::KeyedValidationError<K, TValidationError>".to_owned()
        //     ))
        // );
        assert_eq!(
            validate(quote!(LinkedList<T>)),
            Some(KnownOrUnknown::Known(
                "::fortifier::IndexedValidationError<TValidationError>".to_owned()
            ))
        );
        assert_eq!(
            validate(quote!(Option<T>)),
            Some(KnownOrUnknown::Known("TValidationError".to_owned()))
        );
        assert_eq!(
            validate(quote!(Option<Option<T>>)),
            Some(KnownOrUnknown::Known("TValidationError".to_owned()))
        );
        assert_eq!(
            validate(quote!(Rc<T>)),
            Some(KnownOrUnknown::Known("TValidationError".to_owned()))
        );
        assert_eq!(
            validate(quote!(Vec<T>)),
            Some(KnownOrUnknown::Known(
                "::fortifier::IndexedValidationError<TValidationError>".to_owned()
            ))
        );
        assert_eq!(
            validate(quote!(VecDeque<T>)),
            Some(KnownOrUnknown::Known(
                "::fortifier::IndexedValidationError<TValidationError>".to_owned()
            ))
        );

        assert_eq!(
            validate(quote!(impl Validate)),
            Some(KnownOrUnknown::Unknown)
        );
        assert_eq!(
            validate(quote!(impl ValidateWithContext)),
            Some(KnownOrUnknown::Unknown)
        );
        assert_eq!(
            validate(quote!(impl ValidateWithContext<Context = ()>)),
            Some(KnownOrUnknown::Unknown)
        );
        assert_eq!(
            validate(quote!(impl fortifier::Validate)),
            Some(KnownOrUnknown::Unknown)
        );
        assert_eq!(
            validate(quote!(impl fortifier::ValidateWithContext)),
            Some(KnownOrUnknown::Unknown)
        );
        assert_eq!(
            validate(quote!(impl fortifier::ValidateWithContext<Context = ()>)),
            Some(KnownOrUnknown::Unknown)
        );
        assert_eq!(
            validate(quote!(dyn Validate)),
            Some(KnownOrUnknown::Unknown)
        );
        assert_eq!(
            validate(quote!(dyn ValidateWithContext)),
            Some(KnownOrUnknown::Unknown)
        );
        assert_eq!(
            validate(quote!(dyn ValidateWithContext<Context = ()>)),
            Some(KnownOrUnknown::Unknown)
        );
        assert_eq!(
            validate(quote!(dyn ::fortifier::Validate)),
            Some(KnownOrUnknown::Unknown)
        );
        assert_eq!(
            validate(quote!(dyn ::fortifier::ValidateWithContext)),
            Some(KnownOrUnknown::Unknown)
        );
        assert_eq!(
            validate(quote!(dyn ::fortifier::ValidateWithContext<Context = ()>)),
            Some(KnownOrUnknown::Unknown)
        );
    }

    #[test]
    fn should_not_validate() {
        assert_eq!(validate(quote!(bool)), None);
        assert_eq!(validate(quote!(i8)), None);
        assert_eq!(validate(quote!(i16)), None);
        assert_eq!(validate(quote!(i32)), None);
        assert_eq!(validate(quote!(i64)), None);
        assert_eq!(validate(quote!(i128)), None);
        assert_eq!(validate(quote!(isize)), None);
        assert_eq!(validate(quote!(u8)), None);
        assert_eq!(validate(quote!(u16)), None);
        assert_eq!(validate(quote!(u32)), None);
        assert_eq!(validate(quote!(u64)), None);
        assert_eq!(validate(quote!(u128)), None);
        assert_eq!(validate(quote!(usize)), None);
        assert_eq!(validate(quote!(f32)), None);
        assert_eq!(validate(quote!(f64)), None);
        assert_eq!(validate(quote!(char)), None);
        assert_eq!(validate(quote!(&str)), None);
        assert_eq!(validate(quote!(String)), None);

        assert_eq!(validate(quote!(())), None);
        assert_eq!(validate(quote!((bool, bool))), None);
        assert_eq!(validate(quote!((usize, usize, usize))), None);
        assert_eq!(validate(quote!((usize, &str))), None);

        assert_eq!(validate(quote!([isize])), None);
        assert_eq!(validate(quote!([&str; 3])), None);
        assert_eq!(validate(quote!(&[isize])), None);
        assert_eq!(validate(quote!(&[&str; 3])), None);

        assert_eq!(validate(quote!(Arc<&str>)), None);
        assert_eq!(validate(quote!(BTreeSet<usize>)), None);
        assert_eq!(validate(quote!(BTreeMap<usize, &str>)), None);
        assert_eq!(validate(quote!(IndexSet<&str>)), None);
        assert_eq!(validate(quote!(IndexMap<&str, &str>)), None);
        assert_eq!(validate(quote!(HashSet<&str>)), None);
        assert_eq!(validate(quote!(HashMap<&str, &str>)), None);
        assert_eq!(validate(quote!(LinkedList<char>)), None);
        assert_eq!(validate(quote!(Option<char>)), None);
        assert_eq!(validate(quote!(Option<Option<String>>)), None);
        assert_eq!(validate(quote!(Rc<&str>)), None);
        assert_eq!(validate(quote!(Vec<usize>)), None);
        assert_eq!(validate(quote!(VecDeque<String>)), None);

        assert_eq!(validate(quote!(impl Serialize)), None);
        assert_eq!(validate(quote!(dyn Serialize)), None);
    }

    #[test]
    fn should_validate_with_generics() {
        // TODO: Output error type as `<T as ValidateWithContext>::Error` if possible.

        assert_eq!(
            validate_with_generics(
                quote!(T),
                Generics {
                    lt_token: Default::default(),
                    params: Punctuated::from_iter([syn::parse2::<GenericParam>(
                        quote!(T: Validate)
                    )
                    .expect("valid generic param")]),
                    gt_token: Default::default(),
                    where_clause: None
                }
            ),
            Some(KnownOrUnknown::Unknown)
        );
        assert_eq!(
            validate_with_generics(
                quote!([T]),
                Generics {
                    lt_token: Default::default(),
                    params: Punctuated::from_iter([syn::parse2::<GenericParam>(
                        quote!(T: Validate)
                    )
                    .expect("valid generic param")]),
                    gt_token: Default::default(),
                    where_clause: None
                }
            ),
            Some(KnownOrUnknown::Unknown)
        );
        assert_eq!(
            validate_with_generics(
                quote!(T),
                Generics {
                    lt_token: Default::default(),
                    params: Punctuated::from_iter([syn::parse2::<GenericParam>(
                        quote!(T: ValidateWithContext)
                    )
                    .expect("valid generic param")]),
                    gt_token: Default::default(),
                    where_clause: None
                }
            ),
            Some(KnownOrUnknown::Unknown)
        );
        assert_eq!(
            validate_with_generics(
                quote!(T),
                Generics {
                    lt_token: Default::default(),
                    params: Punctuated::from_iter([syn::parse2::<GenericParam>(
                        quote!(T: ValidateWithContext<Context = ()>)
                    )
                    .expect("valid generic param")]),
                    gt_token: Default::default(),
                    where_clause: None
                }
            ),
            Some(KnownOrUnknown::Unknown)
        );

        assert_eq!(
            validate_with_generics(
                quote!(T),
                Generics {
                    lt_token: Default::default(),
                    params: Punctuated::from_iter([
                        syn::parse2::<GenericParam>(quote!(T)).expect("valid generic param")
                    ]),
                    gt_token: Default::default(),
                    where_clause: Some(
                        syn::parse2(quote!(where T: Validate)).expect("valid where clause")
                    )
                }
            ),
            Some(KnownOrUnknown::Unknown)
        );
        assert_eq!(
            validate_with_generics(
                quote!([T]),
                Generics {
                    lt_token: Default::default(),
                    params: Punctuated::from_iter([
                        syn::parse2::<GenericParam>(quote!(T)).expect("valid generic param")
                    ]),
                    gt_token: Default::default(),
                    where_clause: Some(
                        syn::parse2(quote!(where T: Validate)).expect("valid where clause")
                    )
                }
            ),
            Some(KnownOrUnknown::Unknown)
        );
        assert_eq!(
            validate_with_generics(
                quote!(T),
                Generics {
                    lt_token: Default::default(),
                    params: Punctuated::from_iter([
                        syn::parse2::<GenericParam>(quote!(T)).expect("valid generic param")
                    ]),
                    gt_token: Default::default(),
                    where_clause: Some(
                        syn::parse2(quote!(where T: ValidateWithContext))
                            .expect("valid where clause")
                    )
                }
            ),
            Some(KnownOrUnknown::Unknown)
        );
        assert_eq!(
            validate_with_generics(
                quote!(T),
                Generics {
                    lt_token: Default::default(),
                    params: Punctuated::from_iter([
                        syn::parse2::<GenericParam>(quote!(T)).expect("valid generic param")
                    ]),
                    gt_token: Default::default(),
                    where_clause: Some(
                        syn::parse2(quote!(where T: ValidateWithContext<Context = ()>))
                            .expect("valid where clause")
                    )
                }
            ),
            Some(KnownOrUnknown::Unknown)
        );
    }

    #[test]
    fn should_not_validate_with_generics() {
        assert_eq!(
            validate_with_generics(
                quote!(T),
                Generics {
                    lt_token: Default::default(),
                    params: Punctuated::from_iter([syn::parse2::<GenericParam>(
                        quote!(T: Serialize)
                    )
                    .expect("valid generic param")]),
                    gt_token: Default::default(),
                    where_clause: None
                }
            ),
            None
        );

        assert_eq!(
            validate_with_generics(
                quote!(T),
                Generics {
                    lt_token: Default::default(),
                    params: Punctuated::from_iter([
                        syn::parse2::<GenericParam>(quote!(T)).expect("valid generic param")
                    ]),
                    gt_token: Default::default(),
                    where_clause: Some(
                        syn::parse2(quote!(where T: Serialize)).expect("valid where clause")
                    )
                }
            ),
            None
        );
    }
}
