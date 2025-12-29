use fortifier::{Validate, ValidateWithContext, ValidationErrors};
use serde::{Deserialize, Serialize};

struct Context {
    min: usize,
    max: usize,
}

#[derive(Validate)]
#[validate(
    context = Context,
    custom(function = validate_custom, error = CustomError, context),
)]
struct CreateUser {
    #[validate(length(min = context.min, max = context.max))]
    name: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct CustomError;

fn validate_custom(_value: &CreateUser, _context: &Context) -> Result<(), CustomError> {
    Ok(())
}

fn main() -> Result<(), ValidationErrors<CreateUserValidationError>> {
    let data = CreateUser {
        name: "John Doe".to_owned(),
    };

    data.validate_sync_with_context(&Context { min: 1, max: 256 })?;

    Ok(())
}
