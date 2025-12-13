use fortifier::{Validate, ValidateEmail, ValidateLength, ValidationErrors};

#[derive(Validate)]
struct CreateUser<E: ValidateEmail, N: ValidateLength<usize>> {
    #[validate(email)]
    email: E,

    #[validate(length(min = 1, max = 256))]
    name: N,
}

fn main() -> Result<(), ValidationErrors<CreateUserValidationError>> {
    let data = CreateUser {
        email: "john@doe.com",
        name: "John Doe",
    };

    data.validate_sync()?;

    Ok(())
}
