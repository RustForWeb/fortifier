use std::convert::Infallible;

use fortifier::{Validate, ValidationErrors};

#[derive(Validate)]
enum ChangeEmailAddressRelation {
    Create,
    Update,
    Delete,
}

fn main() -> Result<(), ValidationErrors<Infallible>> {
    let data = ChangeEmailAddressRelation::Create;

    data.validate_sync()?;

    let data = ChangeEmailAddressRelation::Update;

    data.validate_sync()?;

    let data = ChangeEmailAddressRelation::Delete;

    data.validate_sync()?;

    Ok(())
}
