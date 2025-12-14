use syn::{GenericArgument, Path, PathArguments, Type, TypeParamBound};

const PRIMITIVE_AND_BUILT_IN_TYPES: [&str; 18] = [
    "bool", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
    "f32", "f64", "char", "str", "String",
];

const CONTAINER_TYPES: [&str; 20] = [
    "Arc",
    "BTreeMap",
    "BTreeSet",
    "HashMap",
    "HashSet",
    "LinkedList",
    "Option",
    "Rc",
    "Vec",
    "VecDeque",
    "std::collections::BTreeMap",
    "std::collections::BTreeSet",
    "std::collections::HashMap",
    "std::collections::HashSet",
    "std::collections::LinkedList",
    "std::collections::VecDeque",
    "std::option::Option",
    "std::rc::Rc",
    "std::sync::Arc",
    "std::vec::Vec",
];

fn path_to_string(path: &Path) -> String {
    // TODO: This is probably slow, replace with comparisons.
    path.segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

fn is_validate_path(path: &Path) -> bool {
    let path_string = path_to_string(path);
    path_string == "Validate" || path_string == "fortifier::Validate"
}

fn should_validate_generic_argument(arg: &GenericArgument) -> bool {
    match arg {
        GenericArgument::Lifetime(_) => true,
        GenericArgument::Type(r#type) => should_validate_type(r#type),
        GenericArgument::Const(_expr) => todo!(),
        GenericArgument::AssocType(_assoc_type) => todo!(),
        GenericArgument::AssocConst(_assoc_const) => todo!(),
        GenericArgument::Constraint(_constraint) => todo!(),
        _ => true,
    }
}

fn should_validate_path(path: &Path) -> bool {
    if let Some(ident) = path.get_ident() {
        return !PRIMITIVE_AND_BUILT_IN_TYPES.contains(&ident.to_string().as_str());
    }
    let path_string = path_to_string(path);

    if CONTAINER_TYPES.contains(&path_string.as_str())
        && let Some(segment) = path.segments.last()
        && let PathArguments::AngleBracketed(arguments) = &segment.arguments
        && !arguments.args.iter().all(should_validate_generic_argument)
    {
        return false;
    }

    true
}

pub fn should_validate_type(r#type: &Type) -> bool {
    match r#type {
        Type::Array(r#type) => should_validate_type(&r#type.elem),
        Type::BareFn(_) => false,
        Type::Group(r#type) => should_validate_type(&r#type.elem),
        Type::ImplTrait(r#type) => r#type.bounds.iter().any(
            |bound| matches!(bound, TypeParamBound::Trait(bound) if is_validate_path(&bound.path)),
        ),
        Type::Infer(_) => true,
        Type::Macro(_) => true,
        Type::Never(_) => false,
        Type::Paren(r#type) => should_validate_type(&r#type.elem),
        Type::Path(r#type) => should_validate_path(&r#type.path),
        Type::Ptr(r#type) => should_validate_type(&r#type.elem),
        Type::Reference(r#type) => should_validate_type(&r#type.elem),
        Type::Slice(r#type) => should_validate_type(&r#type.elem),
        Type::TraitObject(r#type) => r#type.bounds.iter().any(
            |bound| matches!(bound, TypeParamBound::Trait(bound) if is_validate_path(&bound.path)),
        ),
        Type::Tuple(r#type) => {
            !r#type.elems.is_empty() && r#type.elems.iter().all(should_validate_type)
        }
        Type::Verbatim(_) => false,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::quote;

    use super::should_validate_type;

    fn validate(tokens: TokenStream) -> bool {
        should_validate_type(&syn::parse2(tokens).expect("valid type"))
    }

    #[test]
    fn should_validate() {
        assert!(validate(quote!(&T)));
        assert!(validate(quote!(T)));

        assert!(validate(quote!((T, T))));
        assert!(validate(quote!((A, B, C))));

        assert!(validate(quote!([T])));
        assert!(validate(quote!([T; 3])));
        assert!(validate(quote!([&T])));
        assert!(validate(quote!([&T; 3])));
        assert!(validate(quote!(&[T])));
        assert!(validate(quote!(&[T; 3])));

        assert!(validate(quote!(Arc<T>)));
        assert!(validate(quote!(BTreeSet<T>)));
        assert!(validate(quote!(BTreeMap<K, V>)));
        assert!(validate(quote!(HashSet<T>)));
        assert!(validate(quote!(HashMap<K, V>)));
        assert!(validate(quote!(LinkedList<T>)));
        assert!(validate(quote!(Option<T>)));
        assert!(validate(quote!(Option<Option<T>>)));
        assert!(validate(quote!(Rc<T>)));
        assert!(validate(quote!(Vec<T>)));
        assert!(validate(quote!(VecDeque<T>)));

        assert!(validate(quote!(impl Validate)));
        assert!(validate(quote!(impl fortifier::Validate)));
        assert!(validate(quote!(dyn Validate)));
        assert!(validate(quote!(dyn ::fortifier::Validate)));
    }

    #[test]
    fn should_not_validate() {
        assert!(!validate(quote!(bool)));
        assert!(!validate(quote!(i8)));
        assert!(!validate(quote!(i16)));
        assert!(!validate(quote!(i32)));
        assert!(!validate(quote!(i64)));
        assert!(!validate(quote!(i128)));
        assert!(!validate(quote!(isize)));
        assert!(!validate(quote!(u8)));
        assert!(!validate(quote!(u16)));
        assert!(!validate(quote!(u32)));
        assert!(!validate(quote!(u64)));
        assert!(!validate(quote!(u128)));
        assert!(!validate(quote!(usize)));
        assert!(!validate(quote!(f32)));
        assert!(!validate(quote!(f64)));
        assert!(!validate(quote!(char)));
        assert!(!validate(quote!(&str)));
        assert!(!validate(quote!(String)));

        assert!(!validate(quote!(())));
        assert!(!validate(quote!((bool, bool))));
        assert!(!validate(quote!((usize, usize, usize))));
        assert!(!validate(quote!((usize, &str))));

        assert!(!validate(quote!([isize])));
        assert!(!validate(quote!([&str; 3])));
        assert!(!validate(quote!(&[isize])));
        assert!(!validate(quote!(&[&str; 3])));

        assert!(!validate(quote!(Arc<&str>)));
        assert!(!validate(quote!(BTreeSet<usize>)));
        assert!(!validate(quote!(BTreeMap<usize, &str>)));
        assert!(!validate(quote!(HashSet<&str>)));
        assert!(!validate(quote!(HashMap<&str, &str>)));
        assert!(!validate(quote!(LinkedList<char>)));
        assert!(!validate(quote!(Option<char>)));
        assert!(!validate(quote!(Option<Option<String>>)));
        assert!(!validate(quote!(Rc<&str>)));
        assert!(!validate(quote!(Vec<usize>)));
        assert!(!validate(quote!(VecDeque<String>)));

        assert!(!validate(quote!(impl Serialize)));
        assert!(!validate(quote!(dyn Serialize)));
    }
}
