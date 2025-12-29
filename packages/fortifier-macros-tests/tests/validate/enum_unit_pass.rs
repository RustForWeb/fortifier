use fortifier::{Validate, ValidationErrors};
use serde::{Deserialize, Serialize};

#[derive(Validate)]
#[validate(custom(function = validate_custom, error = CustomError))]
enum ChangeEmailAddressRelation {
    Create,
    Update,
    Delete,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct CustomError;

fn validate_custom(_value: &ChangeEmailAddressRelation) -> Result<(), CustomError> {
    Ok(())
}

fn main() -> Result<(), ValidationErrors<ChangeEmailAddressRelationValidationError>> {
    let data = ChangeEmailAddressRelation::Create;

    data.validate_sync()?;

    let data = ChangeEmailAddressRelation::Update;

    data.validate_sync()?;

    let data = ChangeEmailAddressRelation::Delete;

    data.validate_sync()?;

    Ok(())
}
