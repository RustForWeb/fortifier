use std::error::Error;

use fortifier::Validate;

#[derive(Validate)]
enum ChangeEmailAddressRelation {
    Create {
        #[validate(email)]
        email_address: String,
    },
    Update {
        id: String,

        #[validate(email)]
        email_address: String,
    },
    Delete {
        id: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
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
