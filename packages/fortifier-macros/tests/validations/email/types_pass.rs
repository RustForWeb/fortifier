use email_address::EmailAddress;
use fortifier::{EmailError, Validate, ValidationErrors};

#[derive(Validate)]
struct EmailData<'a> {
    #[validate(email)]
    r#str: &'a str,
    #[validate(email)]
    string: String,
    #[validate(email)]
    email_address: EmailAddress,
}

fn main() {
    let data = EmailData {
        r#str: "admin",
        string: "admin@".to_owned(),
        email_address: EmailAddress::new_unchecked("Admin <admin@localhost>"),
    };

    assert_eq!(
        data.validate_sync(),
        Err(ValidationErrors::from_iter([
            EmailDataValidationError::Str(EmailError::from(email_address::Error::MissingSeparator)),
            EmailDataValidationError::String(EmailError::from(email_address::Error::DomainEmpty)),
            EmailDataValidationError::EmailAddress(EmailError::from(
                email_address::Error::UnsupportedDisplayName
            )),
        ]))
    );
}
