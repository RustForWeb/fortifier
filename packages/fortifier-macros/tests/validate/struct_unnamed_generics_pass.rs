use fortifier::{Validate, ValidateLength, ValidationErrors};

#[derive(Validate)]
struct CreateUser<N: ValidateLength<usize>>(#[validate(length(min = 1, max = 256))] N);

fn main() -> Result<(), ValidationErrors<CreateUserValidationError>> {
    let data = CreateUser("John Doe");

    data.validate_sync()?;

    Ok(())
}
