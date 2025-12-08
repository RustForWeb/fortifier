use std::error::Error;

use fortifier::Validate;

#[derive(Validate)]
struct CreateUser<'a>(#[validate(length(min = 1, max = 256))] &'a str);

fn main() -> Result<(), Box<dyn Error>> {
    let data = CreateUser("John Doe");

    data.validate_sync()?;

    Ok(())
}
