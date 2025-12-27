use fortifier::{EmailAddressError, IndexedValidationError, Validate, ValidationErrors};

#[derive(Validate)]
struct CreateUser {
    name: String,
    email_addresses: Vec<CreateEmailAddress>,
}

#[derive(Validate)]
struct CreateEmailAddress {
    #[validate(email_address)]
    email_address: String,
}

fn main() {
    let data = CreateUser {
        name: "John Doe".to_owned(),
        email_addresses: vec![CreateEmailAddress {
            email_address: "invalid".to_owned(),
        }],
    };

    assert_eq!(
        data.validate_sync(),
        Err(ValidationErrors::from_iter([
            CreateUserValidationError::EmailAddresses(ValidationErrors::from_iter([
                IndexedValidationError::new(
                    0,
                    CreateEmailAddressValidationError::EmailAddress(EmailAddressError::from(
                        email_address::Error::MissingSeparator
                    ))
                )
            ]))
        ]))
    );
}
