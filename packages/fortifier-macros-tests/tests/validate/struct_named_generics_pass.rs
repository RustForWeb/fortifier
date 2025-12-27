use fortifier::{Validate, ValidateEmailAddress, ValidateLength, ValidationErrors};

#[derive(Validate)]
struct CreateUser<E: ValidateEmailAddress, N: ValidateLength<usize>> {
    #[validate(email_address)]
    email_address: E,

    #[validate(length(min = 1, max = 256))]
    name: N,
}

fn main() -> Result<(), ValidationErrors<CreateUserValidationError>> {
    let data = CreateUser {
        email_address: "john@doe.com",
        name: "John Doe",
    };

    data.validate_sync()?;

    Ok(())
}
