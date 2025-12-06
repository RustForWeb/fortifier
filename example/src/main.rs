use std::error::Error;

use fortifier::Validate;

#[derive(Validate)]
struct CreateUser {
    #[validate(email)]
    email: String,

    #[validate(length(min = 1, max = 256))]
    name: String,

    #[validate(url)]
    url: String,

    #[validate(custom(function = validate_one_locale_required, error = OneLocaleRequiredError))]
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
        locales: vec!["en_GB".to_owned()],
    };

    data.validate().await?;

    Ok(())
}
