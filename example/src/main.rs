use std::{error::Error, sync::LazyLock};

use fortifier::Validate;
use regex::Regex;

static COUNTRY_CODE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[A-Z]{2}").expect("Regex should be valid."));

#[derive(Validate)]
struct CreateUser {
    #[validate(email)]
    email: String,

    #[validate(length(min = 1, max = 256))]
    name: String,

    #[validate(url)]
    url: String,

    #[validate(regex = &COUNTRY_CODE_REGEX)]
    country_code: String,

    #[validate(custom(function = validate_one_locale_required, error = OneLocaleRequiredError))]
    #[validate(length(min = 1))]
    locales: Vec<String>,
}

#[derive(Debug)]
struct OneLocaleRequiredError;

fn validate_one_locale_required(locales: &[String]) -> Result<(), OneLocaleRequiredError> {
    if locales.is_empty() {
        Err(OneLocaleRequiredError)
    } else {
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let data = CreateUser {
        email: "john@doe.com".to_owned(),
        name: "John Doe".to_owned(),
        url: "https://john.doe.com".to_owned(),
        country_code: "GB".to_owned(),
        locales: vec!["en_GB".to_owned()],
    };

    data.validate().await?;

    Ok(())
}
