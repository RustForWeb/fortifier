use syn::{
    ConstParam, GenericArgument, GenericParam, Generics, Ident, Lifetime, Path, PathArguments,
    Type, TypeParam, TypePath, WhereClause, WherePredicate, punctuated::Punctuated,
};

#[derive(Clone)]
pub enum Generic {
    Argument(GenericArgument),
    Param(GenericParam),
}

pub fn generic_arguments(r#type: &TypePath) -> Vec<Generic> {
    if let Some(segment) = r#type.path.segments.last()
        && let PathArguments::AngleBracketed(arguments) = &segment.arguments
    {
        arguments
            .args
            .iter()
            .cloned()
            .map(Generic::Argument)
            .collect()
    } else {
        vec![]
    }
}

pub fn filter_generics(generics: &Generics, arguments_or_params: &[Generic]) -> Generics {
    let generic_arguments = arguments_or_params.iter().filter_map(|generic| {
        if let Generic::Argument(argument) = generic {
            Some(argument)
        } else {
            None
        }
    });
    let generic_params = arguments_or_params.iter().filter_map(|generic| {
        if let Generic::Param(param) = generic {
            Some(param)
        } else {
            None
        }
    });

    let params = Punctuated::from_iter(
        generics
            .params
            .iter()
            .filter(|param| {
                generic_params
                    .clone()
                    .any(|generic_param| generic_param_equals_generic_param(generic_param, param))
                    || match param {
                        GenericParam::Lifetime(param) => generic_arguments
                            .clone()
                            .any(|argument| lifetime_matches_argument(&param.lifetime, argument)),
                        GenericParam::Type(param) => generic_arguments
                            .clone()
                            .any(|argument| type_param_matches_argument(param, argument)),
                        GenericParam::Const(param) => generic_arguments
                            .clone()
                            .any(|argument| const_param_matches_argument(param, argument)),
                    }
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
                        WherePredicate::Lifetime(predicate) => {
                            generic_params
                                .clone()
                                .any(|param| lifetime_matches_param(&predicate.lifetime, param))
                                || generic_arguments.clone().any(|argument| {
                                    lifetime_matches_argument(&predicate.lifetime, argument)
                                })
                        }
                        WherePredicate::Type(predicate) => {
                            generic_params
                                .clone()
                                .any(|param| type_matches_param(&predicate.bounded_ty, param))
                                || generic_arguments.clone().any(|argument| {
                                    type_matches_argument(&predicate.bounded_ty, argument)
                                })
                        }
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

fn lifetime_matches_param(lifetime: &Lifetime, param: &GenericParam) -> bool {
    matches!(param, GenericParam::Lifetime(param) if param.lifetime.ident == lifetime.ident)
}

fn type_matches_argument(r#type: &Type, argument: &GenericArgument) -> bool {
    match argument {
        GenericArgument::Lifetime(_) => false,
        GenericArgument::Type(argument_type) => type_equals_type(r#type, argument_type),
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

fn type_matches_param(r#type: &Type, param: &GenericParam) -> bool {
    match param {
        GenericParam::Lifetime(_) => false,
        GenericParam::Type(param_type) => type_matches_ident(r#type, &param_type.ident),
        GenericParam::Const(_expr) => false,
    }
}

fn type_param_matches_argument(param: &TypeParam, argument: &GenericArgument) -> bool {
    match argument {
        GenericArgument::Lifetime(_) => false,
        GenericArgument::Type(r#type) => type_matches_ident(r#type, &param.ident),
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
        _ => false,
    }
}

fn type_matches_ident(r#type: &Type, ident: &Ident) -> bool {
    match r#type {
        Type::Array(_) => false,
        Type::BareFn(_) => false,
        Type::Group(r#type) => type_matches_ident(&r#type.elem, ident),
        Type::ImplTrait(_) => false,
        Type::Infer(_) => false,
        Type::Macro(_) => false,
        Type::Never(_) => false,
        Type::Paren(r#type) => type_matches_ident(&r#type.elem, ident),
        Type::Path(r#type) => r#type.path.is_ident(ident),
        Type::Ptr(r#type) => type_matches_ident(&r#type.elem, ident),
        Type::Reference(r#type) => type_matches_ident(&r#type.elem, ident),
        Type::Slice(_) => false,
        Type::TraitObject(_) => false,
        Type::Tuple(_) => false,
        Type::Verbatim(_) => false,
        _ => false,
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
        (PathArguments::AngleBracketed(_a), PathArguments::AngleBracketed(_b)) => {
            todo!("path argument equals path arguments angle bracketed")
        }
        (PathArguments::Parenthesized(_a), PathArguments::Parenthesized(_b)) => {
            todo!("path argument equals path arguments parenthesized")
        }
        _ => false,
    }
}

fn generic_param_equals_generic_param(a: &GenericParam, b: &GenericParam) -> bool {
    match (a, b) {
        (GenericParam::Lifetime(a), GenericParam::Lifetime(b)) => {
            a.lifetime.ident == b.lifetime.ident
        }
        (GenericParam::Type(a), GenericParam::Type(b)) => a.ident == b.ident,
        (GenericParam::Const(a), GenericParam::Const(b)) => a.ident == b.ident,
        _ => false,
    }
}
