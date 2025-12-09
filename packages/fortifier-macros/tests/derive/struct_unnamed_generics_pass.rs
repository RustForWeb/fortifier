use std::error::Error;

use fortifier::{Validate, ValidateLength};

#[derive(Validate)]
struct CreateUser<N: ValidateLength<usize>>(#[validate(length(min = 1, max = 256))] N);

fn main() -> Result<(), Box<dyn Error>> {
    let data = CreateUser("John Doe");

    data.validate_sync()?;

    Ok(())
}
