use fortifier::{Validate, ValidationErrors};
use serde::{Deserialize, Serialize};

#[derive(Validate)]
#[validate(custom(function = validate_custom, error = CustomError))]
enum ChangeEmailAddressRelation {
    Create {
        #[validate(email_address)]
        email_address: String,
    },
    Update {
        id: String,

        #[validate(email_address)]
        email_address: String,
    },
    Delete {
        id: String,
    },
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct CustomError;

fn validate_custom(_value: &ChangeEmailAddressRelation) -> Result<(), CustomError> {
    Ok(())
}

fn main() -> Result<(), ValidationErrors<ChangeEmailAddressRelationValidationError>> {
    let data = ChangeEmailAddressRelation::Create {
        email_address: "john@doe.com".to_owned(),
    };

    data.validate_sync()?;

    let data = ChangeEmailAddressRelation::Update {
        id: "1".to_owned(),
        email_address: "john@doe.com".to_owned(),
    };

    data.validate_sync()?;

    let data = ChangeEmailAddressRelation::Delete { id: "1".to_owned() };

    data.validate_sync()?;

    Ok(())
}
