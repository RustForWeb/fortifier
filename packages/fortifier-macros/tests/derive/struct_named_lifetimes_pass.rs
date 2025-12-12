use fortifier::{Validate, ValidationErrors};

#[derive(Validate)]
struct CreateUser<'a, 'b> {
    #[validate(email)]
    email: &'a str,

    #[validate(length(min = 1, max = 256))]
    name: &'b str,
}

fn main() -> Result<(), ValidationErrors<CreateUserValidationError>> {
    let data = CreateUser {
        email: "john@doe.com",
        name: "John Doe",
    };

    data.validate_sync()?;

    Ok(())
}
