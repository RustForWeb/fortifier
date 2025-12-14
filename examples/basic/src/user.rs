use std::sync::LazyLock;

use fortifier::Validate;
use regex::Regex;

static COUNTRY_CODE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[A-Z]{2}").expect("valid regex"));

#[derive(Validate)]
pub struct CreateUser {
    #[validate(length(min = 1, max = 256))]
    pub name: String,

    #[validate(email_address)]
    pub email_address: String,

    #[validate(phone_number)]
    pub phone_number: String,

    #[validate(url)]
    pub url: String,

    #[validate(regex = &COUNTRY_CODE_REGEX)]
    pub country_code: String,

    #[validate(custom(function = validate_one_locale_required, error = OneLocaleRequiredError))]
    #[validate(length(min = 1))]
    pub locales: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct OneLocaleRequiredError;

fn validate_one_locale_required(locales: &[String]) -> Result<(), OneLocaleRequiredError> {
    if locales.is_empty() {
        Err(OneLocaleRequiredError)
    } else {
        Ok(())
    }
}
