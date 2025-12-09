use std::error::Error;

use fortifier::Validate;

#[derive(Validate)]
enum ChangeEmailAddressRelation {
    Create,
    Update,
    Delete,
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = ChangeEmailAddressRelation::Create;

    data.validate_sync()?;

    let data = ChangeEmailAddressRelation::Update;

    data.validate_sync()?;

    let data = ChangeEmailAddressRelation::Delete;

    data.validate_sync()?;

    Ok(())
}
