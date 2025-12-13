use fortifier::{Validate, ValidationErrors};

#[derive(Validate)]
enum ChangeEmailAddressRelation {
    Create(#[validate(email)] String),
    Update(String, #[validate(email)] String),
    Delete(String),
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
