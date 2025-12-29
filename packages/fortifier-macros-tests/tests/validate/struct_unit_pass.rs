use fortifier::{Validate, ValidationErrors};
use serde::{Deserialize, Serialize};

#[derive(Validate)]
#[validate(custom(function = validate_custom, error = CustomError))]
struct CreateUser;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct CustomError;

fn validate_custom(_value: &CreateUser) -> Result<(), CustomError> {
    Ok(())
}

fn main() -> Result<(), ValidationErrors<CreateUserValidationError>> {
    let data = CreateUser;

    data.validate_sync()?;

    Ok(())
}
