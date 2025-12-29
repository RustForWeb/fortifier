use fortifier::{Validate, ValidationErrors};
use serde::{Deserialize, Serialize};

#[derive(Validate)]
#[validate(custom(function = validate_custom, error = CustomError))]
struct CreateUser<'a>(#[validate(length(min = 1, max = 256))] &'a str);

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct CustomError;

fn validate_custom<'a>(_value: &CreateUser<'a>) -> Result<(), CustomError> {
    Ok(())
}

fn main() -> Result<(), ValidationErrors<CreateUserValidationError>> {
    let data = CreateUser("John Doe");

    data.validate_sync()?;

    Ok(())
}
