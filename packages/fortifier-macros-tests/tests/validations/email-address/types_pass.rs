use email_address::EmailAddress;
use fortifier::{EmailAddressError, Validate, ValidationErrors};

#[derive(Validate)]
struct EmailAddressData<'a> {
    #[validate(email_address)]
    r#str: &'a str,
    #[validate(email_address)]
    string: String,
    #[validate(email_address)]
    email_address: EmailAddress,
}

fn main() {
    let data = EmailAddressData {
        r#str: "admin",
        string: "admin@".to_owned(),
        email_address: EmailAddress::new_unchecked("Admin <admin@localhost>"),
    };

    assert_eq!(
        data.validate_sync(),
        Err(ValidationErrors::from_iter([
            EmailAddressDataValidationError::Str(EmailAddressError::from(
                email_address::Error::MissingSeparator
            )),
            EmailAddressDataValidationError::String(EmailAddressError::from(
                email_address::Error::DomainEmpty
            )),
            EmailAddressDataValidationError::EmailAddress(EmailAddressError::from(
                email_address::Error::UnsupportedDisplayName
            )),
        ]))
    );
}
