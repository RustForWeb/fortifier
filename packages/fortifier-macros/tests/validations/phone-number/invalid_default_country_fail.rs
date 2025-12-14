use fortifier::Validate;

#[derive(Validate)]
struct PhoneNumberData<'a> {
    #[validate(phone_number(default_country = ZZ))]
    value: &'a str,
}

fn main() {}
