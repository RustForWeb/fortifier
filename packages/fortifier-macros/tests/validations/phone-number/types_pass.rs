use std::str::FromStr;

use fortifier::{PhoneNumberCountry, PhoneNumberError, Validate, ValidationErrors};
use phonenumber::{ParseError, PhoneNumber};

#[derive(Validate)]
struct PhoneNumberData<'a> {
    #[validate(phone_number)]
    r#str: &'a str,
    #[validate(phone_number)]
    string: String,
    #[validate(phone_number(allowed_countries = vec![PhoneNumberCountry::NL]))]
    phone_number: PhoneNumber,
}

fn main() {
    let data = PhoneNumberData {
        r#str: "abc",
        string: "+44".to_owned(),
        phone_number: PhoneNumber::from_str("+44 20 7946 0000").expect("valid phone number"),
    };

    assert_eq!(
        data.validate_sync(),
        Err(ValidationErrors::from_iter([
            PhoneNumberDataValidationError::Str(PhoneNumberError::from(
                ParseError::InvalidCountryCode
            )),
            PhoneNumberDataValidationError::String(PhoneNumberError::from(ParseError::TooShortNsn)),
            PhoneNumberDataValidationError::PhoneNumber(PhoneNumberError::DisallowedCountryCode {
                allowed: vec![PhoneNumberCountry::NL],
                value: Some(PhoneNumberCountry::GB),
            })
        ]))
    );
}
