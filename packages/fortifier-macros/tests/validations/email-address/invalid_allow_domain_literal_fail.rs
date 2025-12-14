use fortifier::Validate;

#[derive(Validate)]
struct EmailAddressData<'a> {
    #[validate(email_address(allow_domain_literal = 1))]
    value: &'a str,
}

fn main() {}
