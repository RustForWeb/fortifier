use std::convert::Infallible;

use fortifier::{Validate, ValidationErrors};

#[derive(Validate)]
struct CreateUser;

fn main() -> Result<(), ValidationErrors<Infallible>> {
    let data = CreateUser;

    data.validate_sync()?;

    Ok(())
}
