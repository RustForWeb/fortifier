use fortifier::{Validate, ValidationErrors};
use serde::{Deserialize, Serialize};

#[derive(Validate)]
#[validate(custom(function = validate_custom, error = CustomError))]
struct CreateUser {
    #[validate(email_address)]
    email_address: String,

    #[validate(length(min = 1, max = 256))]
    name: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct CustomError;

fn validate_custom(_value: &CreateUser) -> Result<(), CustomError> {
    Ok(())
}

fn main() -> Result<(), ValidationErrors<CreateUserValidationError>> {
    let data = CreateUser {
        email_address: "john@doe.com".to_owned(),
        name: "John Doe".to_owned(),
    };

    data.validate_sync()?;

    Ok(())
}
