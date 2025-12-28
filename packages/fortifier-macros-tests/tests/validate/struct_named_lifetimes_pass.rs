use fortifier::{Validate, ValidationErrors};
use serde::{Deserialize, Serialize};

#[derive(Validate)]
#[validate(custom(function = validate_custom, error = CustomError))]
struct CreateUser<'a, 'b> {
    #[validate(email_address)]
    email_address: &'a str,

    #[validate(length(min = 1, max = 256))]
    name: &'b str,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct CustomError;

fn validate_custom<'a, 'b>(_value: &CreateUser<'a, 'b>) -> Result<(), CustomError> {
    Ok(())
}

fn main() -> Result<(), ValidationErrors<CreateUserValidationError>> {
    let data = CreateUser {
        email_address: "john@doe.com",
        name: "John Doe",
    };

    data.validate_sync()?;

    Ok(())
}
