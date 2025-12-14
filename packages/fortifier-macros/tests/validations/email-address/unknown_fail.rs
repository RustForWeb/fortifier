use fortifier::Validate;

#[derive(Validate)]
struct EmailAddressData<'a> {
    #[validate(email_address(unknown = true))]
    value: &'a str,
}

fn main() {}
