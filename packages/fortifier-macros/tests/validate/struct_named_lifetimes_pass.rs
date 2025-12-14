use fortifier::{Validate, ValidationErrors};

#[derive(Validate)]
struct CreateUser<'a, 'b> {
    #[validate(email_address)]
    email_address: &'a str,

    #[validate(length(min = 1, max = 256))]
    name: &'b str,
}

fn main() -> Result<(), ValidationErrors<CreateUserValidationError>> {
    let data = CreateUser {
        email_address: "john@doe.com",
        name: "John Doe",
    };

    data.validate_sync()?;

    Ok(())
}
