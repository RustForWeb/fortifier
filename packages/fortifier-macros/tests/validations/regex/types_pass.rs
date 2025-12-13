use std::sync::LazyLock;

use fortifier::{RegexError, Validate, ValidationErrors};
use regex::Regex;

static REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[A-Z]{2}").expect("valid regex"));

#[derive(Validate)]
struct RegexData<'a> {
    #[validate(regex = &REGEX)]
    r#str: &'a str,
    #[validate(regex = &REGEX)]
    string: String,
}

fn main() {
    let data = RegexData {
        r#str: "A",
        string: "abc".to_owned(),
    };

    assert_eq!(
        data.validate_sync(),
        Err(ValidationErrors::from_iter([
            RegexDataValidationError::Str(RegexError::default()),
            RegexDataValidationError::String(RegexError::default())
        ]))
    );
}
