use convert_case::{Case, Casing};
use quote::format_ident;
use syn::{GenericArgument, Ident, Path, PathArguments, Type};

pub fn upper_camel_ident(ident: &Ident) -> Ident {
    let s = ident.to_string();

    if s.starts_with("r#") {
        format_ident!("{}", (&s[2..]).to_case(Case::UpperCamel))
    } else {
        format_ident!("{}", s.to_case(Case::UpperCamel))
    }
}

pub fn path_to_string(path: &Path) -> String {
    path.segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

pub fn is_option_path(path: &Path) -> bool {
    let path_string = path_to_string(path);
    path_string == "Option" || path_string == "std::option::Option"
}

pub fn count_options(r#type: &Type) -> usize {
    if let Type::Path(r#type) = r#type
        && let Some(segment) = r#type.path.segments.last()
        && let PathArguments::AngleBracketed(arguments) = &segment.arguments
        && arguments.args.len() == 1
        && is_option_path(&r#type.path)
        && let Some(GenericArgument::Type(argument_type)) = arguments.args.first()
    {
        1 + count_options(argument_type)
    } else {
        0
    }
}
