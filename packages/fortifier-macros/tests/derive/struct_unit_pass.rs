use std::error::Error;

use fortifier::Validate;

#[derive(Validate)]
struct CreateUser;

fn main() -> Result<(), Box<dyn Error>> {
    let data = CreateUser;

    data.validate_sync()?;

    Ok(())
}
