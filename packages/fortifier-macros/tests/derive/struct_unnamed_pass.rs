use std::error::Error;

use fortifier::Validate;

#[derive(Validate)]
struct CreateUser(#[validate(length(min = 1, max = 256))] String);

fn main() -> Result<(), Box<dyn Error>> {
    let data = CreateUser("John Doe".to_owned());

    data.validate_sync()?;

    Ok(())
}
