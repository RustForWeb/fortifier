use fortifier::Validate;

#[derive(Validate)]
struct EmailAddressData<'a> {
    #[validate(email_address(allow_display_text = 1))]
    value: &'a str,
}

fn main() {}
