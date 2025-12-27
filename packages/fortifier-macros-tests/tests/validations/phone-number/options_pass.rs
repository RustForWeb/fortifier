use fortifier::{
    PhoneNumberCountry, PhoneNumberError, PhoneNumberErrorCode, Validate, ValidationErrors,
};
use phonenumber::ParseError;

#[derive(Validate)]
struct PhoneNumberData<'a> {
    #[validate(phone_number)]
    international: &'a str,
    #[validate(phone_number(default_country = PhoneNumberCountry::GB))]
    default_country: &'a str,
    #[validate(phone_number(allowed_countries = vec![PhoneNumberCountry::GB]))]
    allowed_countries: &'a str,
}

fn main() {
    let data = PhoneNumberData {
        international: "+31 6 123456789123456789",
        default_country: "1",
        allowed_countries: "+31 6 12345678",
    };

    assert_eq!(
        data.validate_sync(),
        Err(ValidationErrors::from_iter([
            PhoneNumberDataValidationError::International(PhoneNumberError::from(
                ParseError::TooLong
            )),
            PhoneNumberDataValidationError::DefaultCountry(PhoneNumberError::from(
                ParseError::TooShortNsn
            )),
            PhoneNumberDataValidationError::AllowedCountries(
                PhoneNumberError::DisallowedCountryCode {
                    code: PhoneNumberErrorCode,
                    allowed: vec![PhoneNumberCountry::GB],
                    value: Some(PhoneNumberCountry::NL)
                }
            ),
        ]))
    );
}
