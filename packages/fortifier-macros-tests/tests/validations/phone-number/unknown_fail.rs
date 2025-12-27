use fortifier::Validate;

#[derive(Validate)]
struct PhoneNumberData<'a> {
    #[validate(phone_number(unknown = true))]
    value: &'a str,
}

fn main() {}
