use std::error::Error;

use fortifier::Validate;

#[derive(Validate)]
struct CreateUser<'a, 'b> {
    #[validate(email)]
    email: &'a str,

    #[validate(length(min = 1, max = 256))]
    name: &'b str,
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = CreateUser {
        email: "john@doe.com",
        name: "John Doe",
    };

    data.validate_sync()?;

    Ok(())
}
