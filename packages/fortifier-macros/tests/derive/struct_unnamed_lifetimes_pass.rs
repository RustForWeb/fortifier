use fortifier::{Validate, ValidationErrors};

#[derive(Validate)]
struct CreateUser<'a>(#[validate(length(min = 1, max = 256))] &'a str);

fn main() -> Result<(), ValidationErrors<CreateUserValidationError>> {
    let data = CreateUser("John Doe");

    data.validate_sync()?;

    Ok(())
}
