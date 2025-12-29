use fortifier::{Validate, ValidationErrors};
use serde::{Deserialize, Serialize};

#[derive(Validate)]
#[validate(custom(function = validate_custom, error = CustomError))]
enum ChangeEmailAddressRelation {
    Create(#[validate(email_address)] String),
    Update(String, #[validate(email_address)] String),
    Delete(String),
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct CustomError;

fn validate_custom(_value: &ChangeEmailAddressRelation) -> Result<(), CustomError> {
    Ok(())
}

fn main() -> Result<(), ValidationErrors<ChangeEmailAddressRelationValidationError>> {
    let data = ChangeEmailAddressRelation::Create("john@doe.com".to_owned());

    data.validate_sync()?;

    let data = ChangeEmailAddressRelation::Update("1".to_owned(), "john@doe.com".to_owned());

    data.validate_sync()?;

    let data = ChangeEmailAddressRelation::Delete("1".to_owned());

    data.validate_sync()?;

    Ok(())
}
