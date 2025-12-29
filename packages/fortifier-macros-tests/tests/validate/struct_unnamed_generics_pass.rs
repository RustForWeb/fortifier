use fortifier::{Validate, ValidateLength, ValidationErrors};
use serde::{Deserialize, Serialize};

#[derive(Validate)]
#[validate(custom(function = validate_custom, error = CustomError))]
struct CreateUser<N: ValidateLength<usize>>(#[validate(length(min = 1, max = 256))] N);

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct CustomError;

fn validate_custom<N>(_value: &CreateUser<N>) -> Result<(), CustomError>
where
    N: ValidateLength<usize>,
{
    Ok(())
}

fn main() -> Result<(), ValidationErrors<CreateUserValidationError>> {
    let data = CreateUser("John Doe");

    data.validate_sync()?;

    Ok(())
}
