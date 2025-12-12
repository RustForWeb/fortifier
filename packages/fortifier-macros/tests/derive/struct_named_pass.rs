use fortifier::{Validate, ValidationErrors};

#[derive(Validate)]
struct CreateUser {
    #[validate(email)]
    email: String,

    #[validate(length(min = 1, max = 256))]
    name: String,
}

fn main() -> Result<(), ValidationErrors<CreateUserValidationError>> {
    let data = CreateUser {
        email: "john@doe.com".to_owned(),
        name: "John Doe".to_owned(),
    };

    data.validate_sync()?;

    Ok(())
}
