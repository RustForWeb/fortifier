use fortifier::{Validate, ValidateEmailAddress, ValidateLength, ValidationErrors};
use serde::{Deserialize, Serialize};

#[derive(Validate)]
#[validate(custom(function = validate_custom, error = CustomError))]
struct CreateUser<E: ValidateEmailAddress, N: ValidateLength<usize>> {
    #[validate(email_address)]
    email_address: E,

    #[validate(length(min = 1, max = 256))]
    name: N,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct CustomError;

fn validate_custom<E, N>(_value: &CreateUser<E, N>) -> Result<(), CustomError>
where
    E: ValidateEmailAddress,
    N: ValidateLength<usize>,
{
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
