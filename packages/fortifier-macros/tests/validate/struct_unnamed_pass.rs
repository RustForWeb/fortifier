use fortifier::{Validate, ValidationErrors};

#[derive(Validate)]
struct CreateUser(#[validate(length(min = 1, max = 256))] String);

fn main() -> Result<(), ValidationErrors<CreateUserValidationError>> {
    let data = CreateUser("John Doe".to_owned());

    data.validate_sync()?;

    Ok(())
}
