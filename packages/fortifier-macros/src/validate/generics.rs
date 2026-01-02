use syn::{
    ConstParam, GenericArgument, GenericParam, Generics, Lifetime, Path, PathArguments, Type,
    TypeParam, WhereClause, WherePredicate, punctuated::Punctuated,
};

pub fn filter_generics_by_generic_arguments(
    generics: &Generics,
    arguments: &[GenericArgument],
) -> Generics {
    let params = Punctuated::from_iter(
        generics
            .params
            .iter()
            .filter(|param| match param {
                GenericParam::Lifetime(param) => arguments
                    .iter()
                    .any(|argument| lifetime_matches_argument(&param.lifetime, argument)),
                GenericParam::Type(param) => arguments
                    .iter()
                    .any(|argument| type_param_matches_argument(param, argument)),
                GenericParam::Const(param) => arguments
                    .iter()
                    .any(|argument| const_param_matches_argument(param, argument)),
            })
            .cloned(),
    );

    let where_clause = generics
        .where_clause
        .as_ref()
        .map(|where_clause| WhereClause {
            where_token: Default::default(),
            predicates: Punctuated::from_iter(
                where_clause
                    .predicates
                    .iter()
                    .filter(|predicate| match predicate {
                        WherePredicate::Lifetime(predicate) => arguments.iter().any(|argument| {
                            lifetime_matches_argument(&predicate.lifetime, argument)
                        }),
                        WherePredicate::Type(predicate) => arguments
                            .iter()
                            .any(|argument| type_matches_argument(&predicate.bounded_ty, argument)),
                        _ => false,
                    })
                    .cloned(),
            ),
        });

    Generics {
        lt_token: None,
        params,
        gt_token: None,
        where_clause,
    }
}

fn lifetime_matches_argument(lifetime: &Lifetime, argument: &GenericArgument) -> bool {
    matches!(argument, GenericArgument::Lifetime(argument_lifetime) if argument_lifetime.ident == lifetime.ident)
}

fn type_matches_argument(r#type: &Type, argument: &GenericArgument) -> bool {
    match argument {
        GenericArgument::Lifetime(_) => false,
        GenericArgument::Type(argument_type) => type_includes_type(argument_type, r#type),
        GenericArgument::Const(_expr) => todo!("type matches generic argument const"),
        GenericArgument::AssocType(_assoc_type) => {
            todo!("type matches generic argument assoc type")
        }
        GenericArgument::AssocConst(_assoc_const) => {
            todo!("type matches generic argument assoc const")
        }
        GenericArgument::Constraint(_constraint) => {
            todo!("type matches generic argument constraint")
        }
        _ => false,
    }
}

fn type_param_matches_argument(_param: &TypeParam, argument: &GenericArgument) -> bool {
    match argument {
        GenericArgument::Lifetime(_) => false,
        GenericArgument::Type(r#type) => type_matches_argument(r#type, argument),
        GenericArgument::Const(_expr) => todo!("type param matches generic argument const"),
        GenericArgument::AssocType(_assoc_type) => {
            todo!("type param matches generic argument assoc type")
        }
        GenericArgument::AssocConst(_assoc_const) => {
            todo!("type param matches generic argument assoc const")
        }
        GenericArgument::Constraint(_constraint) => {
            todo!("type param matches generic argument constraint")
        }
        _ => false,
    }
}

fn const_param_matches_argument(_param: &ConstParam, argument: &GenericArgument) -> bool {
    match argument {
        GenericArgument::Lifetime(_) => false,
        GenericArgument::Type(_type) => todo!("const param matches generic argument type"),
        GenericArgument::Const(_expr) => todo!("const param matches generic argument const"),
        GenericArgument::AssocType(_assoc_type) => {
            todo!("const param matches generic argument assoc type")
        }
        GenericArgument::AssocConst(_assoc_const) => {
            todo!("const param matches generic argument assoc const")
        }
        GenericArgument::Constraint(_constraint) => {
            todo!("gconst param matches eneric argument constraint")
        }
        _ => todo!(),
    }
}

fn type_includes_type(haystack: &Type, needle: &Type) -> bool {
    if type_equals_type(haystack, needle) {
        true
    } else {
        // TODO: Recurse to check if haystack contains needle.
        false
    }
}

fn type_equals_type(a: &Type, b: &Type) -> bool {
    match (a, b) {
        (Type::Array(a), Type::Array(b)) => type_equals_type(&a.elem, &b.elem),
        (Type::Group(a), Type::Group(b)) => type_equals_type(&a.elem, &b.elem),
        (Type::Infer(_), Type::Infer(_)) => true,
        (Type::Never(_), Type::Never(_)) => true,
        (Type::Path(a), Type::Path(b)) => path_equals_path(&a.path, &b.path),
        (Type::Paren(a), Type::Paren(b)) => type_equals_type(&a.elem, &b.elem),
        (Type::Ptr(a), Type::Ptr(b)) => type_equals_type(&a.elem, &b.elem),
        (Type::Reference(a), Type::Reference(b)) => type_equals_type(&a.elem, &b.elem),
        (Type::Slice(a), Type::Slice(b)) => type_equals_type(&a.elem, &b.elem),
        (Type::Tuple(a), Type::Tuple(b)) => {
            a.elems.len() == b.elems.len()
                && a.elems
                    .iter()
                    .zip(b.elems.iter())
                    .all(|(a, b)| type_equals_type(a, b))
        }
        _ => false,
    }
}

fn path_equals_path(a: &Path, b: &Path) -> bool {
    a.segments.len() == b.segments.len()
        && a.segments.iter().zip(b.segments.iter()).all(|(a, b)| {
            a.ident == b.ident && path_argument_equals_path_argument(&a.arguments, &b.arguments)
        })
}

fn path_argument_equals_path_argument(a: &PathArguments, b: &PathArguments) -> bool {
    match (a, b) {
        (PathArguments::None, PathArguments::None) => true,
        (PathArguments::AngleBracketed(a), PathArguments::AngleBracketed(b)) => {
            todo!("path arguments angle bracketed {a:#?} == {b:#?}")
        }
        (PathArguments::Parenthesized(a), PathArguments::Parenthesized(b)) => {
            todo!("path arguments parenthesized {a:#?} == {b:#?}")
        }
        _ => false,
    }
}
