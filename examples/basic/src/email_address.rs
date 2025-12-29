use fortifier::Validate;

#[derive(Validate)]
#[validate(custom(function = validate_custom, error = CustomError))]
pub enum ChangeEmailAddressRelation {
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

#[derive(Debug, PartialEq)]
pub struct CustomError;

fn validate_custom(_value: &ChangeEmailAddressRelation) -> Result<(), CustomError> {
    Ok(())
}
