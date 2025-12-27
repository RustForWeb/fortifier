use fortifier::{PhoneNumberCountry, Validate};

#[derive(Validate)]
struct PhoneNumberData<'a> {
    #[validate(phone_number(allowed_countries = PhoneNumberCountry::NL))]
    value: &'a str,
}

fn main() {}
